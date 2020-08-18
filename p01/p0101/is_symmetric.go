package p0101

import (
	"github.com/sui-zhiyuan/leetcode_go/define"
)

// TreeNode ...
type TreeNode = define.TreeNode

func isSymmetric(root *TreeNode) bool {
	if root == nil {
		return true
	}

	return checkSymmetric(root.Left, root.Right)
}

func checkSymmetric(left *TreeNode, right *TreeNode) bool {
	if left == nil && right == nil {
		return true
	}
	if left == nil || right == nil {
		return false
	}

	if left.Val != right.Val {
		return false
	}

	if !checkSymmetric(left.Left, right.Right) {
		return false
	}
	if !checkSymmetric(left.Right, right.Left) {
		return false
	}
	return true
}
