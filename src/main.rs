use std::io;
use std::process;
use std::str::FromStr;

mod lib;
use lib::solver::{self, Lights};

fn main() {
    // Start
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let start = Lights::from_str(input_line.trim_end());
    if let Err(error) = start {
        println!("Error: cannot parse input due to {}", error);
        process::exit(1);
    }
    // Target
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let target = Lights::from_str(input_line.trim_end());
    if let Err(error) = target {
        println!("Error: cannot parse input due to {}", error);
        process::exit(1);
    }
    // Solver
    match solver::get_min_steps(start.unwrap(), target.unwrap(), Some(1000)) {
        Some(nb_steps) => println!("{}", nb_steps),
        None => println!("Did not found a solution..."),
    }
}
