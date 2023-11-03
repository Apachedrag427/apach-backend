pub mod interface {
	use std::io;
	use std::sync::{Arc, RwLock};

	use crate::database::database::Database;

	const HELP_MSG: &str = "Commands {
	help: Displays this message
	send: Sends a message to the main server
	user {
		new <name> <password>: Creates a new admin user with <name> and <password>
		list: Lists all users
		id <id>: Searches for a user with <id>
		name <name> Search for a user with <name>
	}
}";

	pub fn handle_server_input(db: Arc<RwLock<Database>>) -> io::Result<()> {
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
				_ => println!("Please enter a valid command."),
			}
		}
	}
}
