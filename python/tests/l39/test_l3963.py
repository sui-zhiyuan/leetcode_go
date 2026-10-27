import pytest

from leetcode.l39.l3963 import create_grid


def count_paths(grid: list[str]) -> int:
    """统计只允许向右/向下移动时，从左上到右下的路径数。"""
    rows, cols = len(grid), len(grid[0])
    dp = [[0] * cols for _ in range(rows)]

    for row in range(rows):
        for col in range(cols):
            if grid[row][col] == "#":
                continue
            if row == 0 and col == 0:
                dp[row][col] = 1
                continue
            from_top = dp[row - 1][col] if row > 0 else 0
            from_left = dp[row][col - 1] if col > 0 else 0
            dp[row][col] = from_top + from_left

    return dp[rows - 1][cols - 1]


@pytest.mark.parametrize(
    ("m", "n"),
    [(1, 1), (1, 5), (5, 1), (2, 2), (3, 4), (5, 5)],
)
def test_create_grid_has_exactly_one_path(m: int, n: int) -> None:
    grid = create_grid(m, n)

    assert len(grid) == m
    assert all(len(row) == n for row in grid)
    assert {cell for row in grid for cell in row} <= {".", "#"}
    assert grid[0][0] == "."
    assert grid[-1][-1] == "."
    assert count_paths(grid) == 1
