package s01

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_generateMatrix(t *testing.T) {
	testTable := []struct {
		name   string
		n      int
		matrix [][]int
	}{
		{
			name: "case 1",
			n:    3,
			matrix: [][]int{
				{1, 2, 3},
				{8, 9, 4},
				{7, 6, 5},
			},
		},
		{
			name: "case 2",
			n:    4,
			matrix: [][]int{
				{1, 2, 3, 4},
				{12, 13, 14, 5},
				{11, 16, 15, 6},
				{10, 9, 8, 7},
			},
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			matrix := generateMatrix(c.n)
			assert.Equal(t, c.matrix, matrix)
		})
	}
}
