package lo59

import (
	"strconv"
)

type KthLargest struct {
	k    int
	head *node
}

func Constructor(k int, nums []int) KthLargest {
	var result = KthLargest{k: k, head: &node{}}
	for _, num := range nums {
		_ = result.Add(num)
	}
	return result
}

func (kl *KthLargest) Add(val int) int {
	inserted := false
	for nIndex, curr := -1, kl.head; ; nIndex, curr = nIndex+1, curr.next {
		if curr == nil {
			return -1
		}
		if nIndex == kl.k-1 {
			curr.next = nil
			return curr.value
		}
		if inserted {
			continue
		}
		if curr.next == nil || curr.next.value < val {
			curr.next = &node{value: val, next: curr.next}
			inserted = true
		}
	}
}

type node struct {
	value int
	next  *node
}

// debug

func (kl *KthLargest) String() string {
	var result = ""
	for curr := kl.head.next; curr != nil; curr = curr.next {
		result += "->" + strconv.FormatInt(int64(curr.value), 10)
	}
	return result
}
