use std::env;
use std::process;

// Import from lib.rs
use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // let query = &args[1];
    // let filename = &args[2];

    // println!("Searching for {}", query);
    // println!("In file {}", filename);

    // let (query, filename) = parse_config(&args);

    // let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    // Removed duplicate file reading and run() call since it's handled in run()

    // let results = search(&query, &contents);

    // for line in results {
    //     println!("{}", line);
    // }

    // let results = search(&config.query, &contents);
    // for line in results {
    //     println!("{}", line);
    // }

    // let poem = fs::read_to_string("poem.txt").expect("Should have been able to read the file");
    // println!("Poem:\n{}", poem);
}