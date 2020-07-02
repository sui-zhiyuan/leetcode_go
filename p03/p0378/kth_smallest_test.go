package p0378

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestKthSmallest(t *testing.T) {
	testTable := []struct {
		name   string
		matrix [][]int
		k      int
		ans    int
	}{
		{
			name: "case 1",
			matrix: [][]int{
				{1, 5, 9},
				{10, 11, 13},
				{12, 13, 15},
			},
			k:   8,
			ans: 13,
		},
		{
			name: "case 2",
			matrix: [][]int{
				{1, 3, 5},
				{6, 7, 12},
				{11, 14, 14},
			},
			k:   3,
			ans: 5,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			ans := kthSmallest(c.matrix, c.k)
			assert.Equal(t, c.ans, ans)
		})
	}
}
