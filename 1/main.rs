use std::fs;

fn main() {
  let contents: String = fs::read_to_string("./input.txt")
      .expect("Something went wrong reading the file");

  let input_list: Vec<&str> = contents.split("\n").collect();

  let mut increase_count: u32 = 0;
  let mut previous_number: u32 = std::u32::MAX;

  for current_number_string in input_list {
    let current_number: u32 = current_number_string.parse::<u32>().unwrap();
    if current_number > previous_number {
      increase_count+= 1;
    }

    previous_number = current_number;
  }

  println!("ANSWER: {}", increase_count);
}
