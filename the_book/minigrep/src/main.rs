use std::env;
use std::process;

use minigrep::*;

fn main() {
    // $config takes the ownership from the iterator $env::args()
    let config = Config::new(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);


    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
