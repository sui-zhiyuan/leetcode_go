package li1717

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMultiSearch(t *testing.T) {
	testTable := []struct {
		name   string
		big    string
		smalls []string
		match  [][]int
	}{
		// cSpell:ignore ssippi
		{
			name:   "case 1",
			big:    "mississippi",
			smalls: []string{"is", "ppi", "hi", "sis", "i", "ssippi"},
			match:  [][]int{{1, 4}, {8}, {}, {3}, {1, 4, 7, 10}, {5}},
		},
		// cSpell:ignore abcabcabc
		{
			name:   "case 2",
			big:    "abcabcabc",
			smalls: []string{"abc", "bc", "c"},
			match:  [][]int{{0, 3, 6}, {1, 4, 7}, {2, 5, 8}},
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			match := multiSearch(c.big, c.smalls)
			assert.Equal(t, c.match, match)
		})
	}
}
