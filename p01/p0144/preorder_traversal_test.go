package p0144

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/sui-zhiyuan/leetcode_go/define"
)

func TestPreorderTrversal(t *testing.T) {
	testTable := []struct {
		name string
		tree *define.TreeNode
		list []int
	}{
		{
			name: "case 1",
			tree: define.NewTree([]int{1, define.Null, 2, define.Null, define.Null, 3}),
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
