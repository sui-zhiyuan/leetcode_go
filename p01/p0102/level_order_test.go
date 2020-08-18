package p0102

import (
	"testing"

	"github.com/google/go-cmp/cmp"
	"github.com/sui-zhiyuan/leetcode_go/define"
)

func Test_levelOrder(t *testing.T) {
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
				{9, 20},
				{15, 7},
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := levelOrder(tt.args.root); !cmp.Equal(got, tt.want) {
				t.Errorf("levelOrder() = %v, want %v\ndiff=%v", got, tt.want, cmp.Diff(got, tt.want))
			}
		})
	}
}
