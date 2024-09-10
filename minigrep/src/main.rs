use minigrep::*;
use std::env;
use std::process;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // let cfg = parse_config(&args);
    // let cfg = Config::new(&args);
    let cfg = Config::build(env::args()).unwrap_or_else(|err| {

    	// eprintln! macro writes to standard error
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {:?}", cfg.query);
    println!("In file {:?}", cfg.file_path);

    if let Err(e) = run(cfg) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
