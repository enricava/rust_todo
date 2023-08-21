use std::{
    fs,
    fs::OpenOptions,
    error::Error,
    io::prelude::*,
    path::PathBuf,
};

use dirs::home_dir;

pub struct Config {
    pub cmd: String,
    pub args: Option<String>
}

impl Config {
    /// Arguments in config must be one of:
    /// `create`, `list`, `add` `item`.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments. Usage: list, new, add [name]");
        }

        let cmd = args[1].clone();

        let args = if cmd == "add" && args.len() < 3{
            return Err("not enough arguments. Usage: list, new, add [name]");
        } else if cmd == "add" {
            Some(args[2..].join(" ").clone())
        } else {
            None
        };
        
        Ok(Config { cmd , args})
    }
}

fn new_todolist(filepath: &PathBuf) -> Result<(), Box<dyn Error>>{
    fs::write(filepath, "")?;
    println!("Created new todo list");
    Ok(())
}

fn list_todolist(filepath: &PathBuf) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filepath)?;
    print!("Todo list:\n{}", contents);
    Ok(())
}

fn add_todolist(filepath: &PathBuf, item: String) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(filepath)?;

    writeln!(file, "* {}", item)?;
    Ok(())
}

/// * `create` must create a new todo list
/// * `list` must list current todo list
/// * `add` `item` must insert a new todo item in the list
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let filepath: PathBuf = home_dir().unwrap_or_else(|| {
        println!("could not find home directory, using curent location");
        PathBuf::new()
    }).join("todo_list");

    match config.cmd.as_str() {
        "new" => {
            new_todolist(&filepath)
        }
        "list" => {
            list_todolist(&filepath)
        }
        "add" => {
            add_todolist(&filepath, config.args.unwrap())
        }
        _ => Err("unrecognized command.")?
    }
}