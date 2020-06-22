package pi1681

import (
	"strings"
)

func patternMatching(pattern string, value string) bool {
	if len(pattern) == 0 {
		return len(value) == 0
	}

	aLength, bLength := 0, 0
	for _, v := range pattern {
		if v == 'a' {
			aLength++
			continue
		}
		bLength++
	}

	if aLength == 0 {
		pattern = strings.Repeat("a", bLength)
		aLength = len(pattern)
		bLength = 0
	}

	valLength := len(value)

	for aWide := 0; aWide*aLength <= valLength; aWide++ {
		bTotal := valLength - aWide*aLength
		if bLength == 0 && bTotal != 0 {
			continue
		}
		if bLength != 0 && bTotal%bLength != 0 {
			continue
		}
		bWide := 0
		if bLength != 0 {
			bWide = bTotal / bLength
		}

		if checkPattern(value, pattern, aWide, bWide) {
			return true
		}
	}

	return false
}

func checkPattern(value string, pattern string, aWide, bWide int) bool {
	aSet, bSet := false, false
	curr := 0
	aValue, bValue := "", ""
	for _, v := range pattern {
		ok := false
		if v == 'a' {
			aSet, aValue, curr, ok = checkCurr(aWide, value, aSet, aValue, curr)
		} else {
			bSet, bValue, curr, ok = checkCurr(bWide, value, bSet, bValue, curr)
		}

		if !ok {
			return false
		}
	}

	return !aSet || !bSet || aValue != bValue
}

func checkCurr(wide int, value string, set bool, cValue string, curr int) (newSet bool, newCValue string, newCurr int, ok bool) {
	if !set {
		return true, value[curr : curr+wide], curr + wide, true
	}

	return true, cValue, curr + wide, cValue == value[curr:curr+wide]
}
