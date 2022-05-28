use rust_url_shortener::server::Server;
use std::{
    env, process,
    sync::{Arc, Mutex},
};

fn main() {
    println!("starting url shortener ...");

    let redis_conn_string;
    match env::var("SERJ_REDIS_PASS") {
        Ok(val) => redis_conn_string = format!("redis://default:{}@127.0.0.1/", val),
        Err(_e) => redis_conn_string = "redis://127.0.0.1/".to_string(),
    }
    println!(">> using redis conn string: {}", redis_conn_string);

    let (host, port) = get_host_and_port();

    let address = format!("{}:{}", host, port);
    println!("will be listening on: {}", address);

    let server = Arc::new(Mutex::new(
        Server::new(redis_conn_string, address, 5).unwrap(),
    ));
    // let server_clone = server.clone();

    ctrlc::set_handler(move || {
        println!("shutdown initiated ... TODO: not yet fully implemented");
        // server_clone.lock().unwrap().shutdown();
        process::exit(0);
    })
    .expect("error setting ctrl-c handler");

    server.lock().unwrap().start();
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
