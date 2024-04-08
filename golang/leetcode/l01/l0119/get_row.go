package l0119

func getRow(rowIndex int) []int {
	rowIndex++
	curr := []int{}
	prev := []int{}

	for i := 1; i <= rowIndex; i++ {
		prev = curr
		curr = make([]int, i)
		for j := range curr {
			left := 0
			if j-1 >= 0 {
				left = prev[j-1]
			}
			right := 0
			if j < i-1 {
				right = prev[j]
			}
			sum := left + right
			if sum == 0 {
				sum = 1
			}
			curr[j] = sum
		}
	}
	return curr
}
