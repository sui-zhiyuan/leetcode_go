package p1334

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFindTheCity(t *testing.T) {
	testTable := []struct {
		name              string
		n                 int
		edges             [][]int
		distanceThreshold int
		index             int
	}{
		{
			name:              "case 1",
			n:                 4,
			edges:             [][]int{{0, 1, 3}, {1, 2, 1}, {1, 3, 4}, {2, 3, 1}},
			distanceThreshold: 4,
			index:             3,
		},
		{
			name:              "case 2",
			n:                 5,
			edges:             [][]int{{0, 1, 2}, {0, 4, 8}, {1, 2, 3}, {1, 4, 2}, {2, 3, 1}, {3, 4, 1}},
			distanceThreshold: 2,
			index:             0,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			index := findTheCity(c.n, c.edges, c.distanceThreshold)
			assert.Equal(t, c.index, index)
		})
	}
}
