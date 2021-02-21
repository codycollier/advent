#!/usr/bin/env python
# -*- coding: utf-8 -*-


def get_expenses(input_file="./input"):
    """Return all expenses as an ordered iterable"""
    inputs = []
    with open(input_file, "r") as input:
        for line in input.readlines():
            line.strip()
            inputs.append(int(line))
    return tuple(inputs)


def part1(expenses):
    """Return the two expenses which sum to 2020"""
    for x in expenses:
        for y in expenses:
            if x == y:
                continue
            else:
                if x + y == 2020:
                    return x, y
    return 0, 0


def part2(expenses):
    """Return the three expenses which sum to 2020"""
    for x in expenses:
        for y in expenses:
            for z in expenses:
                if x + y + z == 2020:
                    return x, y, z
    return 0, 0, 0



if __name__ == "__main__":


    expenses = get_expenses()

    a, b = part1(expenses)
    print(f"part1: {a}, {b}; product: {a * b}")

    a, b, c = part2(expenses)
    print(f"part2: {a}, {b}, {c}; product: {a * b * c}")


