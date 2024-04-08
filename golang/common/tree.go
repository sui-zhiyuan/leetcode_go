package common

import (
	"fmt"
	"math"
	"strings"
)

// TreeNode ...
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// Null ,  null while creating tree
const Null = math.MinInt32

// NewTree ...
func NewTree(values []int) *TreeNode {
	nodes := make([]*TreeNode, len(values))
	for i, v := range values {
		if v == Null {
			continue
		}
		nodes[i] = &TreeNode{
			Val: v,
		}

		if i == 0 {
			continue
		}

		if i%2 == 1 {
			nodes[(i-1)/2].Left = nodes[i]
		} else {
			nodes[(i-1)/2].Right = nodes[i]
		}
	}
	return nodes[0]
}

// PrintTree ...
func PrintTree(root *TreeNode) string {
	sb := &strings.Builder{}
	sb.WriteString("[")
	printTree(root, sb)
	sb.WriteString("]")
	return sb.String()
}

func printTree(root *TreeNode, sb *strings.Builder) {
	if root == nil {
		sb.WriteString("nil,")
		return
	}
	sb.WriteString(fmt.Sprintf("%d,", root.Val))
	printTree(root.Left, sb)
	printTree(root.Right, sb)
}
