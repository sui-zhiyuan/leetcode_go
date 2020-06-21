package define

// ListNode ...
type ListNode struct {
	Val  int
	Next *ListNode
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

// NewListByDecimal ...
func NewListByDecimal(v int) *ListNode {
	result := new(ListNode)
	for k := result; v > 0; v /= 10 {
		k.Next = new(ListNode)
		k = k.Next
		k.Val = v % 10
	}
	return result.Next
}
