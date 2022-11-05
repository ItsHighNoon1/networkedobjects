use std::env;

mod client;
mod server;
mod object;

mod network {
    pub mod connection;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1].starts_with("-s") {
        server::main();
    } else {
        client::main();
    }
}
