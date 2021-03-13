use std::collections::HashMap;

use glfw::{Action, Key};

use crate::GameContext;

pub type KeyAction = (Key, Action);
pub type KeyCallback = fn(Key, Action, &mut GameContext);

pub struct KeyMap {
	map: HashMap<KeyAction, KeyCallback>,
}

impl KeyMap {
	pub fn new() -> Self {
		Self { map: HashMap::new() }
	}
	pub fn map(&mut self, (key, action): KeyAction, cb: KeyCallback) {
		self.map.insert((key, action), cb);
	}
	pub fn get(&self, ka: KeyAction) -> Option<&KeyCallback> {
		self.map.get(&ka)
	}
}
