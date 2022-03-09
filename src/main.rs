use std::{env};

struct Server {
	client: String,
	server: String,
}

trait IServer {
	fn new(client: String, server: String) -> Server;
	fn run(&self);
}

impl IServer for Server {
	fn new(client: String, server: String) -> Server {
		Server { client, server }
	}

	fn run(&self) {
		let (client, server) = (self.client.clone(), self.server.clone());
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 3 {
		println!("usage: mitm <client_addr> <server_addr>");
		return;
	}

	Server::new(args[1].to_string(), args[2].to_string()).run();
}
