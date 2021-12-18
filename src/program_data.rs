//! Used to manage the program data in the filesystem

use directories::ProjectDirs;
use std::fs::File;
use std::io::BufRead;

/// Program data manager
pub struct ProgramData {
    /// Path to the program data directory
    pub path: std::path::PathBuf,
}

impl ProgramData {
    /// Creates a new program data
    ///
    /// It also handles program data directory creation if it doesn't exist.  
    /// Created directory is located at:
    /// ```
    /// Linux: $HOME/.local/share
    /// Windows: {FOLDERID_RoamingAppData}
    /// MacOS: $HOME/Library/ApplicationSupport
    /// ```
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

    /// Get all saved lines from the file at the given path
    pub fn get_saved_lines(&self, filename: &str) -> Vec<String> {
        let file = File::open(self.path.join(filename)).expect("Saved data file is missing");

        let buf = std::io::BufReader::new(file);
        buf.lines().filter_map(std::io::Result::ok).collect()
    }
}
