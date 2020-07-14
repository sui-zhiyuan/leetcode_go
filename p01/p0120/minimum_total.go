package p0120

func minimumTotal(triangle [][]int) int {
	n := len(triangle)
	curr := make([]int, n)
	prev := make([]int, n)
	prevLine := []int{0}
	for _, line := range triangle {
		prev, curr = curr, prev
		for j, v := range line {
			values := make([]int, 0, 2)
			if j-1 >= 0 {
				values = append(values, prev[j-1])
			}
			if j < len(prevLine) {
				values = append(values, prev[j])
			}
			curr[j] = v + min(values...)
		}
		prevLine = line
	}

	return min(curr...)
}

func min(t ...int) int {
	result := t[0]
	for _, v := range t[1:] {
		if result > v {
			result = v
		}
	}
	return result
}
