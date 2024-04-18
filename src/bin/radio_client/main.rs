use clap::{Parser, Subcommand};
use radioklw::utils::RadioResult;

use crate::rclient::Rclient;
mod chooser;
mod rclient;
mod server_initializer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Country of the radio station to search
    #[clap(short, long)]
    country: Option<String>,

    /// Language of the radio station to search
    #[clap(short, long)]
    language: Option<String>,

    /// Stream connection address
    #[clap(short, long, default_value_t = String::from("localhost:8080"))]
    addr: String,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Search radio station
    Search {
        /// Name of the radio station to search
        name: Option<String>,
    },
    /// Get radio station from saved favorites
    Favs,
    /// Stop the radio station stream
    Stop,
    /// Stop the radio stream server
    StopServer,
}

#[tokio::main]
async fn main() -> RadioResult<()> {
    let args = Args::parse();

    let mut rclient = Rclient::new(&args.addr).await?;
    rclient.handle_args(args).await?;

    Ok(())
}
