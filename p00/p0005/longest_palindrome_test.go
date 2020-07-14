package p0005

import "testing"

func Test_longestPalindrome(t *testing.T) {
	type args struct {
		s string
	}
	tests := []struct {
		name string
		args args
		want string
	}{
		{
			name: "case 1",
			args: args{s: "babad"},
			want: "aba",
		},
		{
			name: "case 2",
			args: args{s: "cbbd"},
			want: "bb",
		},
		{
			name: "case 3",
			args: args{s: "abcd"},
			want: "d",
		},
		{
			name: "case 4",
			args: args{s: ""},
			want: "",
		},
		{
			name: "case 5",
			args: args{s: "a"},
			want: "a",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := longestPalindrome(tt.args.s); got != tt.want {
				t.Errorf("longestPalindrome() = %v, want %v", got, tt.want)
			}
		})
	}
}
