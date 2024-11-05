use clap::Parser;
use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

#[derive(Parser, Debug)]

struct Args {
    client_port: u16,

    // by default, the client is an attacker
    #[arg(short, long, default_value_t = false)]
    victim: bool,
}

use quinn::{
    rustls::{self},
    ClientConfig, Endpoint,
};

pub fn make_client_endpoint(
    bind_addr: SocketAddr,
) -> Result<Endpoint, Box<dyn Error + Send + Sync + 'static>> {
    let client_cfg = ClientConfig::with_platform_verifier();
    let mut endpoint = Endpoint::client(bind_addr)?;
    endpoint.set_default_client_config(client_cfg);
    Ok(endpoint)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    //let args = Args::try_parse()?;
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let client_endpoint = make_client_endpoint("0.0.0.0:0".parse().unwrap())?;

    // connect to server
    let connection = client_endpoint
        .connect(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5050),
            "localhost",
        )
        .unwrap()
        .await
        .unwrap();

    println!("[client] connected: addr={}", connection.remote_address());

    // Waiting for a stream will complete with an error when the server closes the connection
    let _ = connection.accept_uni().await;

    // Make sure the server has a chance to clean up
    client_endpoint.wait_idle().await;

    Ok(())
}
