package l0104

import (
	"github.com/sui-zhiyuan/leetcode_go/golang/common"
)

// TreeNode ...
type TreeNode = common.TreeNode

func maxDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}
	return max(maxDepth(root.Left), maxDepth(root.Right)) + 1
}

func max(t ...int) int {
	result := t[0]
	for _, v := range t[1:] {
		if result < v {
			result = v
		}
	}
	return result
}
