use std::collections::HashMap;

pub struct List {
	list: HashMap<String, String>,
}

impl List {
	pub fn new() -> Self {
		Self {
			list: HashMap::new(),
		}
	}

	pub fn add(&mut self, user: &str, pass: &str) -> bool {
		if Self::check_key(self, user) {
			false
		} else {
			self.list.insert(String::from(user), String::from(pass));
			true
		}
	}

/*
	pub fn remove(&mut self, user: &str) -> bool {
		if let Some(_) = self.list.remove(user) {
			true
		} else {
			false
		}
	}
*/

	pub fn check(&self, user: &str, pass: &str) -> bool {
		if let Some(v) = Self::get_value(self, user) {
			if v == pass {
				return true;
			}
		}
		
		false
	}

	pub fn check_key(&self, user: &str) -> bool {
		self.list.contains_key(&String::from(user))
	}

	pub fn get_value(&self, user: &str) -> Option<&String> {
		if let Some((_, value)) = self.list.get_key_value(&String::from(user)) {
			return Some(&value);
		}
		
		None
	}

	pub fn get(&self) -> Option<&HashMap<String, String>> {
		if !self.list.is_empty() {
			Some(&self.list)
		} else {
			None
		}
	}
}