package p0883

func projectionArea(grid [][]int) int {
	n := len(grid)
	rowMax, colMax, area := make([]int, n), make([]int, n), 0
	for row, rowVal := range grid {
		for col, high := range rowVal {
			if rowMax[row] < high {
				rowMax[row] = high
			}
			if colMax[col] < high {
				colMax[col] = high
			}
			if high > 0 {
				area++
			}
		}
	}

	for _, ra := range rowMax {
		area += ra
	}

	for _, ca := range colMax {
		area += ca
	}

	return area
}
