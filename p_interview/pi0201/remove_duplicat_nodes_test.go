package pi0201

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/sui-zhiyuan/leetcode_go/define"
)

func TestRemoveDuplicateNodes(t *testing.T) {
	testTable := []struct {
		name   string
		list   *ListNode
		result *ListNode
	}{
		{
			name:   "case 1",
			list:   define.NewList([]int{1, 2, 3, 3, 2, 1}),
			result: define.NewList([]int{1, 2, 3}),
		},
		{
			name:   "case 2",
			list:   define.NewList([]int{1, 1, 1, 1, 2}),
			result: define.NewList([]int{1, 2}),
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := removeDuplicateNodes(c.list)
			assert.Equal(t, c.result, result)
		})
	}
}
