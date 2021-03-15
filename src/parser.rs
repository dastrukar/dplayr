/// A bunch of functions for parsing stuff here

use regex::Regex;

struct Parser;

/// Returns a value based on the line type
/// `#` : Returns 1
/// `$` : Returns 2
/// `:` : Returns 3
///
/// If none of the above apply, return 0
pub fn get_line_type(text: &String) -> i32 {
    let c = text.chars().nth(0).expect("Failed to load text");

    let result = match c {
        '#' => 1,
        '$' => 2,
        ':' => 3,
        _ => 0,
    };

    result
}

/// Removes all "comments" found in the given string and returns it
pub fn remove_comments(text: &String) -> String {
    let re = Regex::new(r"\#.+").expect("Regex error in \"remove_comments\"!");
    let mut result = String::new();
    let mut offset: usize = 0;

    // Get anything that isn't a "comment"
    for m in re.find_iter(text) {
        result.push_str(&text[offset..m.start()].to_string());

        offset = m.end();
    }

    // Get strings that may have been missed
    result.push_str(&text[offset..text.chars().count()]);

    result
}

/// Returns the variable name
pub fn get_var_name(text: &String) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();

    let re = Regex::new(r"\B\$.\S+=").expect("Regex error in \"get_var_name\"!");

    // Get "variable name"
    for m in re.find_iter(text) {
        let name = m.as_str();
        let len  = name.chars().count();

        names.push(String::from(name)[1..len-1].to_string());
    }

    names
}

/// Returns the variable value
pub fn get_var_value(text: &String) -> Vec<String> {
    let mut values: Vec<String> = Vec::new();

    let re = Regex::new(r"=.?\S*").expect("Regex error in \"get_var_value\"!");

    // Get "variable value"
    for m in re.find_iter(text) {
        let value = m.as_str();
        let len   = value.chars().count();

        if len <= 1 {
            values.push("".to_string());
        } else {
            values.push(String::from(value)[1..len].to_string());
        }
    }
    
    values
}
