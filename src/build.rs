extern crate gl_generator;

use std::env;
use std::fs::File;
use std::path::Path;

use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};

fn main() {
	let ref dest = env::var("OUT_DIR").unwrap();
	let ref mut file = File::create(Path::new(dest).join("bindings.rs")).unwrap();

	Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, [])
		.write_bindings(StructGenerator, file)
		.unwrap();
}
