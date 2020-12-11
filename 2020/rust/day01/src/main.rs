/*
 * Day 01 - parts 1 and then modified for part 2
 * https://adventofcode.com/2020/day/1
*/

use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    println!("Day 01!");

    let target_sum = 2020;

    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    let numbers = input_nums(input_filename).expect("Error getting numbers from input file");

    println!("");
    println!("---------------------------------------------------------");
    println!(":: Part 1");
    println!("---------------------------------------------------------");
    part1(&numbers[..], target_sum);

    println!("");
    println!("---------------------------------------------------------");
    println!(":: Part 2");
    println!("---------------------------------------------------------");
    part2(&numbers[..], target_sum);

    println!("")
}

// Return a vector of numbers from the input file
fn input_nums(filename: impl AsRef<Path>) -> io::Result<Vec<i32>> {
    let infile = File::open(filename).expect("Error opening file");
    let buff = io::BufReader::new(infile);
    Ok(buff
        .lines()
        .map(|l| l.expect("line error"))
        .map(|l| l.parse::<i32>().unwrap())
        .collect())
}

// Part 1 - Find the two numbers which produce the target sum,
// and then print the product of those two numbers
fn part1(numbers: &[i32], target_sum: i32) {
    let mut idx = 0;
    'outer: for i in numbers {
        for j in &numbers[idx..] {
            if i + j == target_sum {
                println!("Target sum found! {} + {} == {}", i, j, target_sum);
                println!("Product! {} * {} = {}", i, j, i * j);
                break 'outer;
            }
        }
        idx = idx + 1;
    }
}

// Part 2 - Find the three numbers which produce the target sum,
// and then print the product of those three numbers
fn part2(numbers: &[i32], target_sum: i32) {
    let mut idx = 0;
    'outer: for i in numbers {
        for j in &numbers[idx..] {
            for k in numbers {
                if i + j + k == target_sum {
                    println!("Target sum found! {} + {} + {} == {}", i, j, k, target_sum);
                    println!("Product! {} * {} * {} = {}", i, j, k, i * j * k);
                    break 'outer;
                }
            }
        }
        idx = idx + 1;
    }
}
