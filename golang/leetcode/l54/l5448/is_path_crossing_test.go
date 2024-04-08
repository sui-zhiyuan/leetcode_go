package l5448

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestIsPathCrossing(t *testing.T) {
	testTable := []struct {
		name  string
		path  string
		cross bool
	}{
		//cspell:disable
		{
			name:  "case 1",
			path:  "NES",
			cross: false,
		},
		{
			name:  "case 2",
			path:  "NESWW",
			cross: true,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			cross := isPathCrossing(c.path)
			assert.Equal(t, c.cross, cross)
		})
	}
}
