use std::env;
use my_grep::{self, Config};
use std::process;
use std::fs;

fn main() {

  let args: Vec<String> = env::args().collect();
  let config: Config = Config::new(&args)
  .unwrap_or_else(|err| {
    println!("{}", err);
    process::exit(1);
  });
  
  let contents = fs::read_to_string(config.filename).expect("work plz");
  for line in my_grep::search(&config.query, &contents) {
    println!("{}", line);
  }

}

