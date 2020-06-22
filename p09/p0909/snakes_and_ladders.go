package p0909

func snakesAndLadders(board [][]int) int {
	n := len(board)
	steps := make([]int, n*n)
	jump := make([]int, n*n)
	for row, colValue := range board {
		for col, v := range colValue {
			if v != -1 {
				v--
			}
			index := getCoordinate(n, coordinate{row, col})
			jump[index] = v
			steps[index] = -1
		}
	}
	q := &queue{}
	steps[0] = 0
	q.enqueue(0)
	destination := []int{1, 2, 3, 4, 5, 6}
	for {
		index, ok := q.dequeue()
		if !ok {
			break
		}
		for _, des := range destination {
			next := index + des
			if next >= n*n {
				continue
			}
			if jump[next] >= 0 {
				next = jump[next]
			}
			if steps[next] == -1 || steps[next] > steps[index]+1 {
				steps[next] = steps[index] + 1
				q.enqueue(next)
			}
		}
	}
	return steps[n*n-1]
}

type coordinate struct {
	row int
	col int
}

func getCoordinate(n int, c coordinate) int {
	group := n - c.row - 1
	index := c.col
	if group%2 == 1 {
		index = n - 1 - index
	}
	return group*n + index
}

type queue struct {
	head *node
	tail *node
}

type node struct {
	value int
	next  *node
}

func (q *queue) enqueue(v int) {
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

func (q *queue) dequeue() (v int, ok bool) {
	if q.tail == nil {
		return 0, false
	}

	v = q.tail.value
	if q.tail.next == nil {
		q.head = nil
	}

	q.tail = q.tail.next
	return v, true
}
