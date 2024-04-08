package lo09

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCQueue(t *testing.T) {
	testTable := []struct {
		name     string
		elements []int
	}{
		{
			name:     "case 1",
			elements: []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10},
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			queue := Constructor()
			for _, v := range c.elements {
				queue.AppendTail(v)
			}
			elements := []int{}
			for v := queue.DeleteHead(); v != -1; v = queue.DeleteHead() {
				elements = append(elements, v)
			}
			assert.Equal(t, c.elements, elements)
		})
	}
}
