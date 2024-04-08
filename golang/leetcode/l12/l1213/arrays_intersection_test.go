package l1213

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestArraysIntersection(t *testing.T) {
	testTable := []struct {
		name   string
		arr1   []int
		arr2   []int
		arr3   []int
		result []int
	}{
		{
			name:   "case 1",
			arr1:   []int{1, 2, 3, 4, 5},
			arr2:   []int{1, 2, 5, 7, 9},
			arr3:   []int{1, 3, 4, 5, 8},
			result: []int{1, 5},
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := arraysIntersection(c.arr1, c.arr2, c.arr3)
			assert.Equal(t, c.result, result)
		})
	}
}
