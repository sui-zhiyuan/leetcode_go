package p0271

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestEncodeDecode(t *testing.T) {
	testTable := []struct {
		name string
		strs []string
	}{
		{
			name: "case 1",
			strs: []string{"abc", "def"},
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			var codec Codec
			strs := codec.Decode(codec.Encode(c.strs))
			assert.Equal(t, c.strs, strs)
		})
	}
}
