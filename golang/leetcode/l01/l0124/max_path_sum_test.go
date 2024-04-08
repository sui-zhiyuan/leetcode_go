package l0124

import (
	"testing"

	"github.com/sui-zhiyuan/leetcode_go/golang/common"

	"github.com/stretchr/testify/assert"
)

func TestMaxPathSum(t *testing.T) {
	testTable := []struct {
		name string
		tree *TreeNode
		path int
	}{
		{
			name: "case 1",
			tree: common.NewTree([]int{1, 2, 3}),
			path: 6,
		},
		{
			name: "case 2",
			tree: common.NewTree([]int{-10, 9, 20, common.Null, common.Null, 15, 7}),
			path: 42,
		},
		{
			name: "case 3",
			tree: common.NewTree([]int{-3}),
			path: -3,
		},
		{
			name: "case 4",
			tree: common.NewTree([]int{2, -1}),
			path: 2,
		},
		{
			name: "case 5",
			tree: common.NewTree([]int{-1, -2, -3}),
			path: -1,
		},
		{
			name: "case 6",
			tree: common.NewTree([]int{1, -2, 3}),
			path: 4,
		},
		{
			name: "case 7",
			tree: common.NewTree([]int{5, 4, 8, 11, common.Null, 13, 4, 7, 2, common.Null, common.Null, common.Null, common.Null, common.Null, 1}),
			path: 48,
		},
		{
			name: "case 8",
			tree: common.NewTree([]int{9, -2, common.Null, -3}),
			path: 9,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			path := maxPathSum(c.tree)
			assert.Equal(t, c.path, path)
		})
	}
}
