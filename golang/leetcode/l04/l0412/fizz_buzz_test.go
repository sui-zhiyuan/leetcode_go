package l0412

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_fizzBuzz(t *testing.T) {
	type args struct {
		n int
	}
	tests := []struct {
		name string
		args args
		want []string
	}{
		{
			name: "case 1",
			args: args{
				n: 15,
			},
			want: []string{"1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz"},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := fizzBuzz(tt.args.n); !cmp.Equal(got, tt.want) {
				t.Errorf("fizzBuzz() = %v, want %v\ndiff=%v", got, tt.want, cmp.Diff(got, tt.want))
			}
		})
	}
}
