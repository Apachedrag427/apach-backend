pub mod database {
	use rusqlite::{Connection, Result, Row, OptionalExtension};
	use rand::RngCore;
	use std::{io::{BufReader, Read}, error::Error};
	use data_encoding::HEXUPPER;
	use ring::digest::{Context, Digest, SHA256};

	type DynResult<T> = std::result::Result<T, Box<dyn Error>>;

	#[allow(dead_code)]
	const ACCOUNT_TYPE: [&str; 4] = ["Regular", "Moderator", "Administrator", "GameDeveloper"];

	#[allow(dead_code)]
	#[derive(Debug)]
	pub enum AccountType {
		Regular,
		Moderator,
		Administrator,
		GameDeveloper
	}

	#[allow(dead_code)]
	const VERIFICATION_STATUS: [&str; 4] = ["Approved", "UnderInvestigation", "Denied", "Pending"];

	#[allow(dead_code)]
	#[derive(Debug)]
	pub enum VerificationStatus {
		Approved,
		UnderInvestigation,
		Denied,
		Pending
	}

	#[allow(dead_code)]
	#[derive(Debug)]
	pub struct Submission {
		id: u32,
		time: f64,
		media: String,
		note: String,
		submitter: u32,
		status: VerificationStatus,
		category: u8
	}

	#[allow(dead_code)]
	#[derive(Debug)]
	pub struct User {
		id: u32,
		name: String,
		account_type: AccountType,
		hashed_pass: String,
		hash_salt: String
	}

	impl User {
		pub fn verify_pass(&self, pass: String) -> DynResult<bool> {
			let salt_vec = HEXUPPER.decode(self.hash_salt.as_bytes())?;

			let newhash = hash_vecsalt(pass, salt_vec)?;

			Ok(newhash == self.hashed_pass)
		}
	}

	#[allow(dead_code)]
	struct Category {
		id: u32,
		name: String,
		desc: String
	}

	#[allow(dead_code)]
	pub struct Database {
		handle: Connection,
	}
	const DB_PATH: &str = "./data.db";

	fn generate_salt() -> [u8; 128] {
		let mut salt: [u8; 128] = [8; 128];

		let mut rng = rand::thread_rng();
		rng.fill_bytes(&mut salt);

		return salt;
	}

	fn sha256_digest<R: Read>(mut reader: R) -> DynResult<Digest> {
		let mut context = Context::new(&SHA256);
		let mut buf = [0; 1024];

		loop {
			let count = reader.read(&mut buf)?;
			if count <= 0 {
				break;
			}

			context.update(&buf[..count]);
		}

		Ok(context.finish())
	}

	fn hash(input: String, salt: [u8; 128]) -> DynResult<String> {
		let mut to_read = input.as_bytes().to_vec();

		to_read.append(&mut salt.to_vec());

		let reader = BufReader::new(&*to_read);
		let digest = sha256_digest(reader)?;

		Ok(HEXUPPER.encode(digest.as_ref()))
	}

	fn hash_vecsalt(input: String, mut salt: Vec<u8>) -> DynResult<String> {
		let mut to_read = input.as_bytes().to_vec();

		to_read.append(&mut salt);

		let reader = BufReader::new(&*to_read);
		let digest = sha256_digest(reader)?;

		Ok(HEXUPPER.encode(digest.as_ref()))
	}


	fn user_from_row(row: &Row<'_>) -> Result<User> {
		Ok(User {
			id: row.get(0)?,
			name: row.get(1)?,
			account_type: match row.get(2)? {
				0 => AccountType::Regular,
				1 => AccountType::Moderator,
				2 => AccountType::Administrator,
				3 => AccountType::GameDeveloper,
				_ => {
					eprintln!("SOMETHING MESSED UP!!! Found user {} with an invalid account type.  Assuming Regular...", row.get::<_, String>(1)?);
					AccountType::Regular
				}
			},
			hashed_pass: row.get(3)?,
			hash_salt: row.get(4)?,
		})
	}

	impl Database {
		pub fn delete_user(&self, id: u32) -> DynResult<()> {
			let handle = &self.handle;

			handle.execute(
				"DELETE FROM user WHERE id = ?1",
				[id]
			)?;

			Ok(())
		}

		pub fn get_user_from_name(&self, name: String) -> Result<Option<User>> {
			let handle = &self.handle;

			Ok(handle.query_row(
				"SELECT id, name, account_type, hashed_pass, hash_salt FROM user WHERE name = '?1'",
				[name],
				user_from_row
			).optional()?)
		}

		pub fn get_user_from_id(&self, id: u32) -> Result<Option<User>> {
			let handle = &self.handle;

			Ok(handle.query_row(
				"SELECT id, name, account_type, hashed_pass, hash_salt FROM user WHERE id = ?1",
				[id],
				user_from_row
			).optional()?)
		}

		pub fn get_users(&self) -> Result<Vec<User>> {
			let mut list = Vec::new();

			let handle = &self.handle;

			let mut stmt = handle.prepare("SELECT id, name, account_type, hashed_pass, hash_salt FROM user")?;
			let iter = stmt.query_map([], user_from_row)?;

			for user in iter {
				list.push(user.unwrap());
			}

			Ok(list)
		}

		pub fn add_user(&self, name: String, acc_type: AccountType, pass: String) -> std::result::Result<(), Box<dyn Error>> {
			let salt = generate_salt();

			let hashed_pass = hash(pass, salt)?;
			

			let handle = &self.handle;

			handle.execute(
				"INSERT INTO user (name, account_type, hashed_pass, hash_salt) VALUES (?1, ?2, ?3, ?4)",
				(name, match acc_type {
					AccountType::Regular => 0,
					AccountType::Moderator => 1,
					AccountType::Administrator => 2,
					AccountType::GameDeveloper => 3,
				}, hashed_pass, HEXUPPER.encode(&salt))
			)?;

			Ok(())
		}

		pub fn build() -> Result<Database> {
			let handle = Connection::open(DB_PATH)?;

			let db = Database {
				handle
			};
			db.setup()?;

			Ok(db)
		}

		pub fn build_test() -> Result<Database> {
			let handle = Connection::open_in_memory()?;

			let db = Database {
				handle
			};
			db.setup()?;

			Ok(db)
		}

		fn setup(&self) -> Result<()> {
			let handle = &self.handle;
			handle.execute(
				"CREATE TABLE IF NOT EXISTS user (
					id INTEGER PRIMARY KEY,
					name TEXT NOT NULL UNIQUE,
					account_type INTEGER,
					hashed_pass TEXT,
					hash_salt TEXT
				)",
				()
			)?;
	
			handle.execute(
				"CREATE TABLE IF NOT EXISTS submission (
					id INTEGER PRIMARY KEY,
					time DECIMAL,
					media TEXT,
					note TEXT,
					mod_note TEXT,
					status INTEGER,
					creator INTEGER,
					category INTEGER
				)",
				()
			)?;
	
			handle.execute(
				"CREATE TABLE IF NOT EXISTS category (
					id INTEGER PRIMARY KEY,
					name TEXT,
					desc TEXT
				)",
				()
			)?;
	
			Ok(())
		}
	}
}