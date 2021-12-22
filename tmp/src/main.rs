use std::env;

use env_logger::Logger;

#[macro_use]
extern crate log;

mod tcp_client;
mod tcp_server;
mod udp_client;
mod udp_server;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env::logger::init();
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        error!("Pleasespecify[tcp|udp][server|client][addr:port].

        Teruya Ono. Introduction to network programming with Rust (Japanese Edition) (Kindle の位置No.202). Kindle 版. ");
        std::process::exit(1);
    }
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address: &str = &args[3];

    match protocol {
        "tcp" => match role {
            "server" => {}
            "client" => {}
            _ => {
                missing_role();
            }
        },
        "udp" => match role {
            "server" => {}
            "client" => {}
            _ => {
                missing_role();
            }
        },
        _ => {
            error!("please specify tcp or udp on the lst argument.protocol");
            std::process::exit(1);
        }
    }
}

fn missing_role() {
    error!("please specify server or client on the 2nd argument.Logger");
    std::process::exit(1);
}
