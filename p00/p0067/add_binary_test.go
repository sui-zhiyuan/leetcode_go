package p0067

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestAddBinary(t *testing.T) {
	testTable := []struct {
		name   string
		a      string
		b      string
		result string
	}{
		{
			name:   "case 1",
			a:      "11",
			b:      "1",
			result: "100",
		},
		{
			name:   "case 2",
			a:      "1010",
			b:      "1011",
			result: "10101",
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := addBinary(c.a, c.b)
			assert.Equal(t, c.result, result)
		})
	}
}
