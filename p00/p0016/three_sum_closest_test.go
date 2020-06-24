package p0016

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestThreeSumClosest(t *testing.T) {
	testTable := []struct {
		name   string
		nums   []int
		target int
		result int
	}{
		{
			name:   "case 1",
			nums:   []int{-1, 2, 1, -4},
			target: 1,
			result: 2,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := threeSumClosest(c.nums, c.target)
			assert.Equal(t, c.result, result)
		})
	}
}
