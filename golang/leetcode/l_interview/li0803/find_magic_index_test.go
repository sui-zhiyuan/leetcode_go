package li0803

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFindMagicIndex(t *testing.T) {
	testTable := []struct {
		name  string
		nums  []int
		index int
	}{
		{
			name:  "case 1",
			nums:  []int{0, 2, 3, 4, 5},
			index: 0,
		},
		{
			name:  "case 2",
			nums:  []int{1, 1, 1},
			index: 1,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			index := findMagicIndex(c.nums)
			assert.Equal(t, c.index, index)
		})
	}
}
