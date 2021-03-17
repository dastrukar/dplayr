/// File related stuff

use std::{fs, io::Read};
use std::fs::File;
use std::path::Path;
use std::io::BufReader;

use crate::config::Config;

/// Creates a config file at the current directory.
pub fn create_config() {
    fs::write("dplayr.cfg", Config::new())
        .expect("Failed to create config file.");
}

/// Opens "dplayr.cfg" and returns its contents.
/// Returns `String`.
pub fn get_config() -> String {
    let file = File::open("dplayr.cfg")
        .expect("Couldn't open config file.");
    let mut cfg = String::new();

    BufReader::new(file).read_to_string(&mut cfg).expect("Something went wrong!");
    cfg
}

/// Returns true if dplayr.cfg exists.
pub fn check_config() -> bool {
    Path::new("dplayr.cfg")
        .exists()
}
