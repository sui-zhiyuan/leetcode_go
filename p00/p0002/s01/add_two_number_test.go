package s01

import (
	"testing"

	"github.com/sui-zhiyuan/leetcode_go/define"

	"github.com/stretchr/testify/assert"
)

func NewList(int) {

}

func Test_addTwoNumbers(t *testing.T) {
	testTable := []struct {
		name   string
		l1     *ListNode
		l2     *ListNode
		result *ListNode
	}{
		{
			name:   "case 1",
			l1:     define.NewListByDecimal(342),
			l2:     define.NewListByDecimal(465),
			result: define.NewListByDecimal(807),
		},
		{
			name:   "case 2",
			l1:     define.NewListByDecimal(81),
			l2:     define.NewListByDecimal(0),
			result: define.NewListByDecimal(81),
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := addTwoNumbers(c.l1, c.l2)
			assert.Equal(t, c.result, result)
		})
	}
}
