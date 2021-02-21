package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

// Return slice of expenses as ints
func get_expenses(input_file string) ([]int, error) {

	input, err := os.Open(input_file)
	if err != nil {
		return nil, err
	}
	defer input.Close()

	var expenses []int
	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		expense, err := strconv.Atoi(scanner.Text())
		if err != nil {
			// skip
		} else {
			expenses = append(expenses, expense)
		}
	}

	return expenses, nil

}

// Return the two expenses which sum to 2020
func part1(expenses []int) (int, int) {
	for _, x := range expenses {
		for _, y := range expenses {
			if x+y == 2020 {
				return x, y
			}
		}
	}
	return 0, 0
}

// Return the three expenses which sum to 2020
func part2(expenses []int) (int, int, int) {
	for _, x := range expenses {
		for _, y := range expenses {
			for _, z := range expenses {
				if x+y+z == 2020 {
					return x, y, z
				}
			}
		}
	}
	return 0, 0, 0
}

// Day01 program
func main() {

	expenses, err := get_expenses("./input")
	if err != nil {
		fmt.Println("error reading input")
		os.Exit(2)
	}

	a, b := part1(expenses)
	fmt.Printf("expenses: %v, %v; product: %v \n", a, b, a*b)

	a, b, c := part2(expenses)
	fmt.Printf("expenses: %v, %v, %v; product: %v \n", a, b, c, a*b*c)

}
