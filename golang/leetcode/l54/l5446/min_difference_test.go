package l5446

import "testing"

func Test_minDifference(t *testing.T) {
	type args struct {
		nums []int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "case 1",
			args: args{nums: []int{5, 3, 2, 4}},
			want: 0,
		},
		{
			name: "case 2",
			args: args{nums: []int{1, 5, 0, 10, 14}},
			want: 1,
		},
		{
			name: "case 3",
			args: args{nums: []int{6, 6, 0, 1, 1, 4, 6}},
			want: 2,
		},
		{
			name: "case 4",
			args: args{nums: []int{1, 5, 6, 14, 15}},
			want: 1,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := minDifference(tt.args.nums); got != tt.want {
				t.Errorf("minDifference() = %v, want %v", got, tt.want)
			}
		})
	}
}
