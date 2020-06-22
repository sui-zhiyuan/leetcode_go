package p0124

import (
	"math"

	"github.com/sui-zhiyuan/leetcode_go/define"
)

// TreeNode ...
type TreeNode = define.TreeNode

func maxPathSum(root *TreeNode) int {
	_, maxDouble := caculate(root)
	return maxDouble
}

func caculate(root *TreeNode) (maxSingle, maxDouble int) {
	if root.Left == nil && root.Right == nil {
		return max(0, root.Val), root.Val
	}

	if root.Right == nil {
		maxLeftSingle, maxLeftDouble := caculate(root.Left)
		return max(0, root.Val+maxLeftSingle), max(root.Val+maxLeftSingle, maxLeftDouble)
	}

	if root.Left == nil {
		maxRightSingle, maxRightDouble := caculate(root.Right)
		return max(0, root.Val+maxRightSingle), max(root.Val+maxRightSingle, maxRightDouble)
	}

	maxLeftSingle, maxLeftDouble := caculate(root.Left)
	maxRightSingle, maxRightDouble := caculate(root.Right)
	maxSingle = max(0, root.Val+maxLeftSingle, root.Val+maxRightSingle)
	maxDouble = max(maxLeftDouble, maxRightDouble, root.Val+maxLeftSingle+maxRightSingle)
	return maxSingle, maxDouble
}

func max(s ...int) int {
	r := math.MinInt32
	for _, v := range s {
		if r < v {
			r = v
		}
	}
	return r
}
