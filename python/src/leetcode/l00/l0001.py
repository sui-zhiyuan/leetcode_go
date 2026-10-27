"""1. Two Sum / 两数之和。

https://leetcode.com/problems/two-sum/
"""


def two_sum(nums: list[int], target: int) -> list[int]:
    """返回和为 target 的两个元素的下标，保证有且仅有一个答案。"""
    seen: dict[int, int] = {}

    for index, value in enumerate(nums):
        complement = target - value
        if complement in seen:
            return [seen[complement], index]
        seen[value] = index

    raise ValueError("no two sum solution")
