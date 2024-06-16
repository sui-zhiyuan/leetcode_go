package main

func countCompleteDayPairs(hours []int) int64 {
	var input = make([]int64, 24)

	for _, v := range hours {
		input[v%24] += 1
	}

	var result int64 = 0
	for i := 1; i < 12; i++ {
		result += input[i] * input[24-i] / 2
	}

	result += input[12] * (input[12] - 1) / 2
	result += input[0] * (input[0] - 1) / 2

	return result
}

func main() {
	countCompleteDayPairs([]int{13, 11})
}
