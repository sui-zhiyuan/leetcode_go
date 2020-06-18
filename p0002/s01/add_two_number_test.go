package s01

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/sui-zhiyuan/leetcode_go/p0002"
)

func Test_addTwoNumbers(t *testing.T) {
	testTable := []struct {
		name   string
		l1     *ListNode
		l2     *ListNode
		result *ListNode
	}{
		{
			name:   "case 1",
			l1:     p0002.NewListNode(342),
			l2:     p0002.NewListNode(465),
			result: p0002.NewListNode(807),
		},
		{
			name:   "case 2",
			l1:     p0002.NewListNode(81),
			l2:     p0002.NewListNode(0),
			result: p0002.NewListNode(81),
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := addTwoNumbers(c.l1, c.l2)
			assert.Equal(t, c.result, result)
		})
	}
}
