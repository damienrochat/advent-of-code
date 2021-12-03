use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn read_input_file(path: &str) -> Result<Vec<u32>, io::Error> {
  let f = File::open(path).expect("Unable to open file");
  let f = BufReader::new(f);

  let numbers = f
    .lines()
    .map(|line| line.unwrap().parse::<u32>().unwrap())
    .collect();

  Ok(numbers)
}

fn count_increasing(numbers: &Vec<u32>) -> usize {
  numbers
    .windows(2)
    .filter(|elem| { elem[0] < elem[1] })
    .count()
}

fn count_windows_increasing(numbers: &Vec<u32>, window_size: usize) -> usize {
  let sums = numbers
    .windows(window_size)
    .map(|window| { window.iter().sum::<u32>() })
    .collect::<Vec<u32>>();

  count_increasing(&sums)
}

fn main() {
  let numbers = read_input_file("input.txt").unwrap();

  let part1 = count_increasing(&numbers);
  assert_eq!(part1, 1446);
  println!("Part 1: {}", part1);

  let part2 = count_windows_increasing(&numbers, 3);
  assert_eq!(part2, 1486);
  println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_count_increasing() {
    assert_eq!(count_increasing(&Vec::from([1])), 0);
    assert_eq!(count_increasing(&Vec::from([1, 0])), 0);
    assert_eq!(count_increasing(&Vec::from([1, 1])), 0);
    assert_eq!(count_increasing(&Vec::from([1, 2])), 1);
    assert_eq!(count_increasing(&Vec::from([1, 2, 3, 3, 4, 3])), 3);
  }

  #[test]
  fn test_count_windows_increasing() {
    assert_eq!(count_windows_increasing(&Vec::from([1, 2]), 3), 0);
    assert_eq!(count_windows_increasing(&Vec::from([199, 200, 208, 210]), 3), 1);
    assert_eq!(count_windows_increasing(&Vec::from([199, 200, 208, 210, 20]), 3), 1);
  }
}
