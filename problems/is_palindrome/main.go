package main

import (
	"fmt"
	"strconv"
)

func main() {
	fmt.Println(isPalindrome(121))  // true
	fmt.Println(isPalindrome(-121)) // false
	fmt.Println(isPalindrome(10))   // false
}

func isPalindrome(x int) bool {
	index := 0
	s := strconv.Itoa(x)
	for index < len(s) {
		if s[index] != s[len(s)-1-index] {
			return false
		}

		index++
	}

	return true
}
