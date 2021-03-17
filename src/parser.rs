/// A bunch of functions for parsing stuff here

use regex::{Regex, Match};

const REGEX_COMMENTS:       &str = r"\#(.+)?";
const REGEX_PRESETSTART:    &str = r"start;(\S+)?";
const REGEX_PRESETEND:      &str = r"end;"; 
const REGEX_VARS:           &str = r"\B\$\S+=(\S+)?";
const REGEX_NAMES:          &str = r"\B\$\S+=";
const REGEX_VALUES:         &str = r"=(\S+)?";
const REGEX_PRESET_SPLIT:   &str = r"[^,]+";
const REGEX_CONTAINER:      &str = r"\[\$(\S+)?\]";
const REGEX_PRESETSTARTEND: &str = r"(start;(\S+)?)|(end;)";


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
    let re = Regex::new(REGEX_COMMENTS).unwrap();
    let mut result = String::new();
    let mut offset: usize = 0;

    // Get anything that isn't a "comment"
    for m in re.find_iter(cfg) {
        result.push_str(
            &cfg[offset..m.start()]
            .to_string()
        );

        println!("{}", &cfg[offset..m.start()]);
        offset = m.end()+1;
        println!("{}", offset);
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

/// Returns for variable declarations from the given `cfg`
pub fn get_var_declares(cfg: &String) -> Vec<Match> {
    let re_vars = Regex::new(REGEX_VARS).unwrap();
    let mut matches: Vec<Match> = Vec::new();

    for m in re_vars
        .find_iter(cfg) {
            matches.push(m);
    }

    matches
}


/// Returns Matches if the following pattern is found:
/// [$name]
pub fn get_container(text: &str) -> Vec<Match> {
    let re_container = Regex::new(REGEX_CONTAINER).unwrap();

    let mut result: Vec<Match> = Vec::new();
    for i in re_container.find_iter(text) {
        println!("YO");
        result.push(i);
    }

    result
}


/// Used to process variables contained in []
/// Returns the variable's value
pub fn get_contained_var(text: &str, m: &Match, vars: &Vars)
    -> String {
    let var_len   = m.as_str().chars().count();
    let var_name  = &m.as_str()[2..var_len-1];
    let var_value = vars.get_value_of(var_name)
        .expect(&format!("\n\nUnknown variable {:?}\n\n", var_name));

    println!("{}", var_value);
    println!("{}", text);
    let text_len = text.chars().count();
    println!("len:{} start-end:{}-{}", text.len(), m.start(), m.end());
    if text_len == m.end() && m.start() == 0 {
        return var_value.to_string();
    } else
    if text_len == m.end() {
        return text[0..m.start()].to_string() + &var_value;
    } else
    if m.start() == 0 {
        return var_value.to_string() + &text[m.end()+1..text_len];
    }
    text[0..m.start()].to_string() + &var_value + &text[m.end()+1..text_len]
}


/// Returns all preset names
pub fn get_preset_names(cfg: &String) -> Vec<&str> {
    let re_presetstart = Regex::new(REGEX_PRESETSTART).unwrap();

    let mut result: Vec<&str> = Vec::new();
    for start in re_presetstart.find_iter(cfg) {
        let s = start.as_str();
        result.push(
            &s[6..s.chars().count()]
        );
    }

    result
}


/// Returns `Vec<usize>` of the positions for the given preset `name`
pub fn get_preset_start_end(name: &str, cfg: &String)
    -> Vec<usize> {
    let re_presetstart = Regex::new(REGEX_PRESETSTART).unwrap();
    let re_presetend = Regex::new(REGEX_PRESETEND).unwrap();

    let mut start_to_end: Vec<usize> = Vec::new();

    // Find where the preset params are
    for start in re_presetstart.find_iter(cfg) {
        let len = start
            .as_str()
            .chars()
            .count();

        // Check name
        // Note: &String[..] creates a `str`
        if start.as_str()[6..len] == name[..] { 
            let end = re_presetend
                .find_at(&cfg, start.start())
                .unwrap();

            start_to_end
                .push(start.start());
            start_to_end
                .push(end.end());
        }
    }

    if start_to_end.len() < 2 {
        panic!(format!("\n\n[!!Error while finding presets!!]\nUnable to find preset with the name {:?}.\n\n", name));
    }

    start_to_end
}


/// Returns parameters from the given preset `name`
pub fn get_preset_params<'a> (name: &'a str, cfg: &'a String, vars: &'a Vars)
    -> Vec<String> {
    let range    = get_preset_start_end(&name, &cfg);
    let name_len = name.chars().count() + 5;

    let re_start = Regex::new(REGEX_PRESETSTART).unwrap();
    let mut result: Vec<String> = Vec::new();

    let cfg_slice = &cfg[range[0]+name_len..range[1]-4];
    for item in cfg_slice.split_whitespace() {
        if re_start.find(&item).is_some() { 
            let name = String::from(name);
            panic!(format!("\n\nUnexpected \"start;\" found inside preset {:?}\n\n", name));
        }

        // Check variables
        let mut has_pushed = false;
        for m in get_container(item) {
            result.push(
                get_contained_var(item, &m, vars)
            );
            has_pushed = true;
        }
        if !has_pushed { result.push(item.to_string()); }
    }

    result
}


