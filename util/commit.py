#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import subprocess
import sys
import pathlib
import os
import re


class Replacement:
    placeholder = ""
    file_name = ""

    def __init__(self, placeholder, file_name):
        self.placeholder = placeholder
        self.file_name = file_name


placeholders = [
    "type TreeNode = crate::common::TreeNode<[fiu]([0-9]+|size)>;\n",
    "type ListNode = crate::common::ListNode<[fiu]([0-9]+|size)>;\n",
]

replacements = [
    Replacement("use crate::common::SegmentTree;", "segment_tree.rs"),
    Replacement("use crate::common::Flyweight;", "flyweight.rs"),
    Replacement("use crate::common::DisjointSet;", "disjoin_set.rs"),
    Replacement("use crate::common::ExtendVec;", "extend_vec.rs"),
    Replacement("use crate::common::Grid;", "grid.rs"),
    Replacement("use crate::common::Coordinate;", "grid.rs"),
    Replacement("use crate::common::{Coordinate, Grid};", "grid.rs"),
    Replacement("use crate::common::binary_search;", "binary_search.rs"),
]

root = None


def get_project_root() -> pathlib.Path:
    global root
    if root is not None:
        return root
    curr = os.path.abspath(__file__)
    root = pathlib.Path(curr).parent.parent.resolve()
    return root


def normal_input_path() -> pathlib.Path:
    if len(sys.argv) != 2:
        current_file = os.path.basename(__file__)
        message = f"invalid parameter, use: {current_file} <target_file>"
        raise Exception(message)
    file = sys.argv[1]
    if not file.startswith("l") or len(file) < 5:
        raise Exception(
            "invalid file name, file name should start with l and followed by a 4 digit number"
        )

    file = pathlib.Path(get_project_root(), "rust", "src", file[0:3], file + ".rs")
    if not file.exists():
        raise Exception("file does not exist", file)
    return file


def remove_debug_code(text: str) -> str:
    solution_index = text.find("\n// debug and test")
    if solution_index < 0:
        solution_index = text.find("\nimpl Debug for")
    if solution_index < 0:
        solution_index = text.find("\n#[cfg(test)]")
    if solution_index >= 0:
        text = text[0:solution_index]
    return text


def process_code(text: str) -> str:
    for r in placeholders:
        m = re.search(r, text)
        if m:
            text = text.replace(m.group(0), "")
    # remove debug and test codes
    text = remove_debug_code(text)
    first_fn_start = text.find("pub fn ")
    if first_fn_start < 0:
        raise Exception("no function found")
    first_fn_end = text.find("\n}", first_fn_start)
    if first_fn_end < 0:
        raise Exception("no function end found")
    first_fn_end += 2
    prefix = text[0:first_fn_start]
    middle = text[first_fn_start:first_fn_end]
    suffix = text[first_fn_end:]
    text = (
            prefix
            + "impl Solution {\n"
            + "".join(["    " + v for v in middle.splitlines(True)])
            + "\n}"
            + suffix
    )
    for r in replacements:
        seg_index = text.find(r.placeholder)
        if seg_index < 0:
            continue

        # replace use
        mod_name = r.file_name.removesuffix(".rs")
        struct_name = r.placeholder.removeprefix("use crate::common::").removesuffix(";")
        replace_use = "use {mod}::{struct};".format(mod=mod_name, struct=struct_name)
        text = text.replace(r.placeholder, replace_use)
        text += "\n"

        # add text to end
        seg_path = pathlib.Path(get_project_root(), "rust", "src", "common", r.file_name)
        seg_text = seg_path.read_text()
        seg_text = remove_debug_code(seg_text)
        while seg_text.startswith("\n"):
            seg_text = seg_text.removeprefix("\n")
        while seg_text.endswith("\n"):
            seg_text = seg_text.removesuffix("\n")

        seg_text = (
                "mod {mod}{{\n".format(mod=mod_name)
                + "".join(["    " + v for v in seg_text.splitlines(True)])
                + "\n}\n"
        )
        text += seg_text
    return text


def show_in_vscode(text: str):
    process = subprocess.Popen(
        ["code", "-"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        shell=True,
    )
    process.communicate(text.encode(encoding="gbk"))


def main():
    text = normal_input_path().read_text(encoding="UTF-8")
    text = process_code(text)
    show_in_vscode(text)


if __name__ == "__main__":
    main()
