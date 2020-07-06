package p5454

import "testing"

func Test_numSubmat(t *testing.T) {
	type args struct {
		mat [][]int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "case 1",
			args: args{
				mat: [][]int{
					{1, 0, 1},
					{1, 1, 0},
					{1, 1, 0},
				},
			},
			want: 13,
		},
		{
			name: "case 2",
			args: args{
				mat: [][]int{
					{0, 1, 1, 0},
					{0, 1, 1, 1},
					{1, 1, 1, 0},
				},
			},
			want: 24,
		},
		{
			name: "case 3",
			args: args{
				mat: [][]int{
					{1, 1, 1, 1, 1, 1},
				},
			},
			want: 21,
		},
		{
			name: "case 4",
			args: args{
				mat: [][]int{
					{1, 0, 1},
					{0, 1, 0},
					{1, 0, 1},
				},
			},
			want: 5,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := numSubmat(tt.args.mat); got != tt.want {
				t.Errorf("numSubmat() = %v, want %v", got, tt.want)
			}
		})
	}
}
