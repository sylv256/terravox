use serde::Deserialize;
use std::fs::{File, DirEntry, ReadDir, read_dir};
use std::io::{Write, Read};
use std::collections::HashMap;
use ron::de::from_bytes;

#[derive(Deserialize, Copy, Clone)]
pub struct ResourcePackConfig<'a> {
	name: &'a str,
}

trait Resource<'a> {
	fn new(path: &str) -> Self;
	fn config(&self) -> ResourcePackConfig<'a>;
	fn name(&self) -> &'a str {
		self.config().name
	}
	fn files(&mut self) -> &mut HashMap<&'a str, &'a File>;
	fn add_res(&mut self, path: &'a str, file: &'a mut File) {
		self.files().insert(path, file);
	}
	fn get_file<'b>(&mut self, path: &'b str) -> &'a File {
		self.files().get(path).unwrap()
	}
}

pub struct ResourcePack<'a> {
	pub config: ResourcePackConfig<'a>,
	files: HashMap<&'a str, &'a File>,
}

impl<'a> Resource<'a> for ResourcePack<'a> {
	fn new(path: &str) -> Self {
		let pack_file = read_dir(path).expect("Failed to read resource pack directory");
		let mut config = Err("pack.ron doesn't exist");
		for result in pack_file.into_iter() {
			let file = result.unwrap();
			if file.file_name() == "pack.ron" {
				let ref mut buf = [];
				File::open(file.path()).unwrap().read(buf);
				config = Ok(from_bytes::<ResourcePackConfig>(buf).unwrap());
			}
		}
		ResourcePack { config: config.unwrap(), files: HashMap::new() }
	}

	fn config(&self) -> ResourcePackConfig<'a> {
		self.config
	}

	fn files(&mut self) -> &mut HashMap<&'a str, &'a File> {
		&mut self.files
	}
}
