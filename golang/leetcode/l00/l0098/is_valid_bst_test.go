package l0098

import (
	"testing"

	"github.com/sui-zhiyuan/leetcode_go/golang/common"
)

func Test_isValidBST(t *testing.T) {
	type args struct {
		root *TreeNode
	}
	tests := []struct {
		name string
		args args
		want bool
	}{
		{
			name: "case 1",
			args: args{
				root: common.NewTree([]int{2, 1, 3}),
			},
			want: true,
		},
		{
			name: "case 2",
			args: args{
				root: common.NewTree([]int{5, 1, 4, common.Null, common.Null, 3, 6}),
			},
			want: false,
		},
		{
			name: "case 3",
			args: args{
				root: common.NewTree([]int{1, 1, 1}),
			},
			want: false,
		},
		{
			name: "case 4",
			args: args{
				root: common.NewTree([]int{5, 2, 7, 1, 6, 4, 8}),
			},
			want: false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := isValidBST(tt.args.root); got != tt.want {
				t.Errorf("isValidBST() = %v, want %v", got, tt.want)
			}
		})
	}
}
