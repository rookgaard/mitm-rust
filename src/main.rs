use std::{env};

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 3 {
		println!("usage: mitm <client_addr> <server_addr>");
		return;
	}
}
