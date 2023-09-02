pub mod database {
	use rusqlite::{Connection, Result};

	#[allow(dead_code)]
	pub enum AccountType {
		Developer,
		Administrator,
		Moderator,
		Regular
	}
	#[allow(dead_code)]
	pub enum VerificationStatus {
		Approved,
		UnderInvestigation(String),
		Pending,
		Denied(String)
	}
	#[allow(dead_code)]
	pub struct Submission {
		id: u32,
		time: f64,
		media: String,
		note: String,
		status: VerificationStatus,
		submitter: u32,
	}
	#[allow(dead_code)]
	pub struct User {
		id: u32,
		name: String,
		submissions: Vec<u32>,
		account_type: AccountType,
	}

	#[allow(dead_code)]
	pub struct Database {
		handle: Connection,
	}
	const DB_PATH: &str = "./data.db";

	fn setup_database(handle: &Connection) -> Result<()> {
		handle.execute(
			"CREATE TABLE IF NOT EXISTS user (
				id INTEGER PRIMARY KEY,
				name TEXT NOT NULL UNIQUE,
				submissions BLOB,
				account_type BLOB
			)",
			()
		)?;

		handle.execute(
			"CREATE TABLE IF NOT EXISTS submission (
				id INTEGER PRIMARY KEY,
				time DECIMAL,
				media TEXT,
				note TEXT,
				status BLOB,
				submitter INTEGER
			)",
			()
		)?;

		Ok(())
	}

	pub fn get_database() -> Result<Database> {
		let handle = Connection::open(DB_PATH)?;

		setup_database(&handle)?;

		Ok(Database {
			handle
		})
	}

	#[allow(dead_code)]
	pub fn get_test_database() -> Result<Database> {
		let handle = Connection::open_in_memory()?;

		setup_database(&handle)?;

		Ok(Database {
			handle
		})
	}

	impl Database {

	}
}