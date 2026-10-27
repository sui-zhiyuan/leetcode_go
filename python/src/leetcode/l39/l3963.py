"""3963. 构造恰好一条路径的网格 / Create Grid With Exactly One Path。

https://leetcode.com/problems/create-grid-with-exactly-one-path/
"""

class Solution:
    def createGrid(self, m: int, n: int) -> list[str]:
        grid = [["#"] * n ] * m;
        print(grid)

        for col in range(n):
            grid[0][col] = "."
        for row in range(m):
            grid[row][n - 1] = "."

        return ["".join(row) for row in grid]
