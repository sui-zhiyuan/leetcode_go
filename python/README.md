# Python 题解

采用 Python 常规的 `src/` 布局和独立 `tests/` 目录。题目用编号命名，并按每 100 题分桶，避免单个目录堆积过多文件。

## 目录结构

```text
python/
├── pyproject.toml
├── src/
│   └── leetcode/
│       ├── __init__.py
│       ├── l00/
│       │   ├── __init__.py
│       │   └── l0001.py
│       └── l01/
│           ├── __init__.py
│           └── l0102.py
└── tests/
    ├── l00/
    │   └── test_l0001.py
    └── l01/
        └── test_l0102.py
```

## 约定

- 题号每 100 题一个子包：`l00` 表示 0000–0099，`l01` 表示 0100–0199，`l39` 表示 3900–3999；1000 以上同样按百位分桶，例如 1002 放在 `l10`。
- 每题一个模块，文件名为 `l` + 四位题号，例如 `l0001.py`；不按主题分类，也不建每题目录。
- 模块名必须是合法标识符，因此用 `l0001` 而不是 `0001`；导入方式为 `from leetcode.l00.l0001 import two_sum`。
- 题目标题和链接写在模块 docstring 中：

  ```python
  """1. Two Sum / 两数之和。

  https://leetcode.com/problems/two-sum/
  """
  ```

- 测试目录镜像源码分桶：`tests/lNN/test_lNNNN.py`，例如 `tests/l00/test_l0001.py`；测试目录不需要 `__init__.py`。
- 不设公共 `common` 包。链表、二叉树等结构按题目需要在模块或测试里直接用 Python 类/列表构造；LeetCode 提交环境会提供标准的 `ListNode` / `TreeNode`。
- 面试题、剑指 Offer 等非唯一编号使用独立子包，例如 `src/leetcode/offer/offer_06.py`、`src/leetcode/interview/interview_01_01.py`，测试对应为 `tests/offer/test_offer_06.py`。
- 纯算法题直接写函数，不强制套 `Solution` 类；有状态设计题直接使用题目要求的类。

## 依赖分组与镜像

测试只需要 pytest，放在 `dev` 组；ruff 和 mypy 分开放在 `lint` / `typecheck` 组，避免每次跑测试都安装或构建它们。

`pyproject.toml` 默认使用清华 PyPI 镜像，`uv.lock` 中的下载地址也已指向该镜像：

- 切换镜像：修改 `[[tool.uv.index]]` 的 `url`，然后执行 `uv lock`。
- 切回官方源：删除 `[[tool.uv.index]]` 后执行 `uv lock`。
- `mypy` 限制为 `<2`，避免 mypy 2.x 的 `ast-serialize` 在当前环境需要源码构建。

## 快速开始

```bash
cd python
uv sync          # 只安装测试所需的 pytest
uv run pytest    # 跑全部测试
```

要一次装齐 lint 和类型检查工具：

```bash
uv sync --group lint --group typecheck
```

## 常用命令

```bash
uv run pytest                                  # 全部测试
uv run pytest tests/l00/test_l0001.py          # 单个测试文件
uv run --group lint ruff check .               # lint
uv run --group lint ruff format .              # 格式化
uv run --group typecheck mypy .                # 类型检查
```

首次执行带 `--group` 的命令时会安装对应工具，之后走 uv 缓存。

不使用 uv 时，可以执行 `pip install -e .` 安装项目，或临时把 `src` 加入 `PYTHONPATH`：

```bash
PYTHONPATH=src pytest
```
