use std::env;
#[macro_use] extern crate log;

use rust_socket::tcp_server;

fn missing_role() {
    error!("Please specify tcp or udp on the second argment.");
    std::process::exit(1);
}

fn main() {
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
                tcp_server::serve(address).unwrap_or_default();
            }
            "client" => {
                // tcp client
            }
            _ => {
                missing_role();
            }
        }
        "udp" => match role {
            "server" => {
                // tcp server
            }
            "client" => {
                // tcp client
            }
            _ => {
                missing_role();
            }
        }
        _ => {
            error!("Please specify tcp or udp on the first argment.");
            std::process::exit(1);
        }
    }

}
