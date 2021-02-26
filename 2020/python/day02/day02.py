#!/usr/bin/env python
# -*- coding: utf-8 -*-
#
# Day 03
#
# Example input:
# 1-3 a: abcde
# 1-3 b: cdefg
# 2-9 c: ccccccccc
#
#


def parse_line(line):
    """Return num 1, num 2, char, and password from a single input line"""
    num1 = int(line.split("-")[0])
    num2 = int(line.split("-")[1].split(" ")[0])
    character = line.split(" ")[1].split(":")[0]
    password = line.split(" ")[2]
    # print(f"{line} :: ({num1}, {num2}, {character}, {password})")
    return num1, num2, character, password


def is_valid_part1(line):
    """Check validity according to ruleset 1"""
    char_min, char_max, required_char, password = parse_line(line)
    char_count = password.count(required_char)
    if (char_min <= char_count <= char_max):
        return True
    return False


def is_valid_part2(line):
    """Check validity according to ruleset 2"""
    pos1, pos2, required_char, password = parse_line(line)
    pos1 = pos1 - 1
    pos2 = pos2 - 1
    if required_char in (password[pos1], password[pos2]):
        if not(password[pos1] == password[pos2]):
            return True
    return False



if __name__ == "__main__":

    input_file = "./input"

    # start with no valid passwords
    valid_p1 = 0
    valid_p2 = 0

    # check each entry
    with open(input_file, "r") as inputs:
        for line in inputs:
            line = line.strip()

            if is_valid_part1(line):
                valid_p1 += 1

            if is_valid_part2(line):
                valid_p2 += 1

    # result
    print(f"valid passwords - part 1: {valid_p1}")
    print(f"valid passwords - part 2: {valid_p2}")

