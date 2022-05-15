use std::net::{TcpListener};
use std::thread;

use rust_url_shortener::router::Router;

// TODO:
// create router, that would parse paths and route to related handler
// create handlers for each route
// create UrlInfo struct (short url, original url, id, etc.)
// add Rest client
// graceful shutdown

fn main() {
    println!("starting url shortener ...");

    // TODO: configurable via app args
    let host = "127.0.0.1";
    let port: u16 = 8080;
    let address = format!("{}:{}", host, port);
    println!("will be listening on: {}", address);

    let listener = TcpListener::bind(address).unwrap();
    println!("listening for connections ...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    // TODO: extract the router outside of the loop
                    let router = Router::new(true);
                    router.route(stream);
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
