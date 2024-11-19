use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(long, default_value_t = String::from("config.toml"))]
    pub config_path: String,
}