package solve1

import (
	"github.com/stretchr/testify/assert"
	"testing"
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
			l1:     newListNode(342),
			l2:     newListNode(465),
			result: newListNode(807),
		},
		{
			name:   "case 2",
			l1:     newListNode(81),
			l2:     newListNode(0),
			result: newListNode(81),
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := addTwoNumbers(c.l1, c.l2)
			assert.Equal(t, c.result, result)
		})
	}
}
