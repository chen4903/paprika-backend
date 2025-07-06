use paprika::api::Cli;
use structopt::StructOpt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::from_args();
    paprika::init::start(cli).await
}
