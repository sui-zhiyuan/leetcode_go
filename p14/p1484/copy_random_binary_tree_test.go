package p1484

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCopyRandomBinaryTree(t *testing.T) {
	testTable := []struct {
		name string
		tree *Node
	}{
		{
			name: "case 1",
			tree: newTree(
				[]int{1, Nil, 4, Nil, Nil, 7},
				[]int{Nil, Nil, 5, Nil, Nil, 0}),
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			tree := copyRandomBinaryTree(c.tree)
			assert.Equal(t, c.tree, tree)
		})
	}
}
