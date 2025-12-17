use tokio::net;
//use tokio::io::{AsyncBufReadExt, BufReader};
use clap::Parser;
use connection::Connection;
use radioklw::utils::RadioResult;
mod connection;
mod player;
mod radio_api;

extern crate lazy_static;

//radio_server -a <ADDR>
//OR
//radio_server  (uses default: localhost:8080)

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Server address
    #[arg(short, long, default_value_t = String::from("localhost:8080"))]
    addr: String,
}

#[tokio::main]
async fn main() -> RadioResult<()> {
    let args = Args::parse();

    let mut conn_handler = Connection::new().await?;
    let listener = net::TcpListener::bind(args.addr).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        log_error(conn_handler.handle(socket).await);
    }
}

fn log_error(result: RadioResult<()>) {
    if let Err(error) = result {
        eprintln!("Error: {}", error);
    }
}
