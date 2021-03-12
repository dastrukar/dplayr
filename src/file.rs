/// File related stuff

use std::fs;
use std::path::Path;

/// Creates a config file at the current directory.
pub fn create_config() {
    fs::write("dplayr.cfg", "test")
        .expect("Couldn't create file.");
}

/// Opens "dplayr.cfg".
/// Returns `std::fs::File`.
pub fn get_config() -> fs::File {
    fs::File::open("dplayr.cfg")
        .expect("Couldn't open config file.")
}

/// Returns true if dplayr.cfg exists.
pub fn check_config() -> bool {
    Path::new("dplayr.cfg")
        .exists()
}


