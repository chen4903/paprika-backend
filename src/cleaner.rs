use crate::{constants::CLEAN_INTERVAL, database::Database};
use eyre::Result;
use std::{sync::Arc, time::Duration};
use tokio::time;
use tracing::{error, info};

pub struct DatabaseCleaner {
    db: Arc<Database>,
}

/// To accommodate all small servers, we regularly delete data that is not frequently used.
impl DatabaseCleaner {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn start_cleaning_task(&self) -> Result<()> {
        let db = Arc::clone(&self.db);

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(CLEAN_INTERVAL));

            loop {
                interval.tick().await;
                if let Err(e) = db.cleanup_expired_records() {
                    error!("Error cleaning expired records: {}", e);
                }
                info!("🚮 Cleaned expired records");
            }
        });

        Ok(())
    }
}
