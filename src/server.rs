pub mod server {
	use crate::database::database::*;
	use std::sync::{Arc, RwLock};

	async fn serve(db: Arc<RwLock<Database>>) {}
}
