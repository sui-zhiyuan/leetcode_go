package pi1716

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMessage(t *testing.T) {
	testTable := []struct {
		name   string
		nums   []int
		result int
	}{
		{
			name:   "case 1",
			nums:   []int{1, 2, 3, 1},
			result: 4,
		},
		{
			name:   "case 2",
			nums:   []int{2, 7, 9, 3, 1},
			result: 12,
		},
		{
			name:   "case 3",
			nums:   []int{2, 1, 4, 5, 3, 1, 1, 3},
			result: 12,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := massage(c.nums)
			assert.Equal(t, c.result, result)
		})
	}
}
