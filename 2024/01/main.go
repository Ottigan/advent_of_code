package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
)

func main() {
	left, right := getSortedLists()
	totalDistance := getTotalDistance(left, right)
	similarity := getSimilarity(left, right)

	fmt.Println("Total distance: ", totalDistance)
	fmt.Println("Similarity: ", similarity)
}

func getSortedLists() ([]int, []int) {
	file, _ := os.Open("./input.txt")

	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	var left, right []int

	for scanner.Scan() {
		var l, r int
		fmt.Sscanf(scanner.Text(), "%d %d", &l, &r)

		left = append(left, l)
		right = append(right, r)
	}

	sort.Ints(left)
	sort.Ints(right)

	return left, right
}

func getTotalDistance(left, right []int) int {
	var totalDistance int

	for i := 0; i < len(left); i++ {
		distance := right[i] - left[i]

		if distance < 0 {
			distance = -distance
		}

		totalDistance += distance
	}

	return totalDistance
}

func getSimilarity(left, right []int) int {
	var tracker = make(map[int]int)
	var similarity int

	for i := 0; i < len(left); i++ {
		l := left[i]

		if tracker[l] != 0 {
			fmt.Println("Found in tracker: ", left[i])
			similarity += tracker[left[i]]
			continue
		}

		var count int
		for j := 0; j < len(right); j++ {
			if l == right[j] {
				count++
			}
		}

		currentSimilarity := l * count
		tracker[left[i]] = currentSimilarity
		similarity += currentSimilarity
	}

	return similarity
}
