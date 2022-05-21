use rust_url_shortener::server::Server;
use std::{env, process};

// TODO:
// create router, that would parse paths and route to related handler
// create handlers for each route
// create UrlInfo struct (short url, original url, id, etc.)
// add Rest client
// graceful shutdown

fn main() {
    println!("starting url shortener ...");

    let (host, port) = get_host_and_port();

    let address = format!("{}:{}", host, port);
    println!("will be listening on: {}", address);

    let server = Server::new(address);
    server.start();
}

fn get_host_and_port() -> (String, u16) {
    let mut host = "127.0.0.1";
    let mut port: u16 = 8080;

    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
        if args[i] == "-host" || args[i] == "-h" {
            if i + 1 == args.len() {
                eprintln!("invalid arguments [host], args len: {}", args.len());
                process::exit(1);
            }
            host = args[i + 1].as_str();
        }
        if args[i] == "-port" || args[i] == "-p" {
            if i + 1 == args.len() {
                eprintln!("invalid arguments [port], args len: {}", args.len());
                process::exit(1);
            }
            match args[i + 1].parse::<u16>() {
                Ok(n) => port = n,
                Err(e) => {
                    eprintln!("invalid port argument: {e}");
                    process::exit(1);
                }
            }
        }
    }

    (String::from(host), port)
}