package l0003

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_lengthOfLongestSubstring(t *testing.T) {
	testTable := []struct {
		name string
		s    string
		ans  int
	}{
		{
			name: "case 1",
			s:    "abcabcbb",
			ans:  3,
		},
		{
			name: "case 2",
			s:    "bbbbb",
			ans:  1,
		},
		{
			name: "case 3",
			s:    "pwwkew",
			ans:  3,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := lengthOfLongestSubstring(c.s)
			assert.Equal(t, c.ans, result)
		})
	}
}
