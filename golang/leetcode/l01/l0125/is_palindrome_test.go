package l0125

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_isPalindrome(t *testing.T) {
	testTable := []struct {
		name string
		s    string
		ans  bool
	}{
		{
			name: "case 1",
			s:    "A man, a plan, a canal: Panama",
			ans:  true,
		},
		{
			name: "case 2",
			s:    "race a car",
			ans:  false,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := isPalindrome(c.s)
			assert.Equal(t, c.ans, result)
		})
	}
}
