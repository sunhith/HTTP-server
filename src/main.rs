use server::Server;
mod server; // mod fileName
fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}