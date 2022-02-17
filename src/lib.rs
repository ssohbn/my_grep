use std::error::Error;

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn searchtest() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn first_word_test() {
    let line: &str = "hello this is test";
    assert_eq!("hello", first_word(line));
  }
}

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
    if args.len() < 3 {
      eprintln!("need more args");
    }

    let query = args.get(1).expect("no query").clone();
    let filename = args.get(2).expect("no filename").clone();
    
    Ok( Config { query, filename} )
  }

}

// 'a is a lifetime linking
// contents to the return value
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut hits: Vec<&str> = vec![];
  let lines = contents.lines();
  for line in lines {
    if line.contains(query) {
      hits.push(line);
    }
  }
  hits
}

/// find da first word in a line
fn first_word(line: &str) -> &str {
  let bytes = line.as_bytes();
  for (i, b) in bytes.iter().enumerate() {
    if b == &b' ' {
      return &line[0..i];
    }
  }
  &line[..]
}