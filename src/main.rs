mod file;
mod parser;
mod config;

use std::io::Result;
use std::process::Command;

use crate::parser::Vars;

fn main() -> Result<()> {

    if !file::check_config() {
        println!("\nNo config file found. \nCreating a new one");
        file::create_config();

        println!("\nExiting! Please edit your config file.");
        return Ok(());
    } else {
        println!("\nExisting config file found.");
    }

    let config: String = parser::remove_comments(&file::get_config());
    let vars:   Vars   = parser::get_vars(&config);

    println!("Variables:\n{:?}\n\nValues:\n{:?}", vars.names, vars.values);

    let parameters: Vec<String> = parser::get_parameters(&config, &vars);
    println!("{:?}", parameters);

    let presets = parser::get_preset(&config, &vars);
    let presets: Vec<String> = match presets {
        Ok(str) => str,
        Err(()) => Vec::new(),
    };

    println!("{:?}", presets);


    // Run the game
    let run = vars.get_value_of("srcprt")
        .expect("\n\nCouldn't find variable \"srcprt\".\n\n");

    Command::new(&run)
        .args(&parameters)
        .args(&presets)
        .status()
        .expect(&format!("\n\nFailed to execute process.\nPerhaps the directory given was incorrect?\n\nCommand:{:?}{:?}\n\n", run, presets));
    Ok(())
}
