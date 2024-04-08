package l0139

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestWordBreak(t *testing.T) {
	testTable := []struct {
		name     string
		s        string
		wordDict []string
		ok       bool
	}{
		{
			name:     "case 1",
			s:        "leetcode",
			wordDict: []string{"leet", "code"},
			ok:       true,
		},
		{
			name:     "case 2",
			s:        "applepenapple",
			wordDict: []string{"apple", "pen"},
			ok:       true,
		},
		{
			name:     "case 3",
			s:        "catsandog",
			wordDict: []string{"cats", "dog", "sand", "and", "cat"},
			ok:       false,
		},
		{
			name:     "case 4",
			s:        "abababccabc",
			wordDict: []string{"abc", "bc", "c"},
			ok:       false,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			ok := wordBreak(c.s, c.wordDict)
			assert.Equal(t, c.ok, ok)
		})
	}
}
