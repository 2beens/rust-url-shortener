use crate::router::Router;
use crate::ThreadPool;
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

        // control requests via Thread Pool
        let pool = ThreadPool::new(5);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    pool.execute(|| {
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
