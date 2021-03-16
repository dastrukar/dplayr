/// A bunch of functions for parsing stuff here

use regex::Regex;


pub struct Vars<'v> {
    names:   Vec<&'v str>,
    values: Vec<&'v str>,
}

impl<'v> Vars<'v> {
    fn new() -> Self {
        Vars { names: Vec::new(), values: Vec::new() }
    }

    fn get_value_of(&self, name: &str) -> Result<&str, ()> {
        let mut vector_index: usize = 0;
        let mut is_exist = false;

        // Find the variable's Vec index
        for variable in &self.names {
            if variable == &name { is_exist = true; break; }
            vector_index += 1;
        }

        // If variable doesn't exist, return an empty string
        if !is_exist { return Err(()); }

        Ok(self.values[vector_index])
    }
}


//=========================================================
// Fetch functions
//----------------

/// Returns variable declarations from the given `text`
pub fn fetch_var_dec(text: &String) -> Vec<&str> {
    let re_vars = Regex::new(r"\B\$\S+=(\S+)?")
        .expect("Regex error!");
    let mut vars: Vec<&str> = Vec::new();

    for m in re_vars.find_iter(&text) { vars.push(m.as_str()); }

    vars
}


pub fn fetch_preset_start_end(name: &str, text: &String) -> Vec<usize> {
    let re_presetstart = Regex::new(r"start;\S+")
        .expect("Regex error!");
    let re_presetend = Regex::new(r"end;")
        .expect("Regex error!");

    let mut start_to_end: Vec<usize> = Vec::new();

    // Find where the preset params are
    for start in re_presetstart.find_iter(&text) {
        let len = start
            .as_str()
            .chars()
            .count();

        // Check name
        // Note: &String[..] creates a static `str`
        // It's done here due how slicing strings makes static strings
        if start.as_str()[6..len] == name[..] { 
            let end = re_presetend
                .find_at(&text, start.start())
                .unwrap();

            start_to_end
                .push(start.end());
            start_to_end
                .push(end.start());
        }
    }

    start_to_end
}

//=========================================================
// Remove functions
//------------------

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

//=========================================================
// Get functions
//---------------

/// Returns variable names and values
pub fn get_var(text: &String) -> Vars {
    // Get the variables first
    let variables = fetch_var_dec(&text);

    let mut vars = Vars::new();

    let re_names = Regex::new(r"\B\$\S+=")
        .expect("Regex error!");
    let re_values = Regex::new(r"=(\S?)+")
        .expect("Regex error!");

    // Get "variable name"
    for name in &variables {
        let result = re_names.find(&name)
            .unwrap()
            .as_str();

        vars.names.push(
            &result[
                1..result
                    .chars()
                    .count() - 1
            ]
        );
    }

    // Get "variable value"
    for value in &variables {
        let result = re_values.find(&value)
            .unwrap()
            .as_str();

        vars.values.push(
            &result[
                1..result
                    .chars()
                    .count()
            ]
        );
    }

    vars
}

/// Returns the parameters from a preset
pub fn get_preset(text: &String, var: &Vars) -> Result<String, ()> {
    let preset_var = var.get_value_of("preset");
    let preset_var = match preset_var {
        Ok(str) => str,
        Err(()) => return Err(()),
    };

    let mut presets: Vec<&str> = Vec::new();
    let re_split = Regex::new(r"[^,]+")
        .expect("Regex error!");

    for preset in re_split
        .find_iter(&preset_var) {
        presets.push(
            preset
                .as_str()
        );
    }

    // Get the preset parameters
    let mut result = String::new();
    for preset in &presets {
        result.push_str(&get_preset_params(preset, text));
        println!("h");
    }

    println!("$preset={:?}", presets);
    Ok(result)
}

/// Returns parameters from the given preset `name`
pub fn get_preset_params<'a> (name: &str, text: &String) -> String {
    let range = fetch_preset_start_end(&name, &text);
    let re_nowhitespace = Regex::new(r"\S+")
        .expect("Regex error!");
    let mut result = String::new();

    let text_slice = &text[range[0]..range[1]];
    for item in re_nowhitespace
        .find_iter(&text_slice) {
            result.push_str(" ");
            result.push_str(item.as_str());
    }

    result
}
