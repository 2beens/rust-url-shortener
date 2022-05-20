use crate::Router;
use std::net::TcpListener;
use std::thread;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Server {
        Server { address }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("listening for connections ...");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| {
                        // TODO: extract the router outside of the loop
                        // let router = Router::new(true);
                        // router.route(stream);
                        Router::new(false, true).with_logs().route(stream);
                    });
                }
                Err(e) => {
                    println!("Unable to connect: {}", e);
                }
            }
        }
    }
}
