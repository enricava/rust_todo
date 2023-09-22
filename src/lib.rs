use std::{
    error::Error,
    fs,
    fs::OpenOptions,
    io::{prelude::*, BufReader},
    path::PathBuf,
};

use dirs::home_dir;

pub struct Config {
    pub cmd: String,
    pub args: Option<String>,
}

impl Config {
    /// Arguments in config must be one of:
    /// `new`, `list`, `add` `item`, `copy` `path`, `delete` `index`.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments.
                Usage: list, new, add [name], copy [path], delete [index]");
        }

        let cmd = args[1].clone();

        let args = if (cmd == "add" || cmd == "copy") && args.len() < 3 {
            return Err("
                not enough arguments.
                Usage: list, new, add [name], copy [path], delete [index]");
        } else if cmd == "add" {
            Some(args[2..].join(" ").clone())
        } else if cmd == "copy" {
            Some(args[2].clone())
        } else if cmd == "delete" {
            Some(args[2].parse().unwrap())
        } else {
            None
        };

        Ok(Config { cmd, args })
    }
}

/// Creates a new file that will hold the todo list.
/// Truncates by default.
fn new_todolist(filepath: &PathBuf) -> Result<(), Box<dyn Error>> {
    fs::write(filepath, "")?;
    println!("Created new todo list");
    Ok(())
}

/// Lists the lines in the todo list.
fn list_todolist(filepath: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(filepath)?;
    let reader = BufReader::new(file).lines();
    for (id, line) in reader.enumerate() {
        println!("{id}. {}", line.unwrap());
    }
    Ok(())
}

/// Adds a new item to the end of the list.
fn add_todolist(filepath: &PathBuf, item: String) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().write(true).append(true).open(filepath)?;

    writeln!(file, "{}", item)?;
    Ok(())
}

/// Copies the contents of a file in `otherpath` into the todolist in `filepath`
fn copy_todolist(filepath: &PathBuf, otherpath: String) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().write(true).append(true).open(filepath)?;

    let copy_contents = fs::read_to_string(otherpath)?;

    write!(file, "{}", copy_contents)?;

    Ok(())
}

/// Deletes the item at position `index` from the list.
fn delete_todolist(filepath: &PathBuf, index: usize) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(filepath)?;
    let mut lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    if index >= lines.len() {
        return Err("index out of bounds.".into());
    }

    lines.remove(index);

    let mut file = fs::File::create(filepath)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

/// * `create` must create a new todo list
/// * `list` must list current todo list
/// * `add` `item` must insert a new todo item in the list
/// * `copy` `otherpath` must copy a file into the list
/// * `delete` `id` removes line from the list
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filepath: PathBuf = home_dir()
        .unwrap_or_else(|| {
            println!("could not find home directory, using curent location");
            PathBuf::new()
        })
        .join("todo_list");

    match config.cmd.as_str() {
        "new" => new_todolist(&filepath),
        "list" => list_todolist(&filepath),
        "add" => add_todolist(&filepath, config.args.unwrap()),
        "copy" => copy_todolist(&filepath, config.args.unwrap()),
        "delete" => delete_todolist(&filepath, config.args.unwrap().parse()?),
        _ => {
            Err("unrecognized command. Usage: list, new, add [name], copy [path], delete [index]")?
        }
    }
}
