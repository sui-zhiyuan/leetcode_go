package l1163

import "testing"

func Test_lastSubstring(t *testing.T) {
	type args struct {
		s string
	}
	tests := []struct {
		name string
		args args
		want string
	}{
		//cspell:disable
		{
			name: "case 1",
			args: args{s: "abab"},
			want: "bab",
		},
		{
			name: "case 2",
			args: args{s: "leetcode"},
			want: "tcode",
		},
		{
			name: "case 3",
			args: args{s: "xbylisvborylklftlkcioajuxwdhahdgezvyjbgaznzayfwsaumeccpfwamfzmkinezzwobllyxktqeibfoupcpptncggrdqbkji"},
			want: "zzwobllyxktqeibfoupcpptncggrdqbkji",
		},
		{
			name: "case 4",
			args: args{s: "cacacb"},
			want: "cb",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := lastSubstring(tt.args.s); got != tt.want {
				t.Errorf("lastSubstring() = %v, want %v", got, tt.want)
			}
		})
	}
}
