// #[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::fs;

fn main() {
  let contents: String = fs::read_to_string("./input.txt")
      .expect("Something went wrong reading the file");

  let input_list: Vec<&str> = contents.split("\n").collect();
  let re = Regex::new(r"^(forward|down|up) ([0-9])$").unwrap();

  let mut x: u32 = 0;
  let mut y: u32 = 0;

  for input in input_list {
    for cap in re.captures_iter(input) { 
      let direction: &str = &cap[1];
      let magnitude: u32 = (&cap[2]).parse::<u32>().unwrap();

      match direction {
        "forward" => x += magnitude,
        "down" => y += magnitude,
        "up" => y -= magnitude,
        _ => println!("Yikes")
      }
    }
  }

  println!("ANSWER: {}", x * y);
}
