/// For my sanity

pub struct Config {
}

impl Config {
    pub fn new<'s> () -> &'s str {
// Start of config file
"# This is a comment

# These are variables
$srcprt= # <- Defines where your preferred source port is
$preset= # <- Defines what preset you'll be using
$skill=3 # <- This is a custom variable

# Any parameter that is outside of a preset will always be used
# Example parameter, that is calling variable \"$skill\"
-skill [$skill]

# This is the start of a preset, named \"default\"
start;default
    # Parameters that are put in here will only be used if this preset is being used
end;
# This is the end of a preset.
"
// End of config file
    }
}
