mod database;
mod tests;
mod server;
mod cli;
pub use database::database::*;
pub use server::server::start_listening;
pub use cli::interface::handle_server_input;

use std::thread;
use std::sync::mpsc;
use rusqlite::Result;

fn main() -> Result<()> {
	let db = Database::build()?;

	let (tx, rx) = mpsc::channel::<String>();
	let mut branches = Vec::new();

	branches.push(thread::spawn(move || {
		start_listening(rx, db);
	}));

	branches.push(thread::spawn(|| {
		handle_server_input(tx).unwrap();
	}));

	for b in branches {
		b.join().unwrap();
	}
	
	Ok(())
}