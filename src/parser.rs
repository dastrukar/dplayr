/// A bunch of functions for parsing stuff here

use regex::{Regex, Match};

const REGEX_COMMENTS:       &str = r"\#(.+)?";
const REGEX_PRESETSTART:    &str = r"start;(\S+)?";
const REGEX_PRESETEND:      &str = r"end;"; 
const REGEX_VARS:           &str = r"\B\$\S+=(\S+)?";
const REGEX_NAMES:          &str = r"\B\$\S+=";
const REGEX_VALUES:         &str = r"=(\S+)?";
const REGEX_PRESET_SPLIT:   &str = r"[^,]+";
const REGEX_CONTAINER:      &str = r"\[\$(\S[^\]]+)?\]";


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
        let mut is_exist:     bool  = false;

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
    let mut result: String = String::new();
    let mut offset: usize  = 0;

    // Get anything that isn't a "comment"
    for m in re.find_iter(cfg) {
        result.push_str(
            &cfg[offset..m.start()]
            .to_string()
        );

        offset = m.end()+1;
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
// Match Functions
//-----------------

/// Returns for variable declarations from the given `cfg`
pub fn match_var_declares(cfg: &String) -> Vec<Match> {
    let re_vars:     Regex      = Regex::new(REGEX_VARS).unwrap();
    let mut matches: Vec<Match> = Vec::new();

    for m in re_vars.find_iter(cfg) {
            matches.push(m);
    }

    matches
}


/// Returns Matches if the following pattern is found:
/// [$name]
pub fn match_contained_var(text: &str) -> Vec<Match> {
    let re_container: Regex = Regex::new(REGEX_CONTAINER).unwrap();

    let mut result: Vec<Match> = Vec::new();
    for i in re_container.find_iter(text) {
        result.push(i);
    }

    result
}


/// Returns preset container matches in the following format:
/// `Vec< container_match: Vec<Match>, name: &str >`
pub fn match_preset_containers(text: &str) -> Vec<(Vec<Match>, &str)> {
    let re_presetstart: Regex = Regex::new(REGEX_PRESETSTART).unwrap();
    let re_presetend:   Regex = Regex::new(REGEX_PRESETEND).unwrap();

    let mut matches: Vec<(Vec<Match>, &str)> = Vec::new();

    // Find where the preset params are
    for start in re_presetstart.find_iter(text) {
        let len: usize = start
            .as_str()
            .chars()
            .count();

        // Get name
        let name: &str = &start.as_str()[6..len];

        // Check name
        let end: Match = re_presetend
            .find_at(&text, start.end())
            .unwrap();

        let mut pack: Vec<Match> = Vec::new();
        pack.push(start);
        pack.push(end);

        matches.push((pack, name));

    }

    matches
}


//=========================================================
// Get functions
//---------------

/// Used to process variables contained in []
/// Returns the variable's value
pub fn get_contained_var(text: &str, vars: &Vars)
    -> String {
    let mut result:  String = String::new();
    let mut var_end: usize  = 0;

    for m in match_contained_var(text) {
        let var_len:   usize = m.as_str().chars().count();
        let var_name:  &str  = &m.as_str()[2..var_len-1];
        let var_value: &str  = vars.get_value_of(var_name)
            .expect(&format!("\n\nUnknown variable {:?}\n\n", var_name));

        if m.start() == 0 { // []
            result.push_str(var_value);
        } else // [][]
        if m.start() == var_end {
            result.push_str(var_value);
        } else { // text[]
            result.push_str(&text[var_end..m.start()]);
            result.push_str(var_value);
        }

        var_end = m.end();
    }

    // Get the remaining bits of the string
    if var_end < text.chars().count() { 
        println!("{}", &text[var_end..text.chars().count()]);
        result.push_str(&text[var_end..text.chars().count()]);
    }

    result
}


/// Returns `Vec<usize>` of the positions for the given preset `name`
pub fn get_preset_range(name: &str, cfg: &String) -> Vec<usize> {
    let mut range: Vec<usize> = Vec::new();

    for set in match_preset_containers(cfg) {
        if set.1 == name {
            let m: Vec<Match> = set.0;

            range.push(m[0].end());
            range.push(m[1].start());
        }
    }

    match range.len() {
        0 => panic!(format!("\n\n[!!Error while finding presets!!]\nUnable to find preset with the name {:?}.\n\n", name)),
        1 => panic!(format!("\n\n[!!Error while finding presets!!]\nUnable to find end; of {:?}.\n\n", name)),
        _ => return range,
    }
}


/// Returns parameters from the given preset `name`
pub fn get_preset_params<'a> (name: &'a str, cfg: &'a String, vars: &'a Vars)
    -> Vec<String> {
    let range: Vec<usize> = get_preset_range(&name, &cfg);

    let re_start: Regex = Regex::new(REGEX_PRESETSTART).unwrap();
    let mut result: Vec<String> = Vec::new();

    let cfg_slice: &str = &cfg[range[0]..range[1]];
    for item in cfg_slice.split_whitespace() {
        if re_start.find(&item).is_some() { 
            let name: String = String::from(name);
            panic!(format!("\n\nUnexpected \"start;\" found inside preset {:?}\n\n", name));
        }

        // Check variables
        result.push(get_contained_var(item, vars));
    }

    result
}


