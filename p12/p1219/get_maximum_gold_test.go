package p1219

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestGetMaxiumGold(t *testing.T) {
	testTable := []struct {
		name   string
		grid   [][]int
		result int
	}{
		{
			name: "case 1",
			grid: [][]int{
				{0, 6, 0},
				{5, 8, 7},
				{0, 9, 0},
			},
			result: 24,
		},
		{
			name: "case 2",
			grid: [][]int{
				{1, 0, 7},
				{2, 0, 6},
				{3, 4, 5},
				{0, 3, 0},
				{9, 0, 20},
			},
			result: 28,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := getMaximumGold(c.grid)
			assert.Equal(t, c.result, result)
		})
	}
}
