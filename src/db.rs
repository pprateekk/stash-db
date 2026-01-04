use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub enum StashError {
	EmptyKet, 
	LockPoisoned,
}

pub type StashResult<T> = Result<T, StashError>;

pub struct Stash {
	pub store: Arc<RwLock<HashMap<String, String>>>,
}

impl Stash {
	pub fn new() -> Self {
		Stash {
			store: Arc::new(RwLock::new(HashMap::new())),
		}
	}

	pub fn set(&self, key: String, value: String) {
		if key.is_empty() {
			return Err(StashError::EmptyKet);
		}

		let mut store = self.store.write().map_err(|_| StashError::LockPoisoned)?;

		store.insert(key, value);

		Ok(())
	}
}
