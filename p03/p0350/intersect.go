package p0350

import (
	"sort"
)

func intersect(nums1 []int, nums2 []int) []int {
	sort.Slice(nums1, func(i, j int) bool {
		return nums1[i] < nums1[j]
	})

	sort.Slice(nums2, func(i, j int) bool {
		return nums2[i] < nums2[j]
	})

	n1, n2 := len(nums1), len(nums2)
	result := make([]int, 0, n1)

	for k1, k2 := 0, 0; k1 < n1 && k2 < n2; {
		if nums1[k1] == nums2[k2] {
			result = append(result, nums1[k1])
			k1++
			k2++
			continue
		}
		if nums1[k1] < nums2[k2] {
			k1++
			continue
		}
		k2++
		continue
	}
	return result
}
