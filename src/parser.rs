/// A bunch of functions for parsing stuff here

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

/// Returns the variable name and value
pub fn get_var(text: &String) -> (String, String) {
    let mut i = 0;

    // Find where the '=' is
    for c in text.chars() {
        if c == '=' {
            break;
        }
        i += 1;
    }

    // If the variable doesn't have a name, panic.
    if i == 0 { panic!("Invalid variable."); }

    // Check if the variable has a value
    let length = text.chars().count();
    println!("{}", length);
    if length == i + 1 {
        return (String::from(&text[1..i]), String::from(""));
    }
    (String::from(&text[1..i]), String::from(&text[i+1..length]))
}
