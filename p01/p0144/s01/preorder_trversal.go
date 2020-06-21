package s1

import (
	"github.com/sui-zhiyuan/leetcode_go/define"
)

// TreeNode ...
type TreeNode = define.TreeNode

func preorderTraversal(root *TreeNode) []int {
	traversal := []int{}

	if root == nil {
		return traversal
	}

	traversal = append(traversal, root.Val)
	traversal = append(traversal, preorderTraversal(root.Left)...)
	traversal = append(traversal, preorderTraversal(root.Right)...)

	return traversal
}
