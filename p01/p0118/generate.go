package p0118

func generate(numRows int) [][]int {
	result := make([][]int, numRows)

	for i := range result {
		result[i] = make([]int, i+1)
		for j := range result[i] {
			left := 0
			if j-1 >= 0 {
				left = result[i-1][j-1]
			}
			right := 0
			if j < i {
				right = result[i-1][j]
			}
			sum := left + right
			if sum == 0 {
				sum = 1
			}
			result[i][j] = sum
		}
	}
	return result
}
