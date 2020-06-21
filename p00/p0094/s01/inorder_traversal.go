package s1

import (
	"github.com/sui-zhiyuan/leetcode_go/define"
)

// TreeNode ...
type TreeNode = define.TreeNode

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
