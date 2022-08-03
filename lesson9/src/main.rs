use std::env;

use std::process;

use lesson9::Config;
use lesson9::run;

fn main() {
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        println!("woring {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("error: {}", e);
        process::exit(1);
    }   
}


