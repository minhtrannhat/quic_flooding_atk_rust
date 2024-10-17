use clap::Parser;
use std::{error::Error, net::SocketAddr, sync::Arc};

#[derive(Parser, Debug)]

struct Args {
    client_port: u16,

    // by default, the client is an attacker
    #[arg(short, long, default_value_t = false)]
    victim: bool,
}

use quinn::{
    rustls::{self, pki_types::CertificateDer},
    ClientConfig, Endpoint,
};

pub fn make_client_endpoint(
    bind_addr: SocketAddr,
    server_certs: &[&[u8]],
) -> Result<Endpoint, Box<dyn Error + Send + Sync + 'static>> {
    let client_cfg = configure_client(server_certs)?;
    let mut endpoint = Endpoint::client(bind_addr)?;
    endpoint.set_default_client_config(client_cfg);
    Ok(endpoint)
}

fn configure_client(
    server_certs: &[&[u8]],
) -> Result<ClientConfig, Box<dyn Error + Send + Sync + 'static>> {
    let mut certs = rustls::RootCertStore::empty();
    for cert in server_certs {
        certs.add(CertificateDer::from(*cert))?;
    }

    Ok(ClientConfig::with_root_certificates(Arc::new(certs))?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let args = Args::try_parse()?;

    let client_endpoint = make_client_endpoint("0.0.0.0:0".parse().unwrap(), &[&server_cert])?;

    // connect to server
    let connection = client_endpoint
        .connect(server_addr, "localhost")
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
