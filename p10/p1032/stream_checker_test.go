package p1032

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSreamChecker(t *testing.T) {
	testTable := []struct {
		name    string
		words   []string
		testing string
		result  []bool
	}{
		{
			name:    "case 1",
			words:   []string{"cd", "f", "kl"},
			testing: "abcdefghijkl",
			result:  []bool{false, false, false, true, false, true, false, false, false, false, false, true},
		},
		{
			name:    "case 2",
			words:   []string{"abc", "bc", "c"},
			testing: "abcdefghijkl",
			result:  []bool{false, false, true, false, false, false, false, false, false, false, false, false},
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			sc := Constructor(c.words)
			for i := 0; i < len(c.testing); i++ {
				result := sc.Query(c.testing[i])
				assert.Equal(t, c.result[i], result, string(c.testing[i]))
			}
		})
	}
}
