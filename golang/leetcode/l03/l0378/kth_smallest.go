package l0378

func kthSmallest(matrix [][]int, k int) int {
	n := len(matrix)
	arr := make([]int, 0, n*n)
	curr := make([]int, n)

	for {
		next := -1
		min := 0
		for i := range matrix {
			if curr[i] < n && (next < 0 || matrix[i][curr[i]] < min) {
				next = i
				min = matrix[i][curr[i]]
			}
		}

		if next == -1 {
			break
		}

		arr = append(arr, min)
		curr[next]++
	}
	return arr[k-1]
}
