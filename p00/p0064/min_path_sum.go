package p0064

func minPathSum(grid [][]int) int {
	rowCount := len(grid)
	colCount := len(grid[0])
	miniSum := make([][]int, rowCount)

	for row, rowV := range grid {
		miniSum[row] = make([]int, colCount)

		for col, v := range rowV {
			if row == 0 && col == 0 {
				miniSum[row][col] = v
				continue
			}
			if row == 0 {
				miniSum[row][col] = miniSum[row][col-1] + v
				continue
			}
			if col == 0 {
				miniSum[row][col] = miniSum[row-1][col] + v
				continue
			}
			miniSum[row][col] = min(miniSum[row-1][col], miniSum[row][col-1]) + v
		}
	}

	return miniSum[rowCount-1][colCount-1]
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
