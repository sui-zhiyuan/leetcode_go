package l5445

import "testing"

func Test_rangeSum(t *testing.T) {
	type args struct {
		nums  []int
		n     int
		left  int
		right int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "case 1",
			args: args{
				nums:  []int{1, 2, 3, 4},
				n:     4,
				left:  1,
				right: 5,
			},
			want: 13,
		},
		{
			name: "case 2",
			args: args{
				nums:  []int{1, 2, 3, 4},
				n:     4,
				left:  3,
				right: 4,
			},
			want: 6,
		},
		{
			name: "case 3",
			args: args{
				nums:  []int{1, 2, 3, 4},
				n:     4,
				left:  1,
				right: 10,
			},
			want: 50,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := rangeSum(tt.args.nums, tt.args.n, tt.args.left, tt.args.right); got != tt.want {
				t.Errorf("rangeSum() = %v, want %v", got, tt.want)
			}
		})
	}
}
