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

