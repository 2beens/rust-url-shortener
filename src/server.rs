use crate::router::Router;
use crate::ThreadPool;
use std::net::{TcpListener};
use std::sync::Arc;

pub struct Server {
    address: String,
    router: Arc<Router>,
    max_concurrent_requests: usize,
}

impl Server {
    pub fn new(address: String, max_concurrent_requests: usize) -> Server {
        Server {
            router: Arc::new(Router::new(false, true).with_logs()),
            address,
            max_concurrent_requests,
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("listening for connections ...");

        // control requests via Thread Pool
        let pool = ThreadPool::new(self.max_concurrent_requests);

        for stream in listener.incoming() {
            // TODO: check if this is right way to use the router in a closure below
            let router_clone = self.router.clone();
            match stream {
                Ok(stream) => {
                    pool.execute(move || {
                        router_clone.route(stream);
                    });
                }
                Err(e) => {
                    println!("unable to connect: {}", e);
                }
            }
        }
    }

    pub fn shutdown(&self) {
        // should call drop(pool); here
        println!("server shutdown not yet implemented :(")
    }
}
