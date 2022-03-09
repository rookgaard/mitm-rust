use std::{env};
use std::net;

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
		let listener = net::TcpListener::bind(client).unwrap();

		for stream in listener.incoming() {
			println!("packet detected");
			let src = stream.unwrap();
			let server = server.clone();
		}
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
