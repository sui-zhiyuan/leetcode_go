package l0108

import (
	"github.com/sui-zhiyuan/leetcode_go/golang/common"
)

// TreeNode ...
type TreeNode = common.TreeNode

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
