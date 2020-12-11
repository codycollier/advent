/*
 * Day 02 - ...
 * https://adventofcode.com/2020/day/2
 *
 * Example input:
 * $ head input
 * 3-5 f: fgfff
 * 6-20 n: qlzsnnnndwnlhwnxhvjn
 * 6-7 j: jjjjjwrj
 * 8-10 g: gggggggggg
 * 5-6 t: ttttttft
 * 6-11 h: khmchszhmzm
 * 4-6 q: qqbjqqqj
 * 6-8 j: gtkwwjjj
 * 13-14 w: lwgwrwczwwhkww
 * 2-4 p: vcnptzdppvpdzp
 *
 */

use std::env;
use std::io;
use std::path::Path;

fn main() {
    println!("Day 02!");

    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let r = input_params(input_file).expect("Error parsing input file");
    for i in r {
        println!("::> {:?}", i);
    }

    println!("");
}

// Return vector of parsed input params from each line
fn input_params(_input_filename: impl AsRef<Path>) -> io::Result<Vec<i32>> {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    Ok(v)
}
