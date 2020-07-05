package p0206

import (
	"testing"

	"github.com/sui-zhiyuan/leetcode_go/define"

	"github.com/google/go-cmp/cmp"
)

func Test_reverseList(t *testing.T) {
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
			args: args{
				head: define.NewList([]int{1, 2, 3, 4, 5}),
			},
			want: define.NewList([]int{5, 4, 3, 2, 1}),
		},
		{
			name: "case 2",
			args: args{
				head: define.NewList([]int{}),
			},
			want: define.NewList([]int{}),
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := reverseList(tt.args.head); !cmp.Equal(got, tt.want) {
				t.Errorf("reverseList() = %v, want %v\ndiff=%v", got, tt.want, cmp.Diff(got, tt.want))
			}
		})
	}
}
