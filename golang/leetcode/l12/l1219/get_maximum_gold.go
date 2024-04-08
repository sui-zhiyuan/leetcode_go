package l1219

// cspell:ignore ccol, ccol, nrow, ncol

func getMaximumGold(grid [][]int) int {
	max := 0
	for row, rowVal := range grid {
		for col, v := range rowVal {
			if v > 0 {
				g := findGroup(grid, row, col)

				// for i, v := range g {
				// 	fmt.Println("g", i, v)
				// }

				v := calculateSum(g)
				if v > max {
					max = v
				}
			}
		}
	}
	return max
}

func findGroup(grid [][]int, row, col int) []square {
	result := []square{}
	q := &intQueue{}
	rowCount := len(grid)
	colCount := len(grid[0])
	dic := make([]int, rowCount*colCount)
	for i := range dic {
		dic[i] = -1
	}

	dic[row*colCount+col] = 0
	q.enqueue(0)
	result = append(result, square{row: row, col: col, gold: grid[row][col]})
	grid[row][col] = 0

	for curr, ok := q.dequeue(); ok; curr, ok = q.dequeue() {
		crow := result[curr].row
		ccol := result[curr].col

		for _, d := range directions {
			nrow := crow + d.row
			ncol := ccol + d.col
			if nrow < 0 || nrow >= rowCount || ncol < 0 || ncol >= colCount {
				continue
			}
			if grid[nrow][ncol] > 0 {
				result = append(result, square{
					row:     nrow,
					col:     ncol,
					gold:    grid[nrow][ncol],
					connect: []int{},
				})
				grid[nrow][ncol] = 0
				dic[nrow*colCount+ncol] = len(result) - 1
				q.enqueue(dic[nrow*colCount+ncol])
			}
			if dic[nrow*colCount+ncol] >= 0 {
				result[curr].connect = append(result[curr].connect, dic[nrow*colCount+ncol])
			}
		}
	}

	return result
}

func calculateSum(s []square) int {
	n := len(s)
	arrived := make([]bool, n)
	max := 0
	for i := range s {
		v := maxSum(s, arrived, i)
		//fmt.Println("finish", i, v)
		if v > max {
			max = v
		}
	}

	return max
}

func maxSum(s []square, arrived []bool, curr int) int {
	arrived[curr] = true
	defer func() {
		arrived[curr] = false
	}()
	max := 0
	for _, next := range s[curr].connect {
		if !arrived[next] {
			v := maxSum(s, arrived, next)
			if v > max {
				max = v
			}
		}
	}
	return max + s[curr].gold
}

var directions = []square{{row: -1, col: 0}, {row: 0, col: 1}, {row: 1, col: 0}, {row: 0, col: -1}}

type square struct {
	row     int
	col     int
	gold    int
	connect []int
}

type intQueue struct {
	head, tail *intQueueNode
}

type intQueueNode struct {
	val  int
	next *intQueueNode
}

func (q *intQueue) enqueue(v int) {
	if q.tail == nil {
		q.tail = &intQueueNode{
			val: v,
		}
		q.head = q.tail
		return
	}

	q.head.next = &intQueueNode{
		val: v,
	}

	q.head = q.head.next
}

func (q *intQueue) dequeue() (v int, ok bool) {
	if q.tail == nil {
		return 0, false
	}

	v, ok = q.tail.val, true
	q.tail = q.tail.next
	return
}
