package p5433

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestKthFactor(t *testing.T) {
	testTable := []struct {
		name   string
		n      int
		k      int
		result int
	}{
		{
			name:   "case 1",
			n:      12,
			k:      3,
			result: 3,
		},
		{
			name:   "case 2",
			n:      7,
			k:      2,
			result: 7,
		},
		{
			name:   "case 3",
			n:      4,
			k:      4,
			result: -1,
		},
		{
			name:   "case 4",
			n:      1,
			k:      1,
			result: 1,
		},
		{
			name:   "case 5",
			n:      1000,
			k:      3,
			result: 4,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := kthFactor(c.n, c.k)
			assert.Equal(t, c.result, result)
		})
	}
}
