package p0108

import (
	"github.com/sui-zhiyuan/leetcode_go/define"
)

// TreeNode ...
type TreeNode = define.TreeNode

func sortedArrayToBST(nums []int) *TreeNode {
	length := len(nums)
	if length == 0 {
		return nil
	}
	mid := length / 2
	return &TreeNode{
		Val:   nums[mid],
		Left:  sortedArrayToBST(nums[:mid]),
		Right: sortedArrayToBST(nums[mid+1:]),
	}
}
