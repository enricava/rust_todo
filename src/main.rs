use std::{env, process};
use rust_todo::{
    Config,
    run
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    }); 

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
