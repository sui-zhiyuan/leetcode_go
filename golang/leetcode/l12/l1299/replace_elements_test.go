package l1299

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestReplaceElement(t *testing.T) {
	testTable := []struct {
		name   string
		arr    []int
		result []int
	}{
		{
			name:   "case 1",
			arr:    []int{17, 18, 5, 4, 6, 1},
			result: []int{18, 6, 6, 6, 1, -1},
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := replaceElements(c.arr)
			assert.Equal(t, c.result, result)
		})
	}
}
