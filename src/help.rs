pub struct Help {
    version: String,
}

impl Help {
    pub fn new(v: &str) -> Self {
        Help { version: String::from(v) }
    }

    pub fn get_help(&self) -> String {
format!(
"dPlayR {}
by dastrukar
\"An improved version of dPlay.\"
Wiki: https://github.com/dastrukar/dplayr/wiki

Arguments:
  -h, --help    | Shows this text.
  -v, --verbose | Enables verbose mode.
  -s, --silent  | Hides all output, including output from the source port used.

Special Variables (Config):
  $srcprt  | Used for telling dPlayR where your preferred source port is located.
  $preset  | Used for stating what presets you want to use.

Note: Enabling both silent and verbose mode is not allowed. Doing so will result in a panic.
"
, self.version)
    }
}
