package l0098

import (
	"github.com/sui-zhiyuan/leetcode_go/golang/common"
)

// TreeNode ...
type TreeNode = common.TreeNode

//lint:file-ignore U1000 leetcode
func isValidBST(root *TreeNode) bool {
	return isBST(root, nil, nil)
}

func isBST(root *TreeNode, min, max *int) (ok bool) {
	if root == nil {
		return true
	}

	if min != nil && root.Val <= *min {
		return false
	}

	if max != nil && root.Val >= *max {
		return false
	}

	return isBST(root.Left, min, &root.Val) && isBST(root.Right, &root.Val, max)
}
