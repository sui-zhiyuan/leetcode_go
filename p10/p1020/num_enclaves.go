package p1020

func numEnclaves(A [][]int) int {
	q := &queue{}
	lenRow := len(A)
	lenCol := len(A[0])
	for row := range A {
		if A[row][0] == 1 {
			A[row][0] = 2
			q.enqueue(coordinate{row, 0})
		}
		if A[row][lenCol-1] == 1 {
			A[row][lenCol-1] = 2
			q.enqueue(coordinate{row, lenCol - 1})
		}
	}

	for col := range A[0] {
		if A[0][col] == 1 {
			A[0][col] = 2
			q.enqueue(coordinate{0, col})
		}
		if A[lenRow-1][col] == 1 {
			A[lenRow-1][col] = 2
			q.enqueue(coordinate{lenRow - 1, col})
		}
	}

	directions := []coordinate{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

	for {
		curr, ok := q.dequeue()
		if !ok {
			break
		}

		for _, d := range directions {
			next := coordinate{curr.row + d.row, curr.col + d.col}
			if next.row >= lenRow || next.row < 0 || next.col >= lenCol || next.col < 0 {
				continue
			}

			if A[next.row][next.col] == 1 {
				A[next.row][next.col] = 2
				q.enqueue(coordinate{next.row, next.col})
			}
		}
	}

	total := 0
	for _, rowValue := range A {
		for _, v := range rowValue {
			if v == 1 {
				total++
			}
		}
	}

	return total
}

type coordinate struct {
	row int
	col int
}

type queue struct {
	head *node
	tail *node
}

type node struct {
	value coordinate
	next  *node
}

func (q *queue) enqueue(v coordinate) {
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

func (q *queue) dequeue() (v coordinate, ok bool) {
	if q.tail == nil {
		return coordinate{}, false
	}

	v = q.tail.value
	if q.tail.next == nil {
		q.head = nil
	}

	q.tail = q.tail.next
	return v, true
}
