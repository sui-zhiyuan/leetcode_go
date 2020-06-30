package p5451

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFindMaxValueOfEquation(t *testing.T) {
	testTable := []struct {
		name   string
		points [][]int
		k      int
		result int
	}{
		{
			name:   "case 1",
			points: [][]int{{1, 3}, {2, 0}, {5, 10}, {6, -10}},
			k:      1,
			result: 4,
		},
		{
			name:   "case 2",
			points: [][]int{{0, 0}, {3, 0}, {9, 2}},
			k:      3,
			result: 3,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := findMaxValueOfEquation(c.points, c.k)
			assert.Equal(t, c.result, result)
		})
	}
}

func TestHeap(t *testing.T) {
	testTable := []struct {
		name   string
		input  []int
		result []int
	}{
		{
			name:   "case 1",
			input:  []int{6, 5, 4, 3, 1, 2},
			result: []int{6, 5, 4, 3, 2, 1},
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			h := newHeap(len(c.input))
			for _, v := range c.input {
				h.insert(headNode{rank: v})
			}

			result := []int{}
			for v, ok := h.extract(); ok; v, ok = h.extract() {
				result = append(result, v.rank)
			}
			assert.Equal(t, c.result, result)
		})
	}
}
