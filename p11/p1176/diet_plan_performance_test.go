package p1176

import "testing"

func Test_dietPlanPerformance(t *testing.T) {
	type args struct {
		calories []int
		k        int
		lower    int
		upper    int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "case 1",
			args: args{
				calories: []int{1, 2, 3, 4, 5},
				k:        1,
				lower:    3,
				upper:    3,
			},
			want: 0,
		},
		{
			name: "case 2",
			args: args{
				calories: []int{3, 2},
				k:        2,
				lower:    0,
				upper:    1,
			},
			want: 1,
		},
		{
			name: "case 1",
			args: args{
				calories: []int{6, 5, 0, 0},
				k:        2,
				lower:    1,
				upper:    5,
			},
			want: 0,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := dietPlanPerformance(tt.args.calories, tt.args.k, tt.args.lower, tt.args.upper); got != tt.want {
				t.Errorf("dietPlanPerformance() = %v, want %v", got, tt.want)
			}
		})
	}
}
