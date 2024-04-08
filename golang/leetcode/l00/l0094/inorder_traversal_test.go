package l0094

import (
	"testing"

	"github.com/sui-zhiyuan/leetcode_go/golang/common"

	"github.com/stretchr/testify/assert"
)

// cspell:ignore inorder

func TestInOrderTraversal(t *testing.T) {
	testTable := []struct {
		name string
		tree *TreeNode
		list []int
	}{
		{
			name: "case 1",
			tree: common.NewTree([]int{1, common.Null, 2, common.Null, common.Null, 3}),
			list: []int{1, 3, 2},
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			list := inorderTraversal(c.tree)
			assert.Equal(t, c.list, list)
		})
	}
}
