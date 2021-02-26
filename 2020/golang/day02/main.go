//
// Day 02
//
// example input:
// 1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc
//
//
package main

import (
	"bufio"
	"fmt"
	"os"
)

// Return num1, num2, character, and password from entry
func parseLine(line string) (int, int, string, string) {
	return 1, 2, "a", "foo"
}

// Return true if valid according to ruleset 1
func isValidPart1(line string) bool {
	charMin, charMax, charRequired, password := parseLine(line)
	charCount := 0
	for _, char := range password {
		if string(char) == charRequired {
			charCount += 1
		}
	}
	if (charMin <= charCount) && (charCount <= charMax) {
		return true
	}
	return false
}

// Return true if valid according to ruleset 2
func isValidPart2(line string) bool {
	pos1, pos2, char, password := parseLine(line)
	inPos1 := string(password[pos1-1]) == char
	inPos2 := string(password[pos2-1]) == char
	if (inPos1 || inPos2) && !(inPos1 && inPos2) {
		return true
	}
	return false
}

func main() {

	inputFile := "./input"
	fmt.Println("day 01")

	// initialize validity counters
	validPart1 := 0
	validPart2 := 0

	// Open input
	inputHandle, err := os.Open(inputFile)
	if err != nil {
		fmt.Println("Error opening file")
		os.Exit(2)
	}
	defer inputHandle.Close()

	// Iterate through list
	scanner := bufio.NewScanner(inputHandle)
	for scanner.Scan() {
		entry := scanner.Text()

		if isValidPart1(entry) {
			validPart1 += 1
		}

		if isValidPart2(entry) {
			validPart2 += 1
		}

	}

	// Results
	fmt.Printf("valid entries - part 1: %v\n", validPart1)
	fmt.Printf("valid entries - part 2: %v\n", validPart2)

}
