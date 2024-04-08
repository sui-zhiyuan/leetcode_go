package l1256

var charMap = []byte{'0', '1'}

func encode(num int) string {
	length := 0
	count := 0
	for num >= count {
		length++
		count = count*2 + 1
	}
	length--
	count = (count - 1) / 2

	value := num - count
	result := make([]byte, length)
	for i := 0; i < length; i++ {
		result[length-i-1] = charMap[value%2]
		value = value / 2
	}
	return string(result)
}
