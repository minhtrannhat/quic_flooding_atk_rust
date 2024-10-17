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
) -> Result<(Endpoint, CertificateDer<'static>), Box<dyn Error + Send + Sync + 'static>> {
    let (server_config, server_cert) = configure_server()?;

    let endpoint = Endpoint::server(server_config, bind_addr)?;

    Ok((endpoint, server_cert))
}

fn configure_server(
) -> Result<(ServerConfig, CertificateDer<'static>), Box<dyn Error + Send + Sync + 'static>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();

    let cert_der = CertificateDer::from(cert.cert);

    let priv_key = PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der());

    let mut server_config =
        ServerConfig::with_single_cert(vec![cert_der.clone()], priv_key.into())?;

    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();

    transport_config.max_concurrent_uni_streams(0_u8.into());

    Ok((server_config, cert_der))
}

#[allow(unreachable_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let args = Args::try_parse()?;

    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), args.server_port);

    let (server_endpoint, server_cert) = make_server_endpoint(server_addr)?;

    loop {
        let server_endpoint_cloned = server_endpoint.clone();

        // every time a connection is accepted, we spawn an async task
        tokio::spawn(async move {
            let incomming_conn = server_endpoint_cloned.accept().await.unwrap();

            let conn = incomming_conn.await.unwrap();

            println!(
                "[server] connection accepted: addr = {}",
                conn.remote_address()
            )

            // client connection implicitly dropped here
        });
    }

    Ok(())
}
