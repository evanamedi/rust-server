mod config;
mod db;
mod middlewares;

use middlewares::execute_middlewares;
use middlewares::initialize_middlewares;
use middlewares::Middleware;

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process;
use std::thread;

fn main() {
    let config = load_configuration();

    let pool = match db::init_db() {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to initialize the database: {}", e);
            std::process::exit(1);
        }
    };

    let middlewares = initialize_middlewares(pool.clone());

    if let Err(e) = init_server(&config, middlewares) {
        eprintln!("Server failed to start: {}", e);
        process::exit(1);
    }
}

fn load_configuration() -> config::Config {
    match config::load_config() {
        Ok(config) => config,
        Err(e) => {
            eprint!("Failed to load configuration: {}", e);
            process::exit(1);
        }
    }
}

fn init_server(
    config: &config::Config,
    middlewares: Vec<Box<dyn Middleware>>,
) -> std::io::Result<()> {
    let address = format!("127.0.0.1:{}", config.port);
    println!("Attempting to bind to address: {}", address);

    let listener = TcpListener::bind(&address)?;
    println!("Server is running on http://{}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let middlewares = middlewares.clone();
                thread::spawn(move || {
                    handle_connection(stream, &middlewares);
                });
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, middlewares: &Vec<Box<dyn Middleware>>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap(); // Mutable borrow here

    let (status_line, filename) = parse_request(&buffer); // Immutable borrow here

    let mut response = fs::read_to_string(filename).unwrap();

    apply_middlewares_and_respond(
        &middlewares,
        buffer,
        &mut response,
        &mut stream,
        &status_line,
    );
}

fn apply_middlewares_and_respond(
    middlewares: &Vec<Box<dyn Middleware>>,
    mut buffer: [u8; 1024],
    response: &mut String,
    stream: &mut TcpStream,
    status_line: &str,
) {
    for middleware in middlewares {
        middleware.handle(&mut buffer, response);
    }

    execute_middlewares(middlewares, &mut buffer, response);
    respond_to_client(stream, status_line, response);
}

fn parse_request(buffer: &[u8]) -> (&str, String) {
    let get = b"GET /";
    if buffer.starts_with(get) {
        let path = &buffer[get.len()..];
        let path = String::from_utf8_lossy(&path[..]);
        let end_of_path = path.find(' ').unwrap_or(path.len());
        let filepath = &path[..end_of_path];

        let filename = if filepath == "" || filepath == "/" {
            "public/index.html".to_string()
        } else {
            format!("public{}", filepath)
        };

        if Path::new(&filename).exists() {
            ("HTTP/1.1 200 OK\r\n\r\n", filename)
        } else {
            (
                "HTTP/1.1 404 NOT FOUND\r\n\r\n",
                "public/404.html".to_string(),
            )
        }
    } else {
        (
            "HTTP/1.1 400 BAD REQUEST\r\n\r\n",
            "public/400.html".to_string(),
        )
    }
}

fn respond_to_client(stream: &mut TcpStream, status_line: &str, response: &str) {
    stream.write(status_line.as_bytes()).unwrap();
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
