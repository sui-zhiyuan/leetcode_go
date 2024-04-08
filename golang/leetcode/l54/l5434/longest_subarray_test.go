package l5434

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

//cspell:ignore subarray

func TestLongestSubarray(t *testing.T) {
	testTable := []struct {
		name string
		nums []int
		ans  int
	}{
		{
			name: "case 1",
			nums: []int{1, 1, 0, 1},
			ans:  3,
		},
		{
			name: "case 2",
			nums: []int{0, 1, 1, 1, 0, 1, 1, 0, 1},
			ans:  5,
		},
		{
			name: "case 3",
			nums: []int{1, 1, 1},
			ans:  2,
		},
		{
			name: "case 4",
			nums: []int{1, 1, 0, 0, 1, 1, 1, 0, 1},
			ans:  4,
		},
		{
			name: "case 5",
			nums: []int{0, 0, 0},
			ans:  0,
		},
		{
			name: "case 6",
			nums: []int{},
			ans:  0,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			ans := longestSubarray(c.nums)
			assert.Equal(t, c.ans, ans)
		})
	}
}
