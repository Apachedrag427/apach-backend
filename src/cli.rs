pub mod interface {
	use std::io;
	use std::sync::mpsc;
	use std::thread;
	use std::time::Duration;

	const HELP_MSG: &str = 
"Commands {
	help: Displays this message
	send: Sends a message to the main server
	user {
		new <name> <password>: Creates a new admin user with <name> and <password>
		list: Lists all users
		id <id>: Searches for a user with <id>
		name <name> Search for a user with <name>
	}
}";

	pub fn handle_server_input(tx: mpsc::Sender<String>) -> io::Result<()> {
		println!("Welcome to the server console.  Type 'help' for more information.");
		loop {
			println!("");
			let mut buf = String::new();
			io::stdin().read_line(&mut buf)?;
			println!("");

			let args = buf.split_ascii_whitespace().collect::<Vec<&str>>();

			if args.len() <= 0 {
				continue;
			}

			match args[0] {
				"help" => println!("{}", HELP_MSG),
				"send" => tx.send(args[1..].concat()).unwrap(),
				"user" => {
					if args.len() < 2 {
						println!("Invalid arguments.");
						continue;
					}
					match args[1] {
						"new" => {
							if args.len() < 4 {
								println!("Invalid arguments.");
								continue;
							}

							let nm = args[2];
							let pass = args[3];

							let req = format!("createuser {} {}", nm, pass);
							tx.send(req).unwrap();
						},
						"remove" => {
							if args.len() < 3 {
								println!("Invalid arguments.");
								continue;
							}

							tx.send(format!("deleteuser {}", args[2])).unwrap();
						},
						"list" => {
							tx.send("listusers".to_string()).unwrap();
						},
						"id" => {
							if args.len() < 3 {
								println!("Invalid arguments.");
								continue;
							}

							tx.send(format!("userfromid {}", args[2])).unwrap();
						},
						"name" => {
							if args.len() < 3 {
								println!("Invalid arguments.");
								continue;
							}

							tx.send(format!("userfromname {}", args[2])).unwrap();
						},
						"testpass" => {
							if args.len() < 4 {
								println!("Invalid arguments.");
								continue;
							}

							tx.send(format!("testuserpass {} {}", args[2], args[3])).unwrap();
						},
						_ => {
							println!("Invalid arguments.");
							continue;
						}
					}
				}
				_ => println!("Please enter a valid command.")
			}

			thread::sleep(Duration::from_millis(50)); // Wait for the server to receive and handle the message
		}
	}
}