/// Returns variable names and values
pub fn get_vars(cfg: &String) -> Vars {
    // Get the variables first
    let mut variables: Vec<&str> = Vec::new();

    for m in match_var_declares(cfg) {
        variables.push(m.as_str()); 
    }

    let mut vars: Vars = Vars::new();

    let re_names:  Regex = Regex::new(REGEX_NAMES).unwrap();
    let re_values: Regex = Regex::new(REGEX_VALUES).unwrap();

    // Get "variable name"
    for name in &variables {
        let result: &str = re_names.find(&name)
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
        let result: &str = re_values.find(&value)
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
    let var_decs: Vec<Match> = match_var_declares(cfg);
    let preset_containers:
        Vec<(Vec<Match>, &str)> = match_preset_containers(cfg);

    let mut is_slicing: bool = true;

    let mut index:   usize = 0;
    let mut v_index: usize = 0;
    let mut p_index: usize = 0;

    let cfg_len = cfg.chars().count();

    // Get parameters
    let mut parameters: Vec<String> = Vec::new();

    let var_len: usize = var_decs.len();
    let set_len: usize = preset_containers.len();

    while is_slicing {
        let slice: String;

        let var_pos: usize = if v_index == var_len { 0 }
            else { var_decs[v_index].start() };

        let set_pos: usize = if p_index == set_len { 0 }
            else { preset_containers[p_index].0[0].start() };

        // Check if `var_pos` is next or `set_pos`
        if (var_pos < set_pos || p_index == set_len) && v_index != var_len {
            // Variable
            slice = cfg[index..var_pos].to_string();
            index = var_decs[v_index].end();

            v_index += 1;
        } else 
        if (set_pos < var_pos || v_index == var_len) && p_index != set_len {
            // Preset
            slice = cfg[index..set_pos].to_string();
            index = preset_containers[p_index].0[1].end();

            p_index += 1;
        } else {
            // No more variables or presets left
            slice = cfg[index..cfg_len].to_string();

            is_slicing = false;
        }

        for text in slice.split_whitespace() {
            parameters.push(get_contained_var(text, vars));
        }
    }

    parameters
}


/// Returns the parameters from a preset
pub fn get_preset<'v> (cfg: &'v String, vars: &'v Vars) -> Result<Vec<String>, ()> {
    let preset_var: &str = vars.get_value_of("preset")?;

    let mut presets: Vec<&str> = Vec::new();
    let re_split: Regex = Regex::new(REGEX_PRESET_SPLIT).unwrap();

    for preset in re_split.find_iter(&preset_var) {
        presets.push(
            preset.as_str()
        );
    }

    // Get the preset parameters
    let mut result: Vec<String> = Vec::new();
    for preset in &presets {
        result.append(&mut get_preset_params(preset, cfg, vars));
    }

    Ok(result)
}
