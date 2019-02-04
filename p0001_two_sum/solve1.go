package p0001_two_sum

func twoSum(nums []int, target int) []int {
	data := make([][2]int, 0, len(nums))
	for i, v := range nums {
		data = append(data, [2]int{i, v})
	}
	quickSort(data)
	i, j := 0, len(data)-1
	for ; ; {
		switch {
		case i == j:
			return []int{}
		case data[i][1]+data[j][1] > target:
			j --
		case data[i][1]+data[j][1] < target:
			i ++
		case data[i][1]+data[j][1] == target:
			return []int{data[i][0], data[j][0]}
		}
	}
	return []int{}
}

func quickSort(data [][2]int) {
	z := data[(0+len(data)-1)/2][1]
	i, j := 0, len(data)-1
	for ; i <= j; {
		for ; data[i][1] < z; {
			i++
		}

		for ; data[j][1] > z; {
			j --
		}
		if i <= j {
			data[i], data[j] = data[j], data[i]
			i ++
			j --
		}
	}
	if j > 0 {
		quickSort(data[ 0 : j+1])
	}
	if i < len(data)-1 {
		quickSort(data[i:])
	}
}
