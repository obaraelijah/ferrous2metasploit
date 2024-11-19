use clap::Parser;
use crate::cli::Cli;

mod cli;

#[tokio::main]
async fn main() -> Result<(),  Box<dyn std::error::Error>> {
    Cli::parse();
    Ok(())
}
