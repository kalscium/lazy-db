use std::fs;
use std::path::{Path, PathBuf};
use std::fmt::Display;
use std::fmt;

static mut UID: u8 = 0;

pub fn new_env() -> TmpPath {
    // creates 'test_tmp' folder if it doesn't exist
    let path = Path::new("./test_tmp/");
    if !path.exists() {
        fs::create_dir_all(path).expect("Error: couldn't create directory!");
    }; unsafe {
        UID += 1;
        TmpPath::new(format!("./test_tmp/{UID}"))
    }
}

pub struct TmpPath(PathBuf);

impl TmpPath {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        if !path.exists() {
            match fs::create_dir_all(path) {
                Ok(_) => (),
                Err(e) => panic!("Error: creating directory: {:?}", e),
            }
        } Self(path.to_path_buf())
    }

    pub fn get_path(&self) -> &Path {
        self.0.as_path()
    }
}

impl Display for TmpPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",  self.0.to_string_lossy())
    }
}

impl Drop for TmpPath {
    fn drop(&mut self) {
        // Ignore errors
        let _ = fs::remove_dir_all(&self.0);
    }
}