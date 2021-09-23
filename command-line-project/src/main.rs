use std::env; // use to read arguments
use std::process; // use to stop process

use command_line_project::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = command_line_project::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}


