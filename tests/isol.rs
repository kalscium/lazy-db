use std::fs;
use std::path::Path;
use std::fmt::Display;
use std::fmt;

static mut UID: u8 = 0;

pub fn new_env() -> TmpPath {
    // creates 'test_tmp' folder if it doesn't exist
    let path = Path::new("./test_tmp/");
    if !path.exists() {
        fs::create_dir_all(path).expect("Error: couldn't create directory!");
    }; unsafe { UID += 1; };
    unsafe { TmpPath::new(format!("./test_tmp/{UID}")) }
}

pub struct TmpPath(String);

impl TmpPath {
    pub fn new(path: String) -> Self {
        let path_link = Path::new(&path);
        if !path_link.exists() {
            match fs::create_dir_all(path_link) {
                Ok(_) => (),
                Err(e) => panic!("Error: creating directory: {:?}", e),
            }
        } Self(path)
    }
}

impl Display for TmpPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",  &self.0)
    }
}

impl Drop for TmpPath {
    fn drop(&mut self) {
        // Ignore errors
        let _ = fs::remove_dir_all(&self.0);
    }
}