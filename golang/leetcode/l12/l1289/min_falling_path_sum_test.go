package l1289

import "testing"

func Test_minFallingPathSum(t *testing.T) {
	type args struct {
		arr [][]int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "case 1",
			args: args{
				arr: [][]int{
					{1, 2, 3},
					{4, 5, 6},
					{7, 8, 9},
				},
			},
			want: 13,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := minFallingPathSum(tt.args.arr); got != tt.want {
				t.Errorf("minFallingPathSum() = %v, want %v", got, tt.want)
			}
		})
	}
}
