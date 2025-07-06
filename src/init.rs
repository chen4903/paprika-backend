use crate::api::{
    api_compare_by_cfg, api_disassemble_bytecode, api_get_calldata, api_get_json_abi,
    api_get_signature_by_selector, api_get_ui_abi, api_guess_magic_result, api_simulate_call, Cli,
};
use crate::cleaner::DatabaseCleaner;
use crate::constants::CLEAN_EXPIRED_DATA;
use crate::database::Database;
use crate::logger;
use crate::utils::check_required_configs;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::sync::Arc;
use tokio::signal;
use tracing::info;

pub async fn start(cli: Cli) -> std::io::Result<()> {
    dotenv().ok();

    logger::init();
    info!("Starting Paprika Backend...");

    // Check if all required configuration parameters are set
    if let Err(e) = check_required_configs() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Configuration error: {}", e),
        ));
    }

    // Start the database cleaner
    if CLEAN_EXPIRED_DATA {
        let db = Arc::new(Database::new(false).expect("Failed to initialize database"));
        let cleaner = DatabaseCleaner::new(Arc::clone(&db));
        cleaner
            .start_cleaning_task()
            .await
            .expect("Failed to start database cleaner");
    }

    // Start the server
    let server = HttpServer::new(|| {
        App::new()
            .service(api_disassemble_bytecode)
            .service(api_compare_by_cfg)
            .service(api_get_json_abi)
            .service(api_get_ui_abi)
            .service(api_get_calldata)
            .service(api_get_signature_by_selector)
            .service(api_simulate_call)
            .service(api_guess_magic_result)
    })
    .bind(format!("127.0.0.1:{}", cli.port))?
    .run();

    info!("Server started at http://127.0.0.1:{}", cli.port);

    // Create a channel to handle graceful shutdown
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();

    // Wait for Ctrl+C signal
    tokio::spawn(async move {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C signal handler");
        info!("Received Ctrl+C, shutting down gracefully...");
        let _ = shutdown_tx.send(());
    });

    // Wait for the server to finish or shutdown signal
    tokio::select! {
        _ = server => {
            info!("Server stopped.");
        }
        _ = shutdown_rx => {
            info!("Shutdown signal received.");
        }
    }

    Ok(())
}
