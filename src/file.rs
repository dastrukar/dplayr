// File related stuff goes here

use std::fs;
use std::io;
use std::path::Path;

/*
 * Creates a config file at the given directory.
 * Returns nothing.
 */
pub fn create_config() {
    fs::write("dplayr.cfg", "test")
        .expect("Couldn't create file.");
}

/*
 * Opens "dplay.cfg".
 * Returns `std::fs::File`.
 */
pub fn get_config() -> io::Result<fs::File> {
    fs::File::open("dplayr.cfg")
}

pub fn check_config() -> bool {
    Path::new("dplayr.cfg").exists()
}
