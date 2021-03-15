/// A bunch of functions for parsing stuff here

use regex::Regex;

/// Removes all "comments" found in the given string and returns it
pub fn remove_comments(text: &String) -> String {
    let re = Regex::new(r"\#.+")
        .expect("Regex error in \"remove_comments\"!");
    let mut result = String::new();
    let mut offset: usize = 0;

    // Get anything that isn't a "comment"
    for m in re.find_iter(text) {
        result.push_str(
            &text[offset..m.start()]
            .to_string()
        );

        offset = m.end();
    }

    // Get strings that may have been missed
    result.push_str(
        &text[
            offset..text
                .chars()
                .count()
        ]
        );

    result
}

/// Returns the variable name
pub fn get_var(text: &String) -> (Vec<String>, Vec<String>) {
    // Get the variables first
    let re_vars = Regex::new(r"\B\$\S+=(\S+)?")
        .expect("Regex error!");
    let mut vars: Vec<String> = Vec::new();

    for m in re_vars.find_iter(text) {
        vars.push(
            m.as_str()
            .to_string()
        );
    }

    let mut names: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();

    let re_names = Regex::new(r"\B\$\S+=")
        .expect("Regex error!");
    let re_values = Regex::new(r"=(\S?)+")
        .expect("Regex error!");

    // Get "variable name"
    for name in &vars {
        let result = re_names.find(&name)
            .unwrap()
            .as_str();
        names.push(
            result[
                1..result
                    .chars()
                    .count() - 1
            ].to_string()
        );
    }

    // Get "variable value"
    for value in &vars {
        let result = re_values.find(&value)
            .unwrap()
            .as_str();

        values.push(
            result[
                1..result
                    .chars()
                    .count()
            ].to_string()
        );
    }

    (names, values)
}
