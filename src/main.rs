//! #norp

// use std::env; // To allow access of CLI arguments
use std::process; // So the program may be terminated early

fn main() {
    if let Err(e) = norp::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
