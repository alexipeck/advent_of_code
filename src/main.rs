use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
fn day_2_part_1() {
    let mut horizontal_position = 0;
    let mut depth = 0;
    for line in BufReader::new(File::open(r"D:\adventofcode\input_day_2.txt").unwrap())
        .lines()
        .map(|t| t.unwrap())
        .collect::<Vec<String>>()
    {
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
    for line in BufReader::new(File::open(r"D:\adventofcode\input_day_2.txt").unwrap())
        .lines()
        .map(|t| t.unwrap())
        .collect::<Vec<String>>()
    {
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
    //day_2_part_1();
    //day_2_part_2();
}
