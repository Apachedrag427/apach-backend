mod database;
mod tests;
pub use database::database::{user_database, submission_database};

use rusqlite::Result;

fn main() -> Result<()> {
	let user_db = user_database()?;
	let submission_db = submission_database()?;



	Ok(())
}