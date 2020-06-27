package pi0806

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestHanota(t *testing.T) {
	testTable := []struct {
		name   string
		plates []int
	}{
		{
			name:   "case 1",
			plates: []int{3, 2, 1},
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			start := make([]int, len(c.plates))
			copy(start, c.plates)
			plates := hanota(start, []int{}, []int{})
			assert.Equal(t, c.plates, plates)
		})
	}
}
