package p5447

import "testing"

func Test_winnerSquareGame(t *testing.T) {
	type args struct {
		n int
	}
	tests := []struct {
		name string
		args args
		want bool
	}{
		{
			name: "case 1",
			args: args{n: 1},
			want: true,
		},
		{
			name: "case 2",
			args: args{n: 2},
			want: false,
		},
		{
			name: "case 3",
			args: args{n: 4},
			want: true,
		},
		{
			name: "case 4",
			args: args{n: 7},
			want: false,
		},
		{
			name: "case 5",
			args: args{n: 17},
			want: false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := winnerSquareGame(tt.args.n); got != tt.want {
				t.Errorf("winnerSquareGame() = %v, want %v", got, tt.want)
			}
		})
	}
}
