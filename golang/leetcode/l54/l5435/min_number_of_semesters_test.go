package l5435

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMinNumberOfSemesters(t *testing.T) {
	testTable := []struct {
		name         string
		n            int
		k            int
		dependencies [][]int
		result       int
	}{
		{
			name:         "case 1",
			n:            4,
			dependencies: [][]int{{2, 1}, {3, 1}, {1, 4}},
			k:            2,
			result:       3,
		},
		{
			name:         "case 2",
			n:            5,
			dependencies: [][]int{{2, 1}, {3, 1}, {4, 1}, {1, 5}},
			k:            2,
			result:       4,
		},
		{
			name:         "case 3",
			n:            11,
			dependencies: [][]int{},
			k:            2,
			result:       6,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := minNumberOfSemesters(c.n, c.dependencies, c.k)
			assert.Equal(t, c.result, result)
		})
	}
}
