package p0001_two_sum

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func Test_two_sum(t *testing.T) {
	testTable := []struct {
		name   string
		nums   []int
		target int
	}{
		{
			name:   "case 1",
			nums:   []int{2, 7, 11, 15},
			target: 9,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := twoSum(c.nums, c.target)
			assert.Equal(t, c.target, c.nums[result[0]]+c.nums[result[1]])
		})
	}
}
