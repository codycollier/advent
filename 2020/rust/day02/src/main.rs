/*
 * Day 02 - ...
 * https://adventofcode.com/2020/day/2
 *
 */

use std::env;

use day02;

fn main() {
    println!("Day 02!");

    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    match day02::process_input(input_file) {
        Ok((valid_count_p1, valid_count_p2)) => {
            println!("Valid Password count - part 1: {:?}", valid_count_p1);
            println!("Valid Password count - part 2: {:?}", valid_count_p2);
        }
        Err(e) => {
            println!("Error with input file ({:?}): {:?}", input_file, e);
        }
    }

    println!("");
}
