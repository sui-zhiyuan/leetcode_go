package l0145

import (
	"testing"

	"github.com/sui-zhiyuan/leetcode_go/golang/common"

	"github.com/stretchr/testify/assert"
)

//cspell:ignore postorder

func TestPostorderTraversal(t *testing.T) {
	testTable := []struct {
		name string
		tree *TreeNode
		list []int
	}{
		{
			name: "case 1",
			tree: common.NewTree([]int{1, common.Null, 2, common.Null, common.Null, 3}),
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
