package p0032

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLongestValidParentheses(t *testing.T) {
	testTable := []struct {
		name   string
		s      string
		result int
	}{
		{
			name:   "case 1",
			s:      "(()",
			result: 2,
		},
		{
			name:   "case 2",
			s:      ")()())",
			result: 4,
		},
		{
			name:   "case 3",
			s:      "((()))",
			result: 6,
		},
		{
			name:   "case 4",
			s:      "((((()))",
			result: 6,
		},
		{
			name:   "case 5",
			s:      "((()))))",
			result: 6,
		},
		{
			name:   "case 6",
			s:      ")(((((()())()()))()(()))(",
			result: 22,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := longestValidParentheses(c.s)
			assert.Equal(t, c.result, result)
		})
	}
}
