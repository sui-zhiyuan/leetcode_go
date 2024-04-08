package p1q1

import (
	"testing"

	"github.com/google/go-cmp/cmp"
	"github.com/sui-zhiyuan/leetcode_go/golang/common"
)

func TestRevert(t *testing.T) {
	type args struct {
		head *ListNode
	}
	tests := []struct {
		name string
		args args
		want *ListNode
	}{
		{
			name: "case 1",
			args: args{head: common.NewList([]int{1, 2, 3, 4, 5})},
			want: common.NewList([]int{5, 4, 3, 2, 1}),
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := Revert(tt.args.head); !cmp.Equal(got, tt.want) {
				t.Errorf("Revert() = %v, want %v\ndiff=%v", got, tt.want, cmp.Diff(got, tt.want))
			}
		})
	}
}
