extern crate glfw;

use std::sync::mpsc::Receiver;

use glfw::{Action, Key};

use crate::key::{KeyMap};

pub mod gfx;
pub mod key;
pub mod resources;

pub static GAME_TITLE: &str = "TerraVox";
pub static NAMESPACE: &str = "terravox";

pub struct GameContext<'a> {
	pub keymap: &'a mut KeyMap,
	pub glfw: &'a mut glfw::Glfw,
	pub window: &'a mut glfw::Window,
	pub events: &'a Receiver<(f64, glfw::WindowEvent)>,
}

fn main() {
	// create a new keymap
	let ref mut keymap = KeyMap::new();
	// init glfw
	let ref mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	// define window and event pool
	let (ref mut window, ref events) = glfw.create_window(1280, 720, GAME_TITLE, glfw::WindowMode::Windowed)
		.expect("Failed to create window");

	// create context
	let ref mut context = GameContext { keymap, glfw, window, events };

	// set keymap callbacks
	context.keymap.map((Key::Escape, Action::Press), exit);

	// call init_gfx & pass context to it
	// this call loops until the window is supposed to close
	gfx::init_gfx(context);

	// after window closes, get ready to exit (unload everything)
	unload(context);
}

fn unload(_context: &mut GameContext) {
	// todo: finish game unload
}

// key callbacks
fn exit(_key: Key, _action: Action, context: &mut GameContext) {
	context.window.set_should_close(true);
}
