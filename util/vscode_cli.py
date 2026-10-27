"""在 Windows、WSL 和 Linux 上调用 VS Code CLI。

Windows 的 `code` 是 `code.cmd`，需要经过 `cmd.exe`；WSL/Linux 的 `code`
通常是一个可执行的 shell 包装脚本，直接用参数列表执行即可。
"""

from __future__ import annotations

import os
import shutil
import subprocess
from pathlib import Path


def _code_command() -> str:
    code = shutil.which("code")
    if code is None:
        raise SystemExit(
            "找不到 code 命令。Windows 请把 VS Code 的 bin 目录加入 PATH；"
            "WSL 请安装 VS Code 的 Remote - WSL 扩展。"
        )
    return code


def _run_code(args: list[str], *, stdin: bytes | None = None) -> None:
    code = _code_command()

    if os.name == "nt":
        # code.cmd 需要通过 cmd.exe 执行，并且路径中可能包含空格。
        command: str | list[str] = subprocess.list2cmdline([code, *args])
        shell = True
    else:
        # WSL/Linux：code 是可执行脚本，直接传参数列表，避免 shell 丢掉参数。
        command = [code, *args]
        shell = False

    result = subprocess.run(
        command,
        input=stdin,
        capture_output=True,
        shell=shell,
        check=False,
    )

    if result.returncode != 0:
        error = result.stderr.decode("utf-8", "replace").strip()
        raise SystemExit(f"code 执行失败（退出码 {result.returncode}）：{error}")


def show_text_in_vscode(text: str) -> None:
    """通过 `code -` 把文本作为未命名文件打开。"""
    _run_code(["-"], stdin=text.encode("utf-8"))


def open_in_vscode(path: str | Path) -> None:
    """在 VS Code 中打开指定文件或目录。"""
    _run_code([str(path)])
