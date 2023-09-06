pub mod server {
	use tiny_http::{Server, Response, Request};
	use crate::database::database::*;
	use std::sync::mpsc;

	fn handle_request(req: Request, _db: &Database) {
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


			"/wou/register" => {

			},


			_ => {
				let response = Response::empty(404);
				req.respond(response).unwrap();
			}
		}
	}

	pub fn start_listening(rx: mpsc::Receiver<String>, db: Database) {
		let server = Server::http("0.0.0.0:80").unwrap();

		loop {
			match server.try_recv() {
				Ok(req) => match req {
					Some(req) => {
						handle_request(req, &db);
					},
					None => ()
				},
				Err(_) => ()
			}

			match rx.try_recv() {
				Ok(req) => {
					let args = req.split_ascii_whitespace().collect::<Vec<&str>>();

					match args[0] {
						"createuser" => {
							db.add_user(args[1].to_string(), AccountType::Administrator, args[2].to_string()).unwrap();
						},
						"deleteuser" => {
							db.delete_user(args[1].parse().unwrap()).unwrap();
						},
						"listusers" => {
							for user in db.get_users().unwrap() {
								println!("{:?}", user);
							}
						},
						"userfromid" => {
							let user = db.get_user_from_id(args[1].parse().unwrap()).unwrap();

							match user {
								Some(usr) => println!("{:?}", usr),
								None => println!("No user found.")
							}
						},
						"userfromname" => {
							let user = db.get_user_from_name(args[1].to_string()).unwrap();

							match user {
								Some(usr) => println!("{:?}", usr),
								None => println!("No user found.")
							}
						},
						"testuserpass" => {
							let user = db.get_user_from_id(args[1].parse().unwrap()).unwrap();

							match user {
								Some(usr) => println!("{:?}", usr.verify_pass(args[2].to_string()).unwrap()),
								None => println!("No user found.")
							}
						}
						_ => ()
					}
				},
				Err(_) => ()
			}
		}
	}
}