package p0315

import (
	"fmt"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_countSmaller(t *testing.T) {
	type args struct {
		nums []int
	}
	tests := []struct {
		name string
		args args
		want []int
	}{
		{
			name: "case 1",
			args: args{
				nums: []int{5, 2, 6, 1},
			},
			want: []int{2, 1, 1, 0},
		},
		{
			name: "case 2",
			args: args{
				nums: []int{5, 5, 5, 1},
			},
			want: []int{1, 1, 1, 0},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := countSmaller(tt.args.nums); !cmp.Equal(got, tt.want) {
				t.Errorf("countSmaller() = %v, want %v\ndiff=%v", got, tt.want, cmp.Diff(got, tt.want))
			}
		})
	}
}

func Test_newBinaryIndexedTree(t *testing.T) {
	count := 10
	nums := make([]int, 10)
	for i := range nums {
		nums[i] = i + 1
	}
	tree := newBinaryIndexedTree(nums)

	for i := 0; i <= count; i++ {
		for j := 0; j <= i; j++ {
			fmt.Printf("[%d,%d->%d]", j, i, tree.cacualteSum(j, i))
		}
		fmt.Println()
	}
}
