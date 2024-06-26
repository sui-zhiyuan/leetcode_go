package li1681

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPatternMatching(t *testing.T) {
	testTable := []struct {
		name    string
		pattern string
		value   string
		ok      bool
	}{
		// cspell:ignore dogcatcatdog
		{
			name:    "case 1",
			pattern: "abba",
			value:   "dogcatcatdog",
			ok:      true,
		},
		// cspell:ignore dogcatcatfish
		{
			name:    "case 2",
			pattern: "abba",
			value:   "dogcatcatfish",
			ok:      false,
		},
		{
			name:    "case 3",
			pattern: "aaaa",
			value:   "dogcatcatdog",
			ok:      false,
		},
		// cspell:ignore dogdogdogdog
		{
			name:    "case 4",
			pattern: "abba",
			value:   "dogdogdogdog",
			ok:      true,
		},
		{
			name:    "case 5",
			pattern: "",
			value:   "",
			ok:      true,
		},
		{
			name:    "case 6",
			pattern: "abba",
			value:   "",
			ok:      false,
		},
		{
			name:    "case 7",
			pattern: "ab",
			value:   "",
			ok:      false,
		},
		// cspell:ignore catcat
		{
			name:    "case 8",
			pattern: "",
			value:   "catcat",
			ok:      false,
		},
		// cspell:ignore thuhrh
		{
			name:    "case 9",
			pattern: "bb",
			value:   "thuhrh",
			ok:      false,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			ok := patternMatching(c.pattern, c.value)
			assert.Equal(t, c.ok, ok)
		})
	}
}
