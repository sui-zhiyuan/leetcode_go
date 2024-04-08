package l0400

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFindNthDigit(t *testing.T) {
	testTable := []struct {
		name  string
		n     int
		digit int
	}{
		{
			name:  "case 1",
			n:     3,
			digit: 3,
		},
		{
			name:  "case 2",
			n:     11,
			digit: 0,
		},
		{
			name:  "case 3",
			n:     100,
			digit: 5,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			digit := findNthDigit(c.n)
			assert.Equal(t, c.digit, digit)
		})
	}
}
