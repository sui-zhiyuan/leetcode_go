package p0102

import (
	"github.com/sui-zhiyuan/leetcode_go/define"
)

// TreeNode ...
type TreeNode = define.TreeNode

func levelOrder(root *TreeNode) [][]int {
	result := [][]int{}
	seek(0, root, &result)
	return result
}

func seek(level int, curr *TreeNode, result *[][]int) {
	if curr == nil {
		return
	}
	if level >= len(*result) {
		*result = append(*result, []int{})
	}

	(*result)[level] = append((*result)[level], curr.Val)
	seek(level+1, curr.Left, result)
	seek(level+1, curr.Right, result)
}
