package p0986

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestIntervalIntersection(t *testing.T) {
	testTable := []struct {
		name   string
		A      [][]int
		B      [][]int
		result [][]int
	}{
		{
			name:   "case 1",
			A:      [][]int{{0, 2}, {5, 10}, {13, 23}, {24, 25}},
			B:      [][]int{{1, 5}, {8, 12}, {15, 24}, {25, 26}},
			result: [][]int{{1, 2}, {5, 5}, {8, 10}, {15, 23}, {24, 24}, {25, 25}},
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := intervalIntersection(c.A, c.B)
			assert.Equal(t, c.result, result)
		})
	}
}
