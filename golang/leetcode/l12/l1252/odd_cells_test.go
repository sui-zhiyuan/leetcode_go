package l1252

import "testing"

func Test_oddCells(t *testing.T) {
	type args struct {
		n       int
		m       int
		indices [][]int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "case 1",
			args: args{
				n:       2,
				m:       3,
				indices: [][]int{{0, 1}, {1, 1}},
			},
			want: 6,
		},
		{
			name: "case 2",
			args: args{
				n:       2,
				m:       2,
				indices: [][]int{{1, 1}, {0, 0}},
			},
			want: 0,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := oddCells(tt.args.n, tt.args.m, tt.args.indices); got != tt.want {
				t.Errorf("oddCells() = %v, want %v", got, tt.want)
			}
		})
	}
}
