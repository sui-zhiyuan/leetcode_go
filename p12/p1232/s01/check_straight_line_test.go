package s01

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCheckStraightLine(t *testing.T) {
	testTable := []struct {
		name       string
		coordinate [][]int
		ok         bool
	}{
		{
			name:       "case 1",
			coordinate: [][]int{{1, 2}, {2, 3}, {3, 4}, {4, 5}, {5, 6}, {6, 7}},
			ok:         true,
		},
		{
			name:       "case 2",
			coordinate: [][]int{{1, 1}, {2, 2}, {3, 4}, {4, 5}, {5, 6}, {7, 7}},
			ok:         false,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			ok := checkStraightLine(c.coordinate)
			assert.Equal(t, c.ok, ok)
		})
	}
}
