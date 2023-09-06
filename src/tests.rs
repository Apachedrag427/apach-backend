#[cfg(test)]
mod tests {
	use crate::database::database::*;
	use rusqlite::Result;
	#[test]
	fn db_write() -> Result<()> {
		let _db = Database::build_test()?;



		Ok(())
	}
}