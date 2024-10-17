use core::net::IpAddr;
use std::net::Ipv4Addr;
use std::{error::Error, net::SocketAddr};

mod client;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5050);

    let (server_endpoint, server_cert) = server::make_server_endpoint(server_addr)?;

    let server_endpoint_cloned = server_endpoint.clone();

    // we only wait for one client connection
    tokio::spawn(async move {
        let incomming_conn = server_endpoint_cloned.accept().await.unwrap();

        let conn = incomming_conn.await.unwrap();

        println!(
            "[server] connection accepted: addr = {}",
            conn.remote_address()
        )

        // client connection implicitly dropped here
    });

    let endpoint = client::make_client_endpoint("0.0.0.0:0".parse().unwrap(), &[&server_cert])?;

    // connect to server
    let connection = endpoint
        .connect(server_addr, "localhost")
        .unwrap()
        .await
        .unwrap();

    println!("[client] connected: addr={}", connection.remote_address());

    // Waiting for a stream will complete with an error when the server closes the connection
    let _ = connection.accept_uni().await;

    // Make sure the server has a chance to clean up
    endpoint.wait_idle().await;

    Ok(())
}
