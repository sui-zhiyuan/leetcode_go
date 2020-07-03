package p0108

import (
	"fmt"
	"testing"

	"github.com/sui-zhiyuan/leetcode_go/define"

	"github.com/stretchr/testify/assert"
)

func TestSortedArrayToBst(t *testing.T) {
	testTable := []struct {
		name     string
		nums     []int
		treeRoot *TreeNode
	}{
		{
			name:     "case 1",
			nums:     []int{-10, -3, 0, 5, 9},
			treeRoot: define.NewTree([]int{0, -3, 9, -10, define.Null, 5}),
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			treeRoot := sortedArrayToBST(c.nums)
			assert.Equal(t, c.treeRoot, treeRoot)
			fmt.Println(define.PrintTree(c.treeRoot))
			fmt.Println(define.PrintTree(treeRoot))
		})
	}
}
