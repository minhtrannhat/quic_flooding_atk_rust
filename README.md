# QUIC Flooding Attack Simulation

Tested on Rust 1.84

## Goals

- Learn about the basics of the QUIC protocol.

## Setup

- Install Rust
- Install `mkcert`
- Setup an intermediate CA on your machine: `mkcert -install`
- Generate SSL certs and private key for localhost: `mkcert localhost`
- Run `cargo run --bin server 5050` to run the server on port 5050
- Run `cargo run --bin client` (in a different terminal window) to run the client.

## Progress

- [x] Server implementation
- [x] Client implementation
- [ ] Attack Mechanism (scripts)
- [ ] CPU usage benchmarking
