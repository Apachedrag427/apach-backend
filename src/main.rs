mod database;
mod tests;
mod server;
pub use database::database::get_database;
pub use server::server::start_listening;

use rusqlite::Result;

fn main() -> Result<()> {
	let _db = get_database()?;

	start_listening();
	
	Ok(())
}