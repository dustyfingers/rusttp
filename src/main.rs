use server::Server;
use http::Request;
use http::Method;

mod server;
mod http;

fn main() {
    // Server is a struct here - kind of like a class in oo languages
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}