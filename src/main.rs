use clap::Parser;
use rorm::Database;
use rorm::DatabaseConfiguration;
use rorm::DatabaseDriver;

use crate::cli::Cli;
use crate::config::Config;

mod cli;
mod config;

#[tokio::main]
async fn main() -> Result<(),  Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = Config::from_path(&cli.config_path)?;

    let db = Database::connect(DatabaseConfiguration::new(DatabaseDriver::Postgres {
        name: config.database.name,
        host: config.database.host,
        port: config.database.port,
        user: config.database.user,
        password: config.database.password,
    }))
    .await?;

    db.close().await;
    
    Ok(())
}