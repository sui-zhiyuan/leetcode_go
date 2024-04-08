package p2q1

import "testing"

func Test_maxProfit(t *testing.T) {
	type args struct {
		values []int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "case 1",
			args: args{values: []int{8, 10, 6, 9, 7, 10, 11, 4}},
			want: 5,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := maxProfit(tt.args.values); got != tt.want {
				t.Errorf("maxProfit() = %v, want %v", got, tt.want)
			}
		})
	}
}
