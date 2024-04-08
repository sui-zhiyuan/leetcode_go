package l0094

import (
	"github.com/sui-zhiyuan/leetcode_go/golang/common"
)

// cspell:ignore inorder

// TreeNode ...
type TreeNode = common.TreeNode

func inorderTraversal(root *TreeNode) []int {
	traversal := []int{}

	if root == nil {
		return traversal
	}

	traversal = append(traversal, inorderTraversal(root.Left)...)
	traversal = append(traversal, root.Val)
	traversal = append(traversal, inorderTraversal(root.Right)...)

	return traversal
}
