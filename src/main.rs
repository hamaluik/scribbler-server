extern crate scribbler_server;

fn main() {
    // run the server by default
    if let Err(e) = scribbler_server::run_server() {
        eprintln!("Error: {:?}", e);
    }
}
