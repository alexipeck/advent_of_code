use std::collections::{HashMap};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use iterslide::SlideIterator;

fn get_lines(path: &Path) -> Vec<String> {
    match File::open(path) {
        Ok(file) => {
            BufReader::new(file)
                .lines()
                .map(|t| t.unwrap())
                .collect::<Vec<String>>()
        },
        Err(err) => {
            panic!("Error opening file: {}", err);
        }
    }
    
}

#[allow(dead_code)]
fn day_1_part_1(input_path: &Path) {
    let mut count = 0;
    let mut last: Option<usize> = None;
    for line in get_lines(input_path).iter().map(|line| line.parse::<usize>().unwrap()) {
        if let Some(last) = last {
            if line > last {
                count += 1;
            }
        }
        last = Some(line);
    }
    println!("{} measurements are larger than the previous measurement", count);
}

#[allow(dead_code)]
fn day_1_part_2(input_path: &Path) {
    let mut count = 0;
    let mut last: Option<usize> = None;
    for grouped_values in get_lines(input_path).iter().map(|line| line.parse::<usize>().unwrap()).collect::<Vec<usize>>().slide(3) {
        let grouped_values_sum: usize = grouped_values.iter().sum();
        if let Some(last) = last {
            if grouped_values_sum > last {
                count += 1;
            }
        }
        last = Some(grouped_values_sum);
    }
    println!("{} measurements are larger than the previous measurement", count);
}

#[allow(dead_code)]
fn day_2_part_1(input_path: &Path) {
    let mut horizontal_position = 0;
    let mut depth = 0;
    for line in get_lines(input_path) {
        let (direction, amount) = {
            let mut t = line.split(' ');
            (
                t.next().unwrap().to_string(),
                t.next().unwrap().parse::<i32>().unwrap(),
            )
        };
        match direction.as_ref() {
            "forward" => {
                horizontal_position += amount;
            }
            "down" => {
                depth += amount;
            }
            "up" => {
                depth -= amount;
            }
            _ => panic! {},
        }
    }
    println!(
        "horizontal position: {}, depth: {}",
        horizontal_position, depth
    );
    println!(
        "horizontal position multiplied by depth: {}",
        horizontal_position * depth
    );
}

#[allow(dead_code)]
fn day_2_part_2(input_path: &Path) {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in get_lines(input_path) {
        let (direction, amount) = {
            let mut t = line.split(' ');
            (
                t.next().unwrap().to_string(),
                t.next().unwrap().parse::<i32>().unwrap(),
            )
        };
        match direction.as_ref() {
            "forward" => {
                horizontal_position += amount;
                depth += aim * amount;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => panic! {},
        }
    }
    println!(
        "horizontal position: {}, depth: {}",
        horizontal_position, depth
    );
    println!(
        "horizontal position multiplied by depth: {}",
        horizontal_position * depth
    );
}

#[allow(dead_code)]
struct BitTracker {
    pub bits_of_bits: Vec<HashMap<char, u32>>,
}

impl BitTracker {
    #[allow(dead_code)]
    pub fn new(bit_count: usize) -> Self {
        Self {
            bits_of_bits: vec![HashMap::new(); bit_count]
        }
    }
}

impl BitTracker {
    #[allow(dead_code)]
    fn increment(&mut self, bit: &char, position: usize) {
        if !self.bits_of_bits[position].contains_key(bit) {
            self.bits_of_bits[position].insert(*bit, 0);
        }
        *self.bits_of_bits[position].get_mut(bit).unwrap() += 1;
    }

    #[allow(dead_code)]
    fn gamma_binary(&self) -> String {
        self.bits_of_bits.iter().map(|k| {
            let mut chosen_bit: Option<char> = None;
            let mut highest_count: u32 = 0;
            for (bit, count) in k {
                if count > &highest_count {
                    chosen_bit = Some(*bit);
                    highest_count = *count;
                }
            }
            chosen_bit.unwrap()
        }).collect::<String>()
    }

    #[allow(dead_code)]
    fn epsilon_binary(&self) -> String {
        self.bits_of_bits.iter().map(|k| {
            let mut chosen_bit: Option<char> = None;
            let mut lowest_count: u32 = u32::MAX;
            for (bit, count) in k {
                if count < &lowest_count {
                    chosen_bit = Some(*bit);
                    lowest_count = *count;
                }
            }
            chosen_bit.unwrap()
        }).collect::<String>()
    }
}

#[allow(dead_code)]
fn day_3_part_1(input_path: &Path) {
    let mut bit_counter = BitTracker::new(12);
    for bits_x12 in get_lines(input_path) {
        for (position, bit) in bits_x12.chars().into_iter().enumerate() {
            bit_counter.increment(&bit, position);
        }
    }

    let gamma_bits = bit_counter.gamma_binary();
    let gamma_int = usize::from_str_radix(&gamma_bits, 2).unwrap();
    println!("{}: {}", gamma_bits, gamma_int);

    let epsilon_bits = bit_counter.epsilon_binary();
    let epsilon_int = usize::from_str_radix(&epsilon_bits, 2).unwrap();
    println!("{}: {}", epsilon_bits, epsilon_int);

    let gamma_x_epsilon = gamma_int * epsilon_int;
    println!("{}", gamma_x_epsilon);
}

#[allow(dead_code)]
fn day_3_part_2(input_path: &Path) {
    for bits_x12 in get_lines(input_path) {
        for (position, bit) in bits_x12.chars().into_iter().enumerate() {
            println!("{}: {}", position, bit);
        }
    }
}

fn main() {
    let current_directory = env::current_dir().unwrap();
    println!("{}", current_directory.display());
    //day_1_part_1(&current_directory.join("input_day_1.txt"));
    //day_1_part_2(&current_directory.join("input_day_1.txt"));

    //day_2_part_1(&current_directory.join("input_day_2.txt"));
    //day_2_part_2(&current_directory.join("input_day_2.txt"));
    
    //day_3_part_1(&current_directory.join("input_day_3.txt"));
    day_3_part_2(&current_directory.join("input_day_3.txt"));
}
