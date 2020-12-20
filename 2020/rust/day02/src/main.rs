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
        Ok(valid_count) => {
            println!("Valid Password count: {:?}", valid_count);
        }
        Err(e) => {
            println!("Error with input file ({:?}): {:?}", input_file, e);
        }
    }

    println!("");
}
