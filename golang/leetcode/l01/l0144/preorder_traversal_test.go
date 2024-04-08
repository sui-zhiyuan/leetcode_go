package l0144

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/sui-zhiyuan/leetcode_go/golang/common"
)

//cspell:ignore preorder, Trversal

func TestPreorderTrversal(t *testing.T) {
	testTable := []struct {
		name string
		tree *common.TreeNode
		list []int
	}{
		{
			name: "case 1",
			tree: common.NewTree([]int{1, common.Null, 2, common.Null, common.Null, 3}),
			list: []int{1, 2, 3},
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			list := preorderTraversal(c.tree)
			assert.Equal(t, c.list, list)
		})
	}
}
