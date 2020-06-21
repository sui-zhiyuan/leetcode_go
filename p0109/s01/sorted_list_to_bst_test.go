package s01

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/sui-zhiyuan/leetcode_go/define"
)

func TestSortedListToBST(t *testing.T) {
	testTable := []struct {
		name string
		list *ListNode
		tree *TreeNode
	}{
		{
			name: "case 1",
			list: define.NewList([]int{-10, -3, 0, 5, 9}),
			tree: define.NewTree([]int{0, -3, 9, -10, define.Null, 5}),
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			tree := sortedListToBST(c.list)
			assert.Equal(t, c.tree, tree)
		})
	}
}
