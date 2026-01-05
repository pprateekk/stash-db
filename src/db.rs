use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, PartialEq)]
pub enum StashError {
	EmptyKey, 
	LockPoisoned,
}

pub type StashResult<T> = Result<T, StashError>;

pub struct Stash {
	store: Arc<RwLock<HashMap<String, String>>>,
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

	pub fn get(&self, key: &str) -> StashResult<Option<String>> {
		if key.is_empty() {
			return Err(StashError::EmptyKey);
		}

		let store = self.store.read().map_err(|_| StashError::LockPoisoned)?;

		Ok(store.get(key).cloned())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn set_inserts_new_key_value() {
		let db = Stash::new();

		db.set("a".to_string(), "1".to_string()).unwrap();

		let val_one = db.get("a").unwrap();
		assert_eq!(val_one, Some("1".to_string()));
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

		let val_one = db.get("a").unwrap();
		assert_eq!(val_one, Some("2".to_string()));
	}

	#[test]
	fn get_non_existent_key_returns_none() {
		let db = Stash::new();

		let val = db.get("non_existent").unwrap();
		assert_eq!(val, None);
	}

	#[test]
	fn get_empty_key_returns_error() {
		let db = Stash::new();

		let result = db.get("");
		assert!(matches!(result, Err(StashError::EmptyKey)));
	}
}
