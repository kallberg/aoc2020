use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn day_one() {
  let file = File::open("input/1.txt").expect("could not read input 1");
  let mut reader = BufReader::new(file);
  let mut entry_set = HashSet::new();

  loop {
    let mut line = String::new();
    reader
      .read_line(&mut line)
      .expect("io error while reading line from input 1");

    if line.len() == 0 {
      break;
    }

    let number: u32 = line.trim().parse().expect("line did not contain a number");
    entry_set.insert(number);
  }

  let mut part_one_values: Vec<u32> = entry_set.into_iter().collect();
  let mut part_two_values: Vec<u32> = part_one_values.clone();

  'part_one: loop {
    if let Some(n1) = part_one_values.pop() {
      for n2 in &part_one_values {
        if n1 + n2 == 2020 {
          println!("{}", n1 * n2);
          break 'part_one;
        }
      }
    } else {
      panic!("We failed!");
    }
  }

  'part_two: loop {
    if let Some(n1) = part_two_values.pop() {
      for n2 in &part_two_values {
        for n3 in &part_two_values {
          if n1 + n2 + n3 == 2020 {
            println!("{}", n1 * n2 * n3);
            break 'part_two;
          }
        }
      }
    } else {
      panic!("We failed!");
    }
  }
}
