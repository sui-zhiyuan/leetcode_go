package i20200725

import "testing"

func Test_calculate(t *testing.T) {
	type args struct {
		a [][]int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "case 1",
			args: args{
				a: [][]int{
					{7},
					{3, 8},
					{8, 1, 0},
					{2, 7, 4, 4},
					{4, 5, 2, 6, 5},
				},
			},
			want: 30,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := calculate(tt.args.a); got != tt.want {
				t.Errorf("calculate() = %v, want %v", got, tt.want)
			}
		})
	}
}
