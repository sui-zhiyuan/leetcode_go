package p1347

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMinSteps(t *testing.T) {
	testTable := []struct {
		name   string
		s      string
		t      string
		result int
	}{
		{
			name:   "case 1",
			s:      "bab",
			t:      "aba",
			result: 1,
		},
		{
			name:   "case 2",
			s:      "leetcode",
			t:      "practice",
			result: 5,
		},
		{
			name:   "case 3",
			s:      "anagram",
			t:      "mangaar",
			result: 0,
		},
		{
			name:   "case 4",
			s:      "xxyyzz",
			t:      "xxyyzz",
			result: 0,
		},
		{
			name:   "case 5",
			s:      "friend",
			t:      "family",
			result: 4,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := minSteps(c.s, c.t)
			assert.Equal(t, c.result, result)
		})
	}
}
