package l1267

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCountServers(t *testing.T) {
	testTable := []struct {
		name  string
		grid  [][]int
		count int
	}{
		{
			name:  "case 1",
			grid:  [][]int{{1, 0}, {0, 1}},
			count: 0,
		},
		{
			name:  "case 2",
			grid:  [][]int{{1, 0}, {1, 1}},
			count: 3,
		},
		{
			name:  "case 3",
			grid:  [][]int{{1, 1, 0, 0}, {0, 0, 1, 0}, {0, 0, 1, 0}, {0, 0, 0, 1}},
			count: 4,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			count := countServers(c.grid)
			assert.Equal(t, c.count, count)
		})
	}
}
