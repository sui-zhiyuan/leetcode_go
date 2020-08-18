package p0101

import (
	"testing"

	"github.com/sui-zhiyuan/leetcode_go/define"
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
				root: define.NewTree([]int{1, 2, 2, 3, 4, 4, 3}),
			},
			want: true,
		},
		{
			name: "case 2",
			args: args{
				root: define.NewTree([]int{1, 2, 2, define.Null, 3, define.Null, 3}),
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
