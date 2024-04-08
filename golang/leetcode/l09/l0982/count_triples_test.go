package l0982

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCountTriplets(t *testing.T) {
	testTable := []struct {
		name  string
		A     []int
		count int
	}{
		{
			name:  "case 1",
			A:     []int{2, 1, 3},
			count: 12,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			count := countTriplets(c.A)
			assert.Equal(t, c.count, count)
		})
	}
}
