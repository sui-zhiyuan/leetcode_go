package s01

func generateMatrix(n int) [][]int {
	metrix := make([][]int, n)
	for i := 0; i < n; i++ {
		metrix[i] = make([]int, n)
	}

	row, col := 0, 0
	dir := 0
	value := 1
	for value <= n*n {
		metrix[row][col] = value
		value++
		row, col, dir = nextLocation(row, col, metrix, n, dir)
	}
	return metrix
}

var directions = [][]int{{0, 1}, {1, 0}, {0, -1}, {-1, 0}}

func nextLocation(row, col int, metrix [][]int, n int, dir int) (nextRow, nextCol, nextDir int) {
	nextRow = row + directions[dir][0]
	nextCol = col + directions[dir][1]
	if nextRow >= n || nextRow < 0 || nextCol >= n || nextCol < 0  || metrix[nextRow][nextCol] > 0{
		dir = (dir + 1) % len(directions)
	}
	return row + directions[dir][0], col + directions[dir][1], dir
}
