# leetcode_go

LeetCode 题解集合，包含 Go、Rust 和 Python 三种语言。

- `golang/`：Go 题解，题目位于 `golang/leetcode/lNN/lNNNN/`
- `rust/`：Rust 题解，题目位于 `rust/src/lNN/lNNNN.rs`
- `python/`：Python 题解，模块位于 `python/src/leetcode/lNN/lNNNN.py`，测试位于 `python/tests/lNN/test_lNNNN.py`
- `util/`：辅助脚本

## 目录约定

- Go/Rust：按每 100 题一个目录（`l00`、`l01`…）组织；另有 `l_interview` / `l_offer` 等特殊目录。
- Python：同样按每 100 题分桶（`l00`、`l01`…），题目用 `lNNNN` 编号命名，不按主题分类；题号、标题和链接写在模块 docstring 中。

## 运行测试

```bash
cd golang && go test ./...
cd rust && cargo test -- --skip l9814  # l9814 目前会死循环
cd python && uv run pytest
```

Python 的目录结构与工具链详见 `python/README.md`。
