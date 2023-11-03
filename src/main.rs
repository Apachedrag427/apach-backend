mod cli;
mod database;
mod server;
pub use cli::interface::handle_server_input;
pub use database::database::*;
pub use server::server::*;

use rusqlite::Result;
use std::sync::{Arc, RwLock};

#[tokio::main]
async fn main() -> Result<()> {
	let db = Arc::new(RwLock::new(Database::build()?));

	Ok(())
}
