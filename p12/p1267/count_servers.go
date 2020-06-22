package p1267

func countServers(grid [][]int) int {
	row, col := make([]int, len(grid)), make([]int, len(grid[0]))
	for i, gridRow := range grid {
		for j, v := range gridRow {
			if v == 1 {
				row[i]++
				col[j]++
			}
		}
	}
	count := 0
	for i, gridRow := range grid {
		for j, v := range gridRow {
			if v == 1 && (row[i] >= 2 || col[j] >= 2) {
				count++
			}
		}
	}
	return count
}
