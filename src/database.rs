pub mod database {
	use rusqlite::{Connection, Result};

	pub enum AccountType {
		Developer,
		Administrator,
		Moderator,
		Regular
	}
	pub enum VerificationStatus {
		Approved,
		UnderInvestigation(String),
		Pending,
		Denied(String)
	}
	pub struct Submission {
		time: f64,
		media: String,
		note: String,
		status: VerificationStatus,
	}
	pub struct User {
		id: u32,
		name: String,
		submissions: Vec<Submission>,
		account_type: AccountType,
	}

	pub struct UserDatabase {
		handle: Connection,
	}
	pub struct SubmissionDatabase {
		handle: Connection,
	}
	const USER_PATH: &str = "./users.db";
	const SUBMISSION_PATH: &str = "./submissions.db";

	pub fn user_database() -> Result<UserDatabase> {
		let handle = Connection::open(USER_PATH)?;

		Ok(UserDatabase {
			handle
		})
	}
	pub fn submission_database() -> Result<SubmissionDatabase> {
		let handle = Connection::open(SUBMISSION_PATH)?;

		Ok(SubmissionDatabase {
			handle
		})
	}

	pub fn test_user_database() -> Result<UserDatabase> {
		let handle = Connection::open_in_memory()?;

		Ok(UserDatabase {
			handle
		})
	}
	pub fn test_submission_database() -> Result<SubmissionDatabase> {
		let handle = Connection::open_in_memory()?;

		Ok(SubmissionDatabase {
			handle
		})
	}
}