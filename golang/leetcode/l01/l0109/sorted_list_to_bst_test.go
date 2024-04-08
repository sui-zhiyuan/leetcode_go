package l0109

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/sui-zhiyuan/leetcode_go/golang/common"
)

func TestSortedListToBST(t *testing.T) {
	testTable := []struct {
		name string
		list *ListNode
		tree *TreeNode
	}{
		{
			name: "case 1",
			list: common.NewList([]int{-10, -3, 0, 5, 9}),
			tree: common.NewTree([]int{0, -3, 9, -10, common.Null, 5}),
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			tree := sortedListToBST(c.list)
			assert.Equal(t, c.tree, tree)
		})
	}
}
