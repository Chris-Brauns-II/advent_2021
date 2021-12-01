use std::fs;

fn main() {
  let contents: String = fs::read_to_string("./input.txt")
      .expect("Something went wrong reading the file");

  let input_string_list: Vec<&str> = contents.split("\n").collect();
  let input_list: Vec<u32> = input_string_list.iter().map(|x| x.parse::<u32>().unwrap()).collect();

  let mut increase_count: u32 = 0;
  let mut previous_vector = &input_list[0..3];

  for i in 1..(input_list.len() - 2) {
    let current_vector = &input_list[i..(i+3)];

    let current_sum:u32 = current_vector.iter().sum();
    let previous_sum:u32 = previous_vector.iter().sum();

    if  current_sum > previous_sum {
      increase_count += 1;
    }

    previous_vector = current_vector
  }

  println!("ANSWER: {}", increase_count);
}
