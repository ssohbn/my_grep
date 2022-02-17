use std::env;
use my_grep::{self, Config};
use std::process;

fn main() {

  let args: Vec<String> = env::args().collect();
  let config: Config = Config::new(&args)
  .unwrap_or_else(|err| {
    println!("{}", err);
    process::exit(1);
  });
  
  println!("{} {}", config.query, config.filename);
  // let contents = fs::read_to_string(filename)
  // .expect("file not found or something");

}

