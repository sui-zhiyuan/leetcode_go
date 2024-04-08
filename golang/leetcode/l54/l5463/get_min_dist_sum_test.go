package l5463

import "testing"

func Test_getMinDistSum(t *testing.T) {
	type args struct {
		positions [][]int
	}
	tests := []struct {
		name string
		args args
		want float64
	}{
		{
			name: "case 1",
			args: args{
				positions: [][]int{{0, 1}, {1, 0}, {1, 2}, {2, 1}},
			},
			want: 4,
		},
		{
			name: "case 2",
			args: args{
				positions: [][]int{{0, 1}, {3, 2}, {4, 5}, {7, 6}, {8, 9}, {11, 1}, {2, 12}},
			},
			want: 32.9403620269167,
		},
		{
			name: "case 3",
			args: args{
				positions: [][]int{{1, 1}},
			},
			want: 0,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := getMinDistSum(tt.args.positions); got != tt.want {
				t.Errorf("getMinDistSum() = %v, want %v", got, tt.want)
			}
		})
	}
}
