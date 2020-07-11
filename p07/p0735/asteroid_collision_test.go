package p0735

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_asteroidCollision(t *testing.T) {
	type args struct {
		asteroids []int
	}
	tests := []struct {
		name string
		args args
		want []int
	}{
		{
			name: "case 1",
			args: args{
				asteroids: []int{5, 10, -5},
			},
			want: []int{5, 10},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := asteroidCollision(tt.args.asteroids); !cmp.Equal(got, tt.want) {
				t.Errorf("asteroidCollision() = %v, want %v\ndiff=%v", got, tt.want, cmp.Diff(got, tt.want))
			}
		})
	}
}
