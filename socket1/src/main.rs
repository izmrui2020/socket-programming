use log::*;
use std::env;

mod tcp_client;
mod tcp_server;
mod udp_client;
mod udp_server;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        error!("Please specify [tcp|udp] [server|client] [addr:port].");
        std::process::exit(1);
    }
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address: &str = &args[3];
    match protocol {
        "tcp" => match role {
            "server" => {
                tcp_server::server(address).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                //TODO: client
            }
            _ => {
                missing_role();
            }
        },
        "udp" => match role {
            "server" => {
                //TODO:
            }
            "client" => {
                //TODO:
            }
            _ => {
                missing_role();
            }
        },
        _ => {
            error!("Please specify tcp or udp on the 1stargument");
            std::process::exit(1);
        }
    }
}

fn missing_role() {
    error!("Please specrfy server or client on 2nd argument");
    std::process::exit(1);
}
