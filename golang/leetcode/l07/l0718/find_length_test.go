package l0718

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFindLength(t *testing.T) {
	testTable := []struct {
		name   string
		A      []int
		B      []int
		length int
	}{
		{
			name:   "case 1",
			A:      []int{1, 2, 3, 2, 1},
			B:      []int{3, 2, 1, 4, 7},
			length: 3,
		},
		{
			name:   "case 2",
			A:      []int{0, 1, 1, 1, 1},
			B:      []int{1, 0, 1, 0, 1},
			length: 2,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			length := findLength(c.A, c.B)
			assert.Equal(t, c.length, length)
		})
	}
}
