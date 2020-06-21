package s01

import (
	"github.com/sui-zhiyuan/leetcode_go/define"
)

// TreeNode ...
type TreeNode = define.TreeNode

func postorderTraversal(root *TreeNode) []int {
	traversal := []int{}

	if root == nil {
		return traversal
	}

	traversal = append(traversal, postorderTraversal(root.Left)...)
	traversal = append(traversal, postorderTraversal(root.Right)...)
	traversal = append(traversal, root.Val)

	return traversal
}
