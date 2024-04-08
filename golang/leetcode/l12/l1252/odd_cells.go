package l1252

func oddCells(n int, m int, indices [][]int) int {
	rows := make([]bool, n)
	cols := make([]bool, m)

	for _, v := range indices {
		rows[v[0]] = !rows[v[0]]
		cols[v[1]] = !cols[v[1]]
	}

	result := 0

	for row := 0; row < n; row++ {
		for col := 0; col < m; col++ {
			if rows[row] && !cols[col] || !rows[row] && cols[col] {
				result++
			}
		}
	}
	return result
}
