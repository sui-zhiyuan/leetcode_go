package l5451

import (
	"math"
)

func findMaxValueOfEquation(points [][]int, k int) int {
	max := math.MinInt32
	h := newHeap(len(points))
	for _, point := range points {
		curr := coordinate{
			x: point[0],
			y: point[1],
		}
		v, ok := h.peak()
		for ; ok && v.value.x < curr.x-k; v, ok = h.peak() {
			h.extract()
		}

		if ok && v.rank+curr.x+curr.y > max {
			max = v.rank + curr.x + curr.y
		}

		h.insert(headNode{
			value: curr,
			rank:  curr.y - curr.x,
		})

	}
	return max
}

type coordinate struct {
	x int
	y int
}

type heap struct {
	values []headNode
}

type headNode struct {
	value coordinate
	rank  int
}

func newHeap(cap int) *heap {
	return &heap{
		values: make([]headNode, 0, cap),
	}
}

func (h *heap) insert(v headNode) {
	h.values = append(h.values, v)
	curr := len(h.values) - 1
	for curr > 0 {
		para := (curr+1)/2 - 1
		if h.values[para].rank < h.values[curr].rank {
			h.values[para], h.values[curr] = h.values[curr], h.values[para]
		}
		curr = para
	}
	// fmt.Println("insert", h.values)
}

func (h *heap) peak() (v headNode, ok bool) {
	if len(h.values) == 0 {
		return headNode{}, false
	}
	return h.values[0], true
}

func (h *heap) extract() (v headNode, ok bool) {
	if len(h.values) == 0 {
		return headNode{}, false
	}

	result := h.values[0]

	length := len(h.values) - 1
	h.values[0] = h.values[length]
	h.values = h.values[:length]

	for curr := 0; curr < length; {
		max := h.values[curr].rank
		maxL := curr
		for _, child := range []int{curr*2 + 1, curr*2 + 2} {
			if child < len(h.values) && max < h.values[child].rank {
				max = h.values[child].rank
				maxL = child
			}
		}
		if maxL == curr {
			break
		}
		h.values[maxL], h.values[curr] = h.values[curr], h.values[maxL]
		curr = maxL
	}

	// fmt.Println("extract", h.values)
	return result, true
}
