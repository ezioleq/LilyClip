use directories::ProjectDirs;
use std::fs::File;
use std::io::BufRead;

pub struct ProgramData {
    pub path: std::path::PathBuf,
}

impl ProgramData {
    pub fn new() -> ProgramData {
        let dir = match ProjectDirs::from("com", "Ezioleq", "LilyClip") {
            Some(dir) => dir,
            None => panic!("Couldn't get application data paths."),
        };
        let dir = dir.data_dir().to_owned();

        match std::fs::create_dir_all(&dir) {
            Ok(()) => (),
            Err(_) => panic!(
                "Couldn't create program data folder in {}",
                dir.to_str().unwrap()
            ),
        }

        ProgramData { path: dir }
    }

    pub fn get_saved_lines(&self, filename: &str) -> Vec<String> {
        let file = File::open(self.path.join(filename))
            .expect("Saved data file is missing");

		let buf = std::io::BufReader::new(file);
		buf.lines().filter_map(std::io::Result::ok).collect()
    }
}
