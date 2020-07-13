package p5211

func maxProbability(n int, edges [][]int, succProb []float64, start int, end int) float64 {
	max := make([]float64, n)

	q := &queueInt{}
	max[start] = 1
	q.enqueue(start)

	edgeP := make([][]edge, n)
	for i, v := range edges {
		edgeP[v[0]] = append(edgeP[v[0]], edge{
			target: v[1],
			prob:   succProb[i],
		})
		edgeP[v[1]] = append(edgeP[v[1]], edge{
			target: v[0],
			prob:   succProb[i],
		})
	}

	for curr, ok := q.dequeue(); ok; curr, ok = q.dequeue() {
		for _, v := range edgeP[curr] {
			updateProb(max, q, v.target, max[curr]*v.prob)
		}
	}

	return max[end]
}

type edge struct {
	target int
	prob   float64
}

func updateProb(max []float64, q *queueInt, next int, prob float64) {
	if max[next] >= prob {
		return
	}

	max[next] = prob
	q.enqueue(next)
}

type queueInt struct {
	head  int
	tail  int
	array []int
}

func (q *queueInt) enqueue(v int) {
	length := len(q.array)
	if length == 0 {
		length = 1
	}
	if (q.head+1)%length == q.tail {
		newArray := make([]int, length*2+1)
		newHead := 0
		for tail := q.tail; tail != q.head; tail = (tail + 1) % length {
			newArray[newHead] = q.array[tail]
			newHead++
		}

		q.head = newHead
		q.tail = 0
		q.array = newArray
		length = len(newArray)
	}
	q.array[q.head] = v
	q.head = (q.head + 1) % length
	return
}

func (q *queueInt) dequeue() (v int, ok bool) {
	if q.tail == q.head {
		return 0, false
	}

	v = q.array[q.tail]
	q.tail = (q.tail + 1) % len(q.array)
	return v, true
}
