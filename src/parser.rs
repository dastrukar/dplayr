/// A bunch of functions for parsing stuff here

use regex::Regex;

const REGEX_PRESETSTART:  &str = r"start;(\S+)?";
const REGEX_PRESETEND:    &str = r"end;"; 
const REGEX_VARS:         &str = r"\B\$\S+=(\S+)?";
const REGEX_NAMES:        &str = r"\B\$\S+=";
const REGEX_VALUES:       &str = r"=(\S?)+";
const REGEX_PRESET_SPLIT: &str = r"[^,]+";


pub struct Vars<'v> {
    pub names:  Vec<&'v str>,
    pub values: Vec<&'v str>,
}

impl<'v> Vars<'v> {
    pub fn new() -> Self {
        Vars { names: Vec::new(), values: Vec::new() }
    }

    pub fn get_value_of(&self, name: &str) -> Result<&str, ()> {
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
// Remove functions
//------------------


/// Removes all "comments" found in the given `cfg` and returns it
pub fn remove_comments(cfg: &String) -> String {
    let re = Regex::new(r"\#.+").unwrap();
    let mut result = String::new();
    let mut offset: usize = 0;

    // Get anything that isn't a "comment"
    for m in re.find_iter(cfg) {
        result.push_str(
            &cfg[offset..m.start()]
            .to_string()
        );

        offset = m.end();
    }

    // Get strings that may have been missed
    result.push_str(
        &cfg[
            offset..cfg
                .chars()
                .count()
        ]
    );

    result
}


//=========================================================
// Get functions
//---------------

/// Returns variable declarations from the given `cfg`
pub fn get_var_declares(cfg: &String) -> Vec<&str> {
    let re_vars = Regex::new(REGEX_VARS).unwrap();
    let mut vars: Vec<&str> = Vec::new();

    for m in re_vars.find_iter(&cfg) { vars.push(m.as_str()); }

    vars
}


/// Returns all preset names


/// Returns `Vec<usize>` of the positions for the given preset `name`
pub fn get_preset_start_end(name: &str, cfg: &String)
    -> Result<Vec<usize>, ()> {
    let re_presetstart = Regex::new(REGEX_PRESETSTART).unwrap();
    let re_presetend = Regex::new(REGEX_PRESETEND).unwrap();

    let mut start_to_end: Vec<usize> = Vec::new();

    // Find where the preset params are
    for start in re_presetstart.find_iter(&cfg) {
        let len = start
            .as_str()
            .chars()
            .count();

        // Check name
        // Note: &String[..] creates a static `str`
        // It's done here due how slicing strings makes static strings
        if start.as_str()[6..len] == name[..] { 
            let end = re_presetend
                .find_at(&cfg, start.start())
                .unwrap();

            start_to_end
                .push(start.end());
            start_to_end
                .push(end.start());
        }
    }

    if start_to_end.len() < 2 {
        return Err(());
    }

    Ok(start_to_end)
}


/// Returns parameters from the given preset `name`
pub fn get_preset_params<'a> (name: &'a str, cfg: &'a String)
    -> Vec<&'a str> {
    let range = get_preset_start_end(&name, &cfg)
        .expect(&format!("\n\n[!!Error while finding presets!!]\nUnable to find preset with the name {:?}.\n\n", name));

    let re_start = Regex::new(REGEX_PRESETSTART).unwrap();
    let mut result: Vec<&str> = Vec::new();

    let cfg_slice = &cfg[range[0]..range[1]];
    for item in cfg_slice.split_whitespace() {
        if !re_start.find(&item).is_none() { 
            let name = String::from(name);
            panic!(format!("\n\nUnexpected \"start;\" found inside preset {:?}\n\n", name));
        }
        result.push(item);
    }

    result
}


/// Returns variable names and values
pub fn get_vars(cfg: &String) -> Vars {
    // Get the variables first
    let variables = get_var_declares(&cfg);

    let mut vars = Vars::new();

    let re_names = Regex::new(REGEX_NAMES)
        .expect("Regex error!");
    let re_values = Regex::new(REGEX_VALUES)
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
pub fn get_preset<'v> (cfg: &'v String, vars: &'v Vars) -> Result<Vec<&'v str>, ()> {
    let preset_var = vars.get_value_of("preset")?;

    let mut presets: Vec<&str> = Vec::new();
    let re_split = Regex::new(REGEX_PRESET_SPLIT).unwrap();

    for preset in re_split
        .find_iter(&preset_var) {
        presets.push(
            preset
                .as_str()
        );
    }

    // Get the preset parameters
    let mut result: Vec<&str> = Vec::new();
    for preset in &presets {
        result.append(&mut get_preset_params(preset, cfg));
    }

    println!("$preset={:?}", presets);
    Ok(result)
}
