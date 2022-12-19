use log::{info, trace, warn, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use rust_url_shortener::server::Server;
use std::{
    env, process,
    sync::{Arc, Mutex},
};

// to run in windows, with redis running in docker, and port:
// $env:SERJ_REDIS_PASS = 'todo'; .\rust-url-shortener.exe -p 9001

fn main() {
    println!("starting url shortener ...");

    setup_logger();

    let redis_conn_string;
    match env::var("SERJ_REDIS_PASS") {
        Ok(val) => redis_conn_string = format!("redis://default:{}@127.0.0.1/", val),
        Err(_e) => redis_conn_string = "redis://127.0.0.1/".to_string(),
    }
    trace!(">> using redis conn string: {}", redis_conn_string);

    let (host, port) = get_host_and_port();

    let address = format!("{}:{}", host, port);
    info!("will be listening on: {}", address);

    let server = Arc::new(Mutex::new(
        Server::new(redis_conn_string, address, 5).unwrap(),
    ));
    // let server_clone = server.clone();

    ctrlc::set_handler(move || {
        warn!("shutdown initiated ... TODO: not yet fully implemented");
        // server_clone.lock().unwrap().shutdown();
        process::exit(0);
    })
    .expect("error setting ctrl-c handler");

    server.lock().unwrap().start();
}

fn setup_logger() {
    let log_file_path;
    match env::var("LOG_FILE_PATH") {
        Ok(val) => log_file_path = val,
        Err(_e) => log_file_path = "log/output.log".to_string(),
    }
    println!(">>> using log path: {}", log_file_path);

    let stdout = ConsoleAppender::builder().build();
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} [{l}]:\t{m}\n")))
        .build(log_file_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stdout")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    info!("logger setup completed...");
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
