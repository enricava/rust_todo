use std::{
    fs,
    fs::OpenOptions,
    error::Error,
    io::prelude::*
};

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
            Some(args[2].clone())
        } else {
            None
        };
        
        Ok(Config { cmd , args})
    }
}

/// * `create` must create a new todo list
/// * `list` must list current todo list
/// * `add` `item` must insert a new todo item in the list
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filepath = "todo_list";
    match config.cmd.as_str() {
        "new" => {
            fs::write(filepath, "")?;
            println!("Created new todo list");
            Ok(())
        }
        "list" => {
            let contents = fs::read_to_string(filepath)?;
            print!("Todo list:\n{}", contents);
            Ok(())
        }
        "add" => {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(filepath)?;

            writeln!(file, "* {}", config.args.unwrap())?;
            Ok(())
        }
        _ => Err("unrecognized command.")?
    }
}