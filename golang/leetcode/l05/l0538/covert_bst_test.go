package l0538

import (
	"testing"

	"github.com/google/go-cmp/cmp"
	"github.com/sui-zhiyuan/leetcode_go/golang/common"
)

func Test_convertBST(t *testing.T) {
	type args struct {
		root *TreeNode
	}
	tests := []struct {
		name string
		args args
		want *TreeNode
	}{
		{
			name: "case 1",
			args: args{
				root: common.NewTree([]int{5, 2, 13}),
			},
			want: common.NewTree([]int{18, 20, 13}),
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := convertBST(tt.args.root); !cmp.Equal(got, tt.want) {
				t.Errorf("convertBST() = %v, want %v\ndiff=%v", got, tt.want, cmp.Diff(got, tt.want))
			}
		})
	}
}
