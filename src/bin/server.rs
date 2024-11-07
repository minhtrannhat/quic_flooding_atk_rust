use quinn::rustls::pki_types::pem::PemObject;
use quinn::ServerConfig;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use core::net::IpAddr;
use std::net::Ipv4Addr;

use clap::Parser;
use quinn::rustls::pki_types::PrivatePkcs8KeyDer;
use quinn::{rustls::pki_types::CertificateDer, Endpoint};

#[derive(Parser, Debug)]
struct Args {
    server_port: u16,
}

pub fn make_server_endpoint(
    bind_addr: SocketAddr,
) -> Result<Endpoint, Box<dyn Error + Send + Sync + 'static>> {
    let server_config = configure_server()?;

    let endpoint = Endpoint::server(server_config, bind_addr)?;

    Ok(endpoint)
}

fn configure_server() -> Result<ServerConfig, Box<dyn Error + Send + Sync + 'static>> {
    let cer_der = CertificateDer::from_pem_file("./localhost-fullchain.pem")?;
    let priv_key = PrivatePkcs8KeyDer::from_pem_file("./localhost-key.pem")?;

    let mut server_config = ServerConfig::with_single_cert(vec![cer_der.clone()], priv_key.into())?;

    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();

    transport_config.max_concurrent_uni_streams(0_u8.into());

    Ok(server_config)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let args = Args::try_parse()?;

    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), args.server_port);

    let server_endpoint = make_server_endpoint(server_addr)?;

    while let Some(conn) = server_endpoint.accept().await {
        println!("connected to {}", conn.remote_address());

        let connection = conn.await?;

        println!("error: {}", connection.closed().await);

        // implicit drop the client connection here
    }

    Ok(())
}
