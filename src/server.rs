use crate::router::Router;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct Server {
    address: String,
    router: Router,
}

impl Server {
    pub fn new(address: String) -> Server {
        Server {
            router: Router::new(false, true).with_logs(),
            address,
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("listening for connections ...");

        // TODO: control requests via Thread Pool

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| {
                        // TODO: try to use the shared router
                        // self.router.route(stream);
                        Router::new(false, true).with_logs().route(stream);
                    });
                }
                Err(e) => {
                    println!("unable to connect: {}", e);
                }
            }
        }
    }
}
