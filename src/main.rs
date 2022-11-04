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
fn day_3_part_1(input_path: &Path) {
    let mut bits_of_bits: Vec<HashMap<char, u32>> = vec![HashMap::new(); 12];
    for bits_x12 in get_lines(input_path) {
        for (position, bit) in bits_x12.chars().into_iter().enumerate() {
            bits_of_bits[position].entry(bit).or_insert(0);
            *bits_of_bits[position].get_mut(&bit).unwrap() += 1;
        }
    }
    let gamma_bits = bits_of_bits.iter().map(|t| {
        let mut chosen_bit: Option<char> = None;
        let mut highest_count: u32 = 0;
        for (bit, count) in t {
            if count > &highest_count {
                chosen_bit = Some(*bit);
                highest_count = *count;
            }
        }
        chosen_bit.unwrap()
    }).collect::<String>();
    let epsilon_bits = gamma_bits.chars().into_iter().map(|bit| if bit == '0' {'1'} else {'0'}).collect::<String>();
    println!("{}", usize::from_str_radix(&gamma_bits, 2).unwrap() * usize::from_str_radix(&epsilon_bits, 2).unwrap());
}

#[allow(dead_code)]
fn day_3_part_2(input_path: &Path) {
    let mut bits_of_bits: Vec<HashMap<char, u32>> = vec![HashMap::new(); 12];
    for bits_x12 in get_lines(input_path) {
        for (position, bit) in bits_x12.chars().into_iter().enumerate() {
            bits_of_bits[position].entry(bit).or_insert(0);
            *bits_of_bits[position].get_mut(&bit).unwrap() += 1;
        }
    }

    let mut most_common_bits = [None; 12];
    for (i, t) in bits_of_bits.iter().enumerate() {
        let mut chosen_bit: char = '1';
        if t.get(&'0').unwrap() > t.get(&'1').unwrap() {
            chosen_bit = '0';
        }
        most_common_bits[i] = Some(chosen_bit);
    }
    let most_common_bits = most_common_bits.iter().map(|t| t.unwrap()).collect::<String>();
    println!("{}", most_common_bits);

    let mut least_common_bits = [None; 12];
    for (i, t) in bits_of_bits.iter().enumerate() {
        let mut chosen_bit: char = '0';
        if t.get(&'1').unwrap() < t.get(&'0').unwrap() {
            chosen_bit = '1';
        }
        least_common_bits[i] = Some(chosen_bit);
    }
    let least_common_bits = least_common_bits.iter().map(|t| t.unwrap()).collect::<String>();
    println!("{}", least_common_bits);
}

fn main() {
    let current_directory = env::current_dir().unwrap();
    //day_1_part_1(&current_directory.join("input_day_1.txt"));
    //day_1_part_2(&current_directory.join("input_day_1.txt"));

    //day_2_part_1(&current_directory.join("input_day_2.txt"));
    //day_2_part_2(&current_directory.join("input_day_2.txt"));

    //day_3_part_1(&current_directory.join("input_day_3.txt"));
    day_3_part_2(&current_directory.join("input_day_3.txt"));
}
