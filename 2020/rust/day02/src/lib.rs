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

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

// Point in time policy and password
#[derive(Debug)]
pub struct PitPass {
    min: usize,
    max: usize,
    required_char: char,
    password: String,
}

// Return a total count of the valid entries
pub fn process_input(input_filename: impl AsRef<Path>) -> Result<(usize, usize), Box<dyn Error>> {
    let mut valid_count_p1 = 0;
    let mut valid_count_p2 = 0;

    let fh = File::open(input_filename).or(Err("Error opening input file"))?;
    let input = io::BufReader::new(fh);
    for line in input.lines() {
        // mask per line errors, and accumulate count of valid / invalid entries
        if let Ok(line) = line {
            if let Ok(entry) = parse_line(line) {
                if is_valid_p1(&entry) {
                    valid_count_p1 = valid_count_p1 + 1;
                }
                if is_valid_p2(&entry) {
                    valid_count_p2 = valid_count_p2 + 1;
                }
            }
        }
    }

    return Ok((valid_count_p1, valid_count_p2));
}

// Parse an input line and return a PitPass
//
// Each line consistently has the following structure:
//  {min_int}-{max_int} {char_required}: {string_password}
//
// Example:
//  * 3-5 f: fgfff
//
fn parse_line(line: String) -> Result<PitPass, Box<dyn Error>> {
    // Parse into vars or return an error
    let min = line
        .split("-")
        .nth(0)
        .ok_or("line split error")?
        .parse::<usize>()
        .or(Err("parse error"))?;

    let max = line
        .split(&['-', ' '][..])
        .nth(1)
        .ok_or("line split error")?
        .parse::<usize>()
        .or(Err("parse error"))?;

    let required_char = line
        .split(&[' ', ':'][..])
        .nth(1)
        .ok_or("line split error")?
        .parse::<char>()
        .or(Err("parse error"))?;

    let pass = String::from(line.split(" ").nth(2).ok_or("line split error")?);

    // Parse success, return PitPass
    let p = PitPass {
        min: min,
        max: max,
        required_char: required_char,
        password: pass,
    };

    return Ok(p);
}

// // Return true it the point in time password entry is valid (part 1 rules)
fn is_valid_p1(entry: &PitPass) -> bool {
    let count = entry.password.matches(entry.required_char).count();
    return (entry.min <= count) && (count <= entry.max);
}

// Return true it the point in time password entry is valid (part 2 rules)
fn is_valid_p2(entry: &PitPass) -> bool {
    let count = entry.password.matches(entry.required_char).count();
    return (entry.min <= count) && (count <= entry.max);
}
