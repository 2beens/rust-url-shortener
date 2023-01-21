use crate::router::Router;
use crate::thread_pool::ThreadPool;
use log::{debug, warn};
use redis::RedisError;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

pub struct Server {
    address: String,
    router: Arc<Mutex<Router>>,
    max_concurrent_requests: usize,
}

impl Server {
    pub fn new(
        redis_conn_string: String,
        address: String,
        max_concurrent_requests: usize,
        with_insecure_auth_service: bool,
    ) -> Result<Server, RedisError> {
        let router =
            Router::new(redis_conn_string, false, true, with_insecure_auth_service)?.with_logs();
        let router = Arc::new(Mutex::new(router));

        Ok(Server {
            address,
            router,
            max_concurrent_requests,
        })
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        debug!("listening for connections ...");

        // control requests via Thread Pool
        let pool = ThreadPool::new(self.max_concurrent_requests);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let router_clone = Arc::clone(&(self.router));
                    pool.execute(move || {
                        let mut r = router_clone.lock().unwrap();
                        r.route(stream);
                    });
                }
                Err(e) => {
                    warn!("unable to connect: {}", e);
                }
            }
        }
    }

    pub fn shutdown(&self) {
        // should call drop(pool); here
        debug!("server shutdown not yet implemented :(")
    }
}
