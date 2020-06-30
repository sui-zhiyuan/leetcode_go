package p5450

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNumSubseq(t *testing.T) {
	testTable := []struct {
		name   string
		nums   []int
		target int
		count  int
	}{
		{
			name:   "case 1",
			nums:   []int{3, 5, 6, 7},
			target: 9,
			count:  4,
		},
		{
			name:   "case 2",
			nums:   []int{3, 3, 6, 8},
			target: 10,
			count:  6,
		},
		{
			name:   "case 3",
			nums:   []int{2, 3, 3, 4, 6, 7},
			target: 12,
			count:  61,
		},
		{
			name:   "case 4",
			nums:   []int{5, 2, 4, 1, 7, 6, 8},
			target: 16,
			count:  127,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			count := numSubseq(c.nums, c.target)
			assert.Equal(t, c.count, count)
		})
	}
}
