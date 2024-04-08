package li1611

func divingBoard(shorter int, longer int, k int) []int {
	if k == 0 {
		return []int{}
	}
	if shorter == longer {
		return []int{shorter * k}
	}

	result := make([]int, k+1)

	for i := 0; i <= k; i++ {
		result[i] = shorter*(k-i) + longer*i
	}

	return result
}
