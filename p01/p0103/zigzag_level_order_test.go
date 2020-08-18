package p0103

import (
	"testing"

	"github.com/google/go-cmp/cmp"
	"github.com/sui-zhiyuan/leetcode_go/define"
)

func Test_zigzagLevelOrder(t *testing.T) {
	type args struct {
		root *TreeNode
	}
	tests := []struct {
		name string
		args args
		want [][]int
	}{
		{
			name: "case 1",
			args: args{
				root: define.NewTree([]int{3, 9, 20, define.Null, define.Null, 15, 7}),
			},
			want: [][]int{
				{3},
				{20, 9},
				{15, 7},
			},
		},
		{
			name: "case 2",
			args: args{
				root: define.NewTree([]int{1, 2, 3, 4, define.Null, define.Null, 5}),
			},
			want: [][]int{
				{1},
				{3, 2},
				{4, 5},
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := zigzagLevelOrder(tt.args.root); !cmp.Equal(got, tt.want) {
				t.Errorf("zigzagLevelOrder() = %v, want %v\ndiff=%v", got, tt.want, cmp.Diff(got, tt.want))
			}
		})
	}
}
