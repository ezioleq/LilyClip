use directories::ProjectDirs;

pub struct Data {
	pub path: std::path::PathBuf
}

impl Data {
	pub fn new() -> Data {
		let dir = match ProjectDirs::from("com", "Ezioleq", "LilyClip") {
			Some(dir) => dir,
			None => panic!("Couldn't get application data paths.")
		};
		let dir = dir.data_dir().to_owned();
		
		match std::fs::create_dir_all(&dir) {
			Ok(()) => (),
			Err(_) => panic!("Couldn't create program data folder in {}", dir.to_str().unwrap())
		}

		Data {
			path: dir
		}
	}
}
