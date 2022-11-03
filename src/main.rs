use std::fs::File;
use std::io::{BufRead, BufReader};

use iterslide::SlideIterator;

fn get_lines(path: &str) -> Vec<String> {
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
fn day_1_part_1() {
    let mut count = 0;
    let mut last: Option<usize> = None;
    for line in get_lines(r"D:\adventofcode\input_day_1.txt").iter().map(|line| line.parse::<usize>().unwrap()) {
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
fn day_1_part_2() {
    let mut count = 0;
    let mut last: Option<usize> = None;
    for grouped_values in get_lines(r"D:\adventofcode\input_day_1.txt").iter().map(|line| line.parse::<usize>().unwrap()).collect::<Vec<usize>>().slide(3) {
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
fn day_2_part_1() {
    let mut horizontal_position = 0;
    let mut depth = 0;
    for line in get_lines(r"D:\adventofcode\input_day_2.txt") {
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
fn day_2_part_2() {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in get_lines(r"D:\adventofcode\input_day_2.txt") {
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

fn main() {
    //day_1_part_1();
    //day_1_part_2();

    //day_2_part_1();
    //day_2_part_2();
}
