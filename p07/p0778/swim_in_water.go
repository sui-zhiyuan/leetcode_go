package p0778

func swimInWater(grid [][]int) int {
	n := len(grid)
	reach := make([][]int, n)
	for i := range reach {
		reach[i] = make([]int, n)
		for j := range reach[i] {
			reach[i][j] = -1
		}
	}

	q := &queue{}
	reach[0][0] = grid[0][0]
	q.enqueue(square{0, 0})
	directions := []square{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

	for {
		curr, ok := q.dequeue()
		if !ok {
			break
		}

		for _, d := range directions {
			next := square{
				curr.row + d.row,
				curr.col + d.col,
			}

			if next.row < 0 || next.row >= n || next.col < 0 || next.col >= n {
				continue
			}
			level := reach[curr.row][curr.col]
			if level < grid[next.row][next.col] {
				level = grid[next.row][next.col]
			}

			if reach[next.row][next.col] < 0 || reach[next.row][next.col] > level {
				reach[next.row][next.col] = level
				q.enqueue(next)
			}
		}
	}
	return reach[n-1][n-1]
}

type square struct {
	row int
	col int
}

type queue struct {
	head *node
	tail *node
}

type node struct {
	value square
	next  *node
}

func (q *queue) enqueue(v square) {
	if q.head == nil {
		q.head = &node{
			value: v,
		}
		q.tail = q.head
		return
	}

	next := &node{
		value: v,
	}
	q.head.next = next
	q.head = next
}

func (q *queue) dequeue() (v square, ok bool) {
	if q.tail == nil {
		return square{}, false
	}

	v = q.tail.value
	if q.tail.next == nil {
		q.head = nil
	}

	q.tail = q.tail.next
	return v, true
}
