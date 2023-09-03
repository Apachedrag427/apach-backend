pub mod server {
	use std::{sync::Arc, thread};
	use tiny_http::{Server, Response, Request};

	fn handle_request(req: Request) {
		match req.url() {
			"/" => {
				let response = Response::from_string("This is a backend.");
				req.respond(response).unwrap();
			},
			"/discord" => {
				let mut response = Response::empty(301);
				response.add_header("Location: https://discord.com/users/609177928437071872/".parse::<tiny_http::Header>().unwrap());

				req.respond(response).unwrap();
			},
			"/guilded" => {
				let mut response = Response::empty(301);
				response.add_header("Location: https://www.guilded.gg/u/apach".parse::<tiny_http::Header>().unwrap());

				req.respond(response).unwrap();
			},
			"/guildedref" => {
				let mut response = Response::empty(301);
				response.add_header("Location: https://www.guilded.gg/?r=mR6pKEQm".parse::<tiny_http::Header>().unwrap());

				req.respond(response).unwrap();
			},
			_ => {
				let response = Response::empty(404);
				req.respond(response).unwrap();
			}
		}
	}

	pub fn start_listening() {
		let server = Arc::new(Server::http("0.0.0.0:80").unwrap());
		println!("Listening on port 80");

		let mut handles = Vec::new();

		for _ in 0..4 {
			let server = server.clone();
			
			handles.push(thread::spawn(move || {
				for request in server.incoming_requests() {
					handle_request(request);
				}
			}));
		}

		for h in handles {
			h.join().unwrap();
		}
	}
}