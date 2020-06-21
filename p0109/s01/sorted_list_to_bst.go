package s01

import (
	"github.com/sui-zhiyuan/leetcode_go/p0109"
)

// ListNode ...
type ListNode = p0109.ListNode

// TreeNode ...
type TreeNode = p0109.TreeNode

func sortedListToBST(head *ListNode) *TreeNode {
	if head == nil {
		return nil
	}
	fast, slow := head, head
	var prev *ListNode
	for fast != nil && fast.Next != nil {
		prev = slow
		slow = slow.Next
		fast = fast.Next.Next
	}

	if prev == nil {
		head = nil
	} else {
		prev.Next = nil
	}
	root := &TreeNode{
		Val:   slow.Val,
		Left:  sortedListToBST(head),
		Right: sortedListToBST(slow.Next),
	}

	return root
}