/// Returns variable names and values
pub fn get_vars(cfg: &String) -> Vars {
    // Get the variables first
    let mut variables: Vec<&str> = Vec::new();

    for m in get_var_declares(cfg) {
        variables.push(m.as_str()); 
    }

    let mut vars = Vars::new();

    let re_names = Regex::new(REGEX_NAMES).unwrap();
    let re_values = Regex::new(REGEX_VALUES).unwrap();

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

/// Returns any parameters that is outside of a preset
/// (no variable declarations)
pub fn get_parameters(cfg: &String, vars: &Vars) -> Vec<String> {
    let var_decs = get_var_declares(cfg);
    let preset_names = get_preset_names(cfg);

    let mut slice_pos: Vec<usize> = Vec::new();

    // Get the positions of all Variable declarations and presets
    for var in var_decs {
        slice_pos.push(var.start());
        slice_pos.push(var.end());
    }

    // Get the positions of all start and end of presets
    for preset in preset_names {
        for i in get_preset_start_end(preset, cfg) {
            slice_pos.push(i);
        }
    }

    // Sort the Vec and get the length
    slice_pos.sort();
    let slice_len = slice_pos.len();

    // Safety checks
    if slice_pos[0] != 0 { slice_pos.push(0); } 
    else { slice_pos.remove(0); }
    if slice_pos[slice_len-1] != cfg.chars().count() { slice_pos.push(cfg.chars().count()); }
    else { slice_pos.remove(slice_len-1); }
    slice_pos.sort();
    println!("{:?}", slice_pos);

    // Get all parameters
    let mut parameters: Vec<String> = Vec::new();

    let re_presetstartend = Regex::new(REGEX_PRESETSTARTEND).unwrap();

    for i in 0..slice_len/2 {
        let index = i * 2;

        let slice = &cfg[slice_pos[index]..slice_pos[index+1]];
        println!("{:?}", slice);
        // Check for presets
        let preset_err = re_presetstartend.find(slice);
        if preset_err.is_some() {
            let err_msg = preset_err.unwrap().as_str();
            panic!(format!("\n\nUnexpected {:?}.\n\n", err_msg));
        }

        for s in slice.split_whitespace() {
            // Check for variables
            let mut has_pushed = false;
            for m in get_container(s) {
                // Get variable value
                parameters.push(
                    get_contained_var(s, &m, vars)
                );
                has_pushed = true;
            }
            if !has_pushed { parameters.push(s.to_string()); }
        }
    }

    parameters
}


/// Returns the parameters from a preset
pub fn get_preset<'v> (cfg: &'v String, vars: &'v Vars) -> Result<Vec<String>, ()> {
    let preset_var = vars.get_value_of("preset")?;

    let mut presets: Vec<&str> = Vec::new();
    let re_split = Regex::new(REGEX_PRESET_SPLIT).unwrap();

    for preset in re_split
        .find_iter(&preset_var) {
        presets.push(
            preset.as_str()
        );
    }

    // Get the preset parameters
    let mut result: Vec<String> = Vec::new();
    for preset in &presets {
        result.append(&mut get_preset_params(preset, cfg, vars));
    }

    println!("$preset={:?}", presets);
    Ok(result)
}
