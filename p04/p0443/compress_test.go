package p0443

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCompress(t *testing.T) {
	testTable := []struct {
		name        string
		chars       []byte
		length      int
		charsResult []byte
	}{
		{
			name:        "case 1",
			chars:       []byte{'a', 'a', 'b', 'b', 'c', 'c', 'c'},
			length:      6,
			charsResult: []byte{'a', '2', 'b', '2', 'c', '3'},
		},
		{
			name:        "case 2",
			chars:       []byte{'a'},
			length:      1,
			charsResult: []byte{'a'},
		},
		{
			name:        "case 3",
			chars:       []byte{'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'},
			length:      4,
			charsResult: []byte{'a', 'b', '1', '2'},
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			chars := make([]byte, len(c.chars))
			copy(chars, c.chars)
			length := compress(chars)
			assert.Equal(t, c.length, length)
			assert.Equal(t, c.charsResult, chars[:length])
		})
	}
}
