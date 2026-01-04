use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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
		let mut store = self.store.write().unwrap();
		store.insert(key, value);
	}
}
