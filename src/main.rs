use std::{env, io, io::Read, thread, net};

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
			let src = stream.unwrap();
			let server = server.clone();

			thread::spawn(move || {
				let dst = net::TcpStream::connect(server).unwrap();
				forward(src, dst);
			});
		}
	}
}

fn forward(src: net::TcpStream, dst: net::TcpStream) {
	let (mut src_read, mut src_write) = (src.try_clone().unwrap(), src.try_clone().unwrap());
	let (mut dst_read, mut dst_write) = (dst.try_clone().unwrap(), dst.try_clone().unwrap());

	let threads = vec![
		thread::spawn(move || match io::copy(&mut src_read, &mut dst_write) {
			_ => {
				println!("odbieram");
				src_read.set_nonblocking(true).expect("set_nonblocking call on src_read failed");
				let mut buf = vec![];
				match src_read.read_to_end(&mut buf) {
					Ok(_) => println!("odczytane"),
					Err(e) => panic!("encountered IO error: {}", e),
				};
				println!("bytes: {:?}", buf);
				return;
			}
		}),
		thread::spawn(move || match io::copy(&mut dst_read, &mut src_write) {
			_ => {
				println!("wysylam");
				src_write.set_nonblocking(true).expect("set_nonblocking call on src_write failed");
				let mut buf = vec![];
				match src_write.read_to_end(&mut buf) {
					Ok(_) => println!("odczytane"),
					Err(e) => panic!("encountered IO error: {}", e),
				};
				println!("bytes: {:?}", buf);
				return;
			}
		}),
	];

	for t in threads {
		t.join().unwrap();
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 3 {
		println!("usage: mitm-rust <client_addr> <server_addr>");
		return;
	}

	Server::new(args[1].to_string(), args[2].to_string()).run();
}
