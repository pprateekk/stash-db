use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, PartialEq)]
pub enum StashError {
	EmptyKey, 
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

	pub fn set(&self, key: String, value: String) -> StashResult<()> {
		if key.is_empty() {
			return Err(StashError::EmptyKey);
		}

		let mut store = self.store.write().map_err(|_| StashError::LockPoisoned)?;

		store.insert(key, value);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn set_inserts_new_key_value() {
		let db = Stash::new();

		db.set("a".to_string(), "1".to_string()).unwrap();

		let store = db.store.read().unwrap();
		assert_eq!(store.get("a"), Some(&"1".to_string()));
	}

	#[test]
	fn set_empty_key_returns_error() {
		let db = Stash::new();

		let result = db.set("".to_string(), "1".to_string());

		assert!(matches!(result, Err(StashError::EmptyKey)));
	}

	#[test]
	fn set_overwrites_existing_key() {
		let db = Stash::new();

		db.set("a".to_string(), "1".to_string()).unwrap();
		db.set("a".to_string(), "2".to_string()).unwrap();

		let store = db.store.read().unwrap();
		assert_eq!(store.get("a"), Some(&"2".to_string()));
	}
}
