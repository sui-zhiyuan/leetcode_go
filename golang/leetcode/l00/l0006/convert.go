package l0006

func convert(s string, numRows int) string {
	length := len(s)
	result := make([]byte, 0, length)
	groupCount := numRows*2 - 2
	if numRows == 1 {
		groupCount = 1
	}

	for i := 0; ; i++ {
		if i*groupCount >= length {
			break
		}
		result = append(result, s[i*groupCount])
	}

	for row := 1; row < numRows-1; row++ {
		for i := 0; ; i++ {
			if i*groupCount+row >= length {
				break
			}
			result = append(result, s[i*groupCount+row])
			if i*groupCount+groupCount-row >= length {
				break
			}
			result = append(result, s[i*groupCount+groupCount-row])
		}
	}

	if numRows > 1 {
		for i := 0; ; i++ {
			if i*groupCount+numRows-1 >= length {
				break
			}
			result = append(result, s[i*groupCount+numRows-1])
		}
	}

	return string(result)
}
