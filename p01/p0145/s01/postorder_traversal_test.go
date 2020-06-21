package s01

import (
	"testing"

	"github.com/sui-zhiyuan/leetcode_go/define"

	"github.com/stretchr/testify/assert"
)

func TestPostorderTraversal(t *testing.T) {
	testTable := []struct {
		name string
		tree *TreeNode
		list []int
	}{
		{
			name: "case 1",
			tree: define.NewTree([]int{1, define.Null, 2, define.Null, define.Null, 3}),
			list: []int{3, 2, 1},
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			list := postorderTraversal(c.tree)
			assert.Equal(t, c.list, list)
		})
	}
}
