use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Config::new(&args).unwrap_or_else(|err|
    {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}",conf.query);
    println!("In file {}",conf.file_path);
    if let Err(e) = run(&conf){
        println!("Application error: {e}");
        process::exit(1);
    }
}

struct Config<'a>{
    query: &'a str,
    file_path: &'a str
}

impl<'a> Config<'a>{
    fn new(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config{
            query: &args[1],
            file_path: &args[2],
        })
    }
}

fn run(config: &Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}

