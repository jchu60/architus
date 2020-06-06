use log::info;

mod handler;
mod zipper;

use std::env::args;
use std::net::TcpListener;
use std::thread::{sleep, spawn};
use std::time::Duration;

use manager::manager_client::ManagerClient;

pub mod manager {
    include!("/app/src/manager.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    sleep(Duration::from_secs(10));
    simple_logger::init().unwrap();
    info!("Starting recording microservice");

    let port = args().nth(1).expect("No port given");
    let port = port.parse::<u16>().expect("Didn't pass a valid number");
    assert!(port < 65535);

    let server = TcpListener::bind(format!("record-service:{}", port)).expect("Failed to bind");
    info!("Listening on port {}", port);

    let rpc = ManagerClient::connect("http://manager:50051").await?;
    info!("Connected to manager service.");

    for stream in server.incoming() {
        match stream {
            Ok(s) => {
                let h = handler::WAVReceiver::new();
                info!(
                    "Serving: {}",
                    s.peer_addr().expect("Peer doesn't have address")
                );
                spawn(move || h.handle(s));
            }
            Err(_) => break,
        }
    }

    Ok(())
}