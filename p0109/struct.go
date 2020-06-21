package p0109

import (
	"math"
)

// ListNode ...
type ListNode struct {
	Val  int
	Next *ListNode
}

// TreeNode ...
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// NewList ...
func NewList(values []int) *ListNode {
	head := &ListNode{}
	curr := head

	for _, v := range values {
		next := &ListNode{
			Val: v,
		}
		curr.Next = next
		curr = next
	}
	return head.Next
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
