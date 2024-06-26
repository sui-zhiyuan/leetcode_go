package l0109

import "github.com/sui-zhiyuan/leetcode_go/golang/common"

// ListNode ...
type ListNode = common.ListNode

// TreeNode ...
type TreeNode = common.TreeNode

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
