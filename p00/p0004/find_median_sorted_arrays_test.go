package p0004

import "testing"

func Test_findMedianSortedArrays(t *testing.T) {
	type args struct {
		nums1 []int
		nums2 []int
	}
	tests := []struct {
		name string
		args args
		want float64
	}{
		{
			name: "case 1",
			args: args{
				nums1: []int{1, 3},
				nums2: []int{2},
			},
			want: 2,
		},
		{
			name: "case 2",
			args: args{
				nums1: []int{1, 3},
				nums2: []int{2, 4},
			},
			want: 2.5,
		},
		{
			name: "case 3",
			args: args{
				nums1: []int{0, 0, 0, 0, 0},
				nums2: []int{-1, 0, 0, 0, 0, 0, 1},
			},
			want: 0,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := findMedianSortedArrays(tt.args.nums1, tt.args.nums2); got != tt.want {
				t.Errorf("findMedianSortedArrays() = %v, want %v", got, tt.want)
			}
		})
	}
}
