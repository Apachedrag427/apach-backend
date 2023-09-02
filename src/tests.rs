#[cfg(test)]
mod tests {
	use crate::database::database::get_test_database;
	use rusqlite::Result;
	#[test]
	fn db_write() -> Result<()> {
		let _db = get_test_database()?;



		Ok(())
	}
}