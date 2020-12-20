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

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

// Point in time policy and password
#[derive(Debug)]
pub struct PitPass {
    min: i32,
    max: i32,
    required_char: char,
    password: String,
}

// Return vector of parsed input params from each line
//
// The input is per line, and consistently uses the following structure:
// {min_int}-{max_int} {char_required}: {string_password}
//
pub fn process_input(input_filename: impl AsRef<Path>) -> i32 {
    let mut valid_count = 0;

    let fh = File::open(input_filename).expect("Error opening input file");
    let input = io::BufReader::new(fh);
    for line in input.lines() {
        if let Ok(line) = line {
            if let Ok(entry) = parse_line(line) {
                if is_valid(entry) {
                    valid_count = valid_count + 1;
                }
            }
        }
    }

    return valid_count;
}

// Parse an input line and return a PitPass
fn parse_line(_line: String) -> io::Result<PitPass> {
    return Ok(PitPass {
        min: 1,
        max: 2,
        required_char: 'a',
        password: String::from("foo"),
    });
}

// Return true if the point in time password entry is valid
fn is_valid(_entry: PitPass) -> bool {
    return true;
}
