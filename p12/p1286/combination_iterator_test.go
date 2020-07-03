package p1286

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCombinationIterator(t *testing.T) {
	testTable := []struct {
		name      string
		character string
		length    int
		combine   []string
	}{
		{
			name:      "case 1",
			character: "abc",
			length:    2,
			combine:   []string{"ab", "ac", "bc"},
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			iterator := Constructor(c.character, c.length)
			combine := []string{}
			for ok := iterator.HasNext(); ok; ok = iterator.HasNext() {
				v := iterator.Next()
				combine = append(combine, v)
			}
			assert.Equal(t, c.combine, combine)
		})
	}
}
