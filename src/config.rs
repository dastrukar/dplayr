/// For my sanity

pub struct Config {
}

impl Config {
    pub fn new<'s> () -> &'s str {
// Start of config file
"# This is a comment

# :Variables:
# Variables are declared with the following syntax:
# $name=value
#
# ^ Note how there are no spaces.
# 
# Variables can then be called with the following syntax:
# $name
# 
# Variables can also be used for declaring other variables:
# $name=$variable
#
# There are 2 special variables:
# $srcprt, and $preset
#
# The $srcprt variable is used to tell dPlayR where your preferred source port is located.
# It has to be declared, or else dPlayR will throw an error.
#
# The $preset variable is used for stating what presets you want to use.
# It's a special variable, and can take multiple values.
# Each value is seperated by a comma[,].
# Example:
# $preset=preset1,preset2,preset3
#
# It should be noted that, variables declared in a preset, will still be declared.
# Even if the preset isn't being used.

$srcprt=
$preset=default
$testvar1=value
$testvar2=test


# Note: Parameters outside of presets will always be used.
# :Presets:
# Example of a preset:
# start;presetname
#     parameter1
#     parameter2
# end;
#
# A preset can also be written in one line:
# start;presetname parameter1 parameter2 end;
#
# It should be noted that presets can't contain other presets.
# Doing so will cause an error.

start;default
    # Example preset
end;
"
// End of config file
    }
}
