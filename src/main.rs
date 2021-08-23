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
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error!("Please specify [tcp/udp] [server/client] [addr:port].");
        std::process::exit(1);
    }
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address = &args[3];
    match protocol {
        "tcp" => match role {
            "server" => {
                //todo: tcpサーバの呼び出し
            }
            "client" => {
                //todo: call tcp client
            }
            _ => {
                missing_role();
            }
        },
        "udp" => match role {
            "server" => {
                //todo: tcpサーバの呼び出し
            }
            "client" => {
                //todo: call tcp client
            }
            _ => {
                missing_role();
            }
        },
        _ => {
            error!("Please specify tcp or usp on the 1st argumet.");
            std::process::exit(1);
        }
    }
}

/**
 * 第二引数が不正だった場合に出す関数
 */
fn missing_role() {
    error!("Please specify server or client on the 2st argumet.");
    std::process::exit(1);
}
