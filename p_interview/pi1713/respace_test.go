package pi1713

import "testing"

func Test_respace(t *testing.T) {
	type args struct {
		dictionary []string
		sentence   string
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "case 1",
			args: args{
				dictionary: []string{"looked", "just", "like", "her", "brother"},
				sentence:   "jesslookedjustliketimherbrother",
			},
			want: 7,
		},
		{
			name: "case 2",
			args: args{
				dictionary: []string{"abc"},
				sentence:   "abcabc",
			},
			want: 0,
		},
		{
			name: "case 3",
			args: args{
				dictionary: []string{"abc", "cde"},
				sentence:   "abced",
			},
			want: 2,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := respace(tt.args.dictionary, tt.args.sentence); got != tt.want {
				t.Errorf("respace() = %v, want %v", got, tt.want)
			}
		})
	}
}
