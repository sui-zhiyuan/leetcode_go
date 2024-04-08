package l5449

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCanArrange(t *testing.T) {
	testTable := []struct {
		name string
		arr  []int
		k    int
		can  bool
	}{
		{
			name: "case 1",
			arr:  []int{1, 2, 3, 4, 5, 10, 6, 7, 8, 9},
			k:    5,
			can:  true,
		},
		{
			name: "case 2",
			arr:  []int{1, 2, 3, 4, 5, 6},
			k:    7,
			can:  true,
		},
		{
			name: "case 3",
			arr:  []int{1, 2, 3, 4, 5, 6},
			k:    10,
			can:  false,
		},
		{
			name: "case 4",
			arr:  []int{-10, 10},
			k:    2,
			can:  true,
		},
		{
			name: "case 5",
			arr:  []int{-1, 1, -2, 2, -3, 3, -4, 4},
			k:    3,
			can:  true,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			can := canArrange(c.arr, c.k)
			assert.Equal(t, c.can, can)
		})
	}
}
