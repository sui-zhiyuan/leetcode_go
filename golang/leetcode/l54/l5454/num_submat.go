package l5454

// cspell:ignore submat

func numSubmat(mat [][]int) int {
	lenRow := len(mat)
	lenCol := len(mat[0])
	ans := 0
	for row, colV := range mat {
		for col, v := range colV {
			if v == 1 {
				ans += countSqure(mat, lenRow, lenCol, row, col)
			}

		}
	}
	return ans
}

func countSqure(mat [][]int, lenRow, lenCol int, row, col int) int {
	result := 0
	maxCol := lenCol
	for rows := row; rows < lenRow; rows++ {
		cols := col
		for ; cols < maxCol; cols++ {
			if mat[rows][cols] == 1 {
				result++
				continue
			}
			if maxCol >= cols {
				maxCol = cols
			}
			break
		}
		if cols == col {
			break
		}
	}

	return result
}
