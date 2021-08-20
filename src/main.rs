use std::io::{Read, Write, Error};
use std::{env, error};

#[macro_use]
extern crate log;

mod tcp_client;
mod tcp_server;
mod udp_client;
mod udp_server;
fn main() {
    env::set_var("RUST_LOG", "debug");
    envlogger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error!("Please specify [tcp/udp] [server/client] [addr:port].");
        std::process::exit(1);
    }
}
