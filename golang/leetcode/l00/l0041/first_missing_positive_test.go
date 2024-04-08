package l0041

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFirstMissingPositive(t *testing.T) {
	testTable := []struct {
		name   string
		nums   []int
		result int
	}{
		{
			name:   "case 1",
			nums:   []int{1, 2, 0},
			result: 3,
		},
		{
			name:   "case 2",
			nums:   []int{3, 4, -1, 1},
			result: 2,
		},
		{
			name:   "case 3",
			nums:   []int{7, 8, 9, 11, 12},
			result: 1,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := firstMissingPositive(c.nums)
			assert.Equal(t, c.result, result)
		})
	}
}
