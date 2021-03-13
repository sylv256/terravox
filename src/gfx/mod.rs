use glfw::{Context, WindowEvent};
use ron::de::from_str;
use serde::Deserialize;

use crate::{GameContext};

pub mod gl {
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// yes i know that this looks like dfu. trust me; it isn't.
pub type RGB<T> = (T, T, T);
pub type RGBA<T> = (T, T, T, T);

#[derive(Debug, Deserialize)]
struct ViewportConfig {
	viewport_color: RGB<f32>,
}

pub fn init_gfx(context: &mut GameContext) {
	// make window's context current
	context.window.make_current();
	// enable key polling
	context.window.set_key_polling(true);

	// load opengl
	let gl = gl::Gl::load_with(|s| context.window.get_proc_address(s) as *const _);

	// todo: create and organize resource system
	let config: ViewportConfig = from_str(include_str!("../../res/terravox/viewport.ron")).expect("Failed to load viewport config");

	// todo: cleanup code, and add color module for converting from GLcolor to RGB
	unsafe { gl.ClearColor(config.viewport_color.0 / 255.0, config.viewport_color.1 / 255.0, config.viewport_color.2 / 255.0, 1.0) }

	// until window should close, loop
	while !context.window.should_close() {
		// swap buffers
		context.window.swap_buffers();

		unsafe { gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }

		// poll events
		context.glfw.poll_events();
		// exhaust & flush events
		for (_, event) in glfw::flush_messages(context.events) {
			match event {
				WindowEvent::Key(key, _, action, _) => {
					let cb = context.keymap.get((key, action));
					if cb.is_some() {
						cb.unwrap()(key, action, context);
					}
				},
				_ => {},
			}
		}
	}
}
