package p0007

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestReverse(t *testing.T) {
	testTable := []struct {
		name   string
		x      int
		result int
	}{
		{
			name:   "case 1",
			x:      123,
			result: 321,
		},
		{
			name:   "case 2",
			x:      -123,
			result: -321,
		},
		{
			name:   "case 3",
			x:      120,
			result: 21,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := reverse(c.x)
			assert.Equal(t, c.result, result)
		})
	}
}
