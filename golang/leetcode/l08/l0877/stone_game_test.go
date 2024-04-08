package l0877

import "testing"

func Test_stoneGame(t *testing.T) {
	type args struct {
		piles []int
	}
	tests := []struct {
		name string
		args args
		want bool
	}{
		{
			name: "case 1",
			args: args{
				piles: []int{5, 3, 4, 5},
			},
			want: true,
		},
		{
			name: "case 2",
			args: args{
				piles: []int{3, 2, 10, 4},
			},
			want: true,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := stoneGame(tt.args.piles); got != tt.want {
				t.Errorf("stoneGame() = %v, want %v", got, tt.want)
			}
		})
	}
}
