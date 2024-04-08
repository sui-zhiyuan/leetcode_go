package l1020

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNumEncalves(t *testing.T) {
	testTable := []struct {
		name string
		area [][]int
		num  int
	}{
		{
			name: "case 1",
			area: [][]int{{0, 0, 0, 0}, {1, 0, 1, 0}, {0, 1, 1, 0}, {0, 0, 0, 0}},
			num:  3,
		},
		{
			name: "case 2",
			area: [][]int{
				{0, 1, 1, 0}, 
				{0, 0, 1, 0},
				{0, 0, 1, 0}, 
				{0, 0, 0, 0}},
			num:  0,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			num := numEnclaves(c.area)
			assert.Equal(t, c.num, num)
		})
	}
}
