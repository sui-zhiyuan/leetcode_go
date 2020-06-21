package s01

func isPalindrome(s string) bool {
	begin, end := 0, len(s)-1

	for begin < end {
		beginChar, endChar := lower(s[begin]), lower(s[end])
		switch {
		case beginChar == endChar:
			begin++
			end--
			continue
		case beginChar == '\b':
			begin++
			continue
		case endChar == '\b':
			end--
			continue
		default:
			return false
		}
	}
	return true
}

func lower(c byte) byte {
	if c >= '0' && c <= '9' {
		return c
	}
	if c >= 'a' && c <= 'z' {
		return c
	}
	if c >= 'A' && c <= 'Z' {
		return c + ('a' - 'A')
	}
	return '\b'
}
