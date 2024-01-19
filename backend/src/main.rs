use clap::{Parser, Subcommand};
use dotenvy::dotenv;
mod api;
mod dat;
mod db;
mod import;
mod model;
use crate::api::serve;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Starts the API HTTP server
    Serve,
    /// Imports a dump from SIS from the ./data/ folder
    Import,
}

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let pool = db::get_pool().await.expect("Couldn't connect");
    sqlx::migrate!()
        .run(pool)
        .await
        .expect("Couldn't run migrations");

    let cli = Cli::parse();
    match cli.command {
        Subcommands::Import => import::import(pool).await.expect("Couldn't import :("),
        Subcommands::Serve => serve().await,
    }
}
