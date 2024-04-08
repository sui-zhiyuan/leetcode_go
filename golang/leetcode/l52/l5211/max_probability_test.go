package l5211

import "testing"

// cspell:ignore succ

func Test_maxProbability(t *testing.T) {
	type args struct {
		n        int
		edges    [][]int
		succProb []float64
		start    int
		end      int
	}
	tests := []struct {
		name string
		args args
		want float64
	}{
		{
			name: "case 1",
			args: args{
				n:        3,
				edges:    [][]int{{0, 1}, {1, 2}, {0, 2}},
				succProb: []float64{0.5, 0.5, 0.2},
				start:    0,
				end:      2,
			},
			want: 0.25,
		},
		{
			name: "case 2",
			args: args{
				n:        3,
				edges:    [][]int{{0, 1}, {1, 2}, {0, 2}},
				succProb: []float64{0.5, 0.5, 0.3},
				start:    0,
				end:      2,
			},
			want: 0.3,
		},
		{
			name: "case 3",
			args: args{
				n:        3,
				edges:    [][]int{{0, 1}},
				succProb: []float64{0.5},
				start:    0,
				end:      2,
			},
			want: 0,
		},
		{
			name: "case 4",
			args: args{
				n:        5,
				edges:    [][]int{{1, 4}, {2, 4}, {0, 4}, {0, 3}, {0, 2}, {2, 3}},
				succProb: []float64{0.37, 0.17, 0.93, 0.23, 0.39, 0.04},
				start:    3,
				end:      4,
			},
			want: 0.2139,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := maxProbability(tt.args.n, tt.args.edges, tt.args.succProb, tt.args.start, tt.args.end); got != tt.want {
				t.Errorf("maxProbability() = %v, want %v", got, tt.want)
			}
		})
	}
}
