package p0206

import (
	"github.com/sui-zhiyuan/leetcode_go/define"
)

// ListNode ...
type ListNode = define.ListNode

func reverseList(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}
	newHead, _ := revert(head)
	return newHead
}

func revert(head *ListNode) (newHead *ListNode, newTail *ListNode) {
	if head.Next == nil {
		return head, head
	}

	newHead, newTail = revert(head.Next)
	head.Next = nil
	newTail.Next = head
	return newHead, head
}
