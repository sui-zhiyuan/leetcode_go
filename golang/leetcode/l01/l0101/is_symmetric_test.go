package l0101

import (
	"testing"

	"github.com/sui-zhiyuan/leetcode_go/golang/common"
)

func Test_isSymmetric(t *testing.T) {
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
				root: common.NewTree([]int{1, 2, 2, 3, 4, 4, 3}),
			},
			want: true,
		},
		{
			name: "case 2",
			args: args{
				root: common.NewTree([]int{1, 2, 2, common.Null, 3, common.Null, 3}),
			},
			want: false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := isSymmetric(tt.args.root); got != tt.want {
				t.Errorf("isSymmetric() = %v, want %v", got, tt.want)
			}
		})
	}
}
