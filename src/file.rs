/// File related stuff

use std::{fs, io::Read};
use std::fs::File;
use std::path::Path;
use std::io::BufReader;

/// Creates a config file at the current directory.
pub fn create_config() {
    fs::write("dplayr.cfg",
"# This is a comment
$srcprt=/home/das/Games/gzdoom/gzdoom
$preset=

# These are \"items\"
item1
item2
item3

start;presetname
    item4
    item5
    item6
end;")
        .expect("Couldn't create file.");
}

/// Opens "dplayr.cfg" and returns its contents.
/// Returns `String`.
pub fn get_config() -> String {
    let file = File::open("dplayr.cfg")
        .expect("Couldn't open config file.");
    let mut text = String::new();

    BufReader::new(file).read_to_string(&mut text).expect("Something went wrong!");
    text
}

/// Returns true if dplayr.cfg exists.
pub fn check_config() -> bool {
    Path::new("dplayr.cfg")
        .exists()
}
