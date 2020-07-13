package p0350

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_intersect(t *testing.T) {
	type args struct {
		nums1 []int
		nums2 []int
	}
	tests := []struct {
		name string
		args args
		want []int
	}{
		{
			name: "case 1",
			args: args{
				nums1: []int{1, 2, 2, 1},
				nums2: []int{2, 2},
			},
			want: []int{2, 2},
		},
		{
			name: "case 2",
			args: args{
				nums1: []int{4, 9, 5},
				nums2: []int{9, 4, 9, 8, 4},
			},
			want: []int{4, 9},
		},
		{
			name: "case 3",
			args: args{
				nums1: []int{},
				nums2: []int{},
			},
			want: []int{},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := intersect(tt.args.nums1, tt.args.nums2); !cmp.Equal(got, tt.want) {
				t.Errorf("intersect() = %v, want %v\ndiff=%v", got, tt.want, cmp.Diff(got, tt.want))
			}
		})
	}
}
