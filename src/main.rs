mod file;
mod parser;
mod config;
mod help;

use std::io::Result;
use std::process::Command;
use std::env;

use crate::parser::Vars;
use crate::help::Help;

const VERSION: &str = "v0.2.1";

fn main() -> Result<()> {
    let mut verbose: bool = false;
    let mut silent:  bool = false;

    let args: Vec<String> = env::args().collect();

    let help: Help = Help::new(VERSION);

    for arg in args {
        if arg == "--help"    || arg == "-h" { println!("{}", help.get_help()); return Ok(()); }
        if arg == "--verbose" || arg == "-v" { verbose = true; }
        if arg == "--silent"  || arg == "-s" { silent = true; }
    }

    if verbose && silent { panic!("\n\nCan't have both silent and verbose mode enabled.\nPlease enable only one.\n\n"); }

    if !file::check_config() {
        match silent {
            false => {
                println!("\nNo config file found. \nCreating a new one.");
                file::create_config();

                println!("\nExiting! Please edit your config file.");
            },
            true => {
                file::create_config();
            },
        }
        return Ok(());
    } else {
        if !silent { println!("\nExisting config file found."); }
    }

    let config: String = parser::remove_comments(&file::get_config());
    let vars:   Vars   = parser::get_vars(&config);

    if verbose { println!("Variables:\n{:?}\n\nValues:\n{:?}", vars.names, vars.values); }

    let arguments: Vec<String> = parser::get_arguments(&config, &vars);

    if verbose { println!("Arguments:\n{:?}", arguments); }

    let presets = parser::get_preset(&config, &vars);
    let presets: Vec<String> = match presets {
        Ok(str) => str,
        Err(()) => Vec::new(),
    };

    if verbose { println!("Presets:\n{:?}", presets); }


    // Run the game
    let run = vars.get_value_of("srcprt")
        .expect("\n\nCouldn't find variable \"srcprt\".\n\n");

    if silent {
        Command::new(&run)
            .args(&arguments)
            .args(&presets)
            .output()
            .expect(&format!("\n\nFailed to execute process.\nPerhaps the directory given was incorrect?\n\nCommand:{:?}{:?}\n\n", run, presets));
    } else {
        Command::new(&run)
            .args(&arguments)
            .args(&presets)
            .status()
            .expect(&format!("\n\nFailed to execute process.\nPerhaps the directory given was incorrect?\n\nCommand:{:?}{:?}\n\n", run, presets));
    }
    Ok(())
}
