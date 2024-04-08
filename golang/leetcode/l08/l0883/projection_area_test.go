package l0883

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestProjectionArea(t *testing.T) {
	testTable := []struct {
		name string
		grid [][]int
		area int
	}{
		{
			name: "case 1",
			grid: [][]int{{2}},
			area: 5,
		},
		{
			name: "case 2",
			grid: [][]int{{1, 2}, {3, 4}},
			area: 17,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			area := projectionArea(c.grid)
			assert.Equal(t, c.area, area)
		})
	}
}
