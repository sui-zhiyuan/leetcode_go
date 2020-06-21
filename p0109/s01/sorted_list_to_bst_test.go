package s01

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/sui-zhiyuan/leetcode_go/p0109"
)

func TestSortedListToBST(t *testing.T) {
	testTable := []struct {
		name string
		list *ListNode
		tree *TreeNode
	}{
		{
			name: "case 1",
			list: p0109.NewList([]int{-10, -3, 0, 5, 9}),
			tree: p0109.NewTree([]int{0, -3, 9, -10, p0109.Null, 5}),
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			tree := sortedListToBST(c.list)
			assert.Equal(t, c.tree, tree)
		})
	}
}
