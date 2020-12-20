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

use day02;

fn main() {
    println!("Day 02!");

    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let passwords = day02::process_input(input_file).expect("Error parsing input file");
    for p in passwords {
        println!("::> {:?}", p);
    }

    println!("");
}
