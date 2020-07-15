package p1176

func dietPlanPerformance(calories []int, k int, lower int, upper int) int {
	n := len(calories)
	sum := 0
	for i := 0; i < k-1; i++ {
		sum += calories[i]
	}

	result := 0
	for i := k - 1; i < n; i++ {
		sum += calories[i]

		if sum < lower {
			result--
		}
		if sum > upper {
			result++
		}

		sum -= calories[i-k+1]
	}

	return result
}
