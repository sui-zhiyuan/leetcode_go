package p0124

import (
	"testing"

	"github.com/sui-zhiyuan/leetcode_go/define"

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
			tree: define.NewTree([]int{1, 2, 3}),
			path: 6,
		},
		{
			name: "case 2",
			tree: define.NewTree([]int{-10, 9, 20, define.Null, define.Null, 15, 7}),
			path: 42,
		},
		{
			name: "case 3",
			tree: define.NewTree([]int{-3}),
			path: -3,
		},
		{
			name: "case 4",
			tree: define.NewTree([]int{2, -1}),
			path: 2,
		},
		{
			name: "case 5",
			tree: define.NewTree([]int{-1, -2, -3}),
			path: -1,
		},
		{
			name: "case 6",
			tree: define.NewTree([]int{1, -2, 3}),
			path: 4,
		},
		{
			name: "case 7",
			tree: define.NewTree([]int{5, 4, 8, 11, define.Null, 13, 4, 7, 2, define.Null, define.Null, define.Null, define.Null, define.Null, 1}),
			path: 48,
		},
		{
			name: "case 8",
			tree: define.NewTree([]int{9, -2, define.Null, -3}),
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
