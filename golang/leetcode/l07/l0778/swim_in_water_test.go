package l0778

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSwimInWater(t *testing.T) {
	testTable := []struct {
		name   string
		grid   [][]int
		result int
	}{
		{
			name:   "case 1",
			grid:   [][]int{{0, 2}, {1, 3}},
			result: 3,
		},
		{
			name:   "case 2",
			grid:   [][]int{{0, 1, 2, 3, 4}, {24, 23, 22, 21, 5}, {12, 13, 14, 15, 16}, {11, 17, 18, 19, 20}, {10, 9, 8, 7, 6}},
			result: 16,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := swimInWater(c.grid)
			assert.Equal(t, c.result, result)
		})
	}
}
