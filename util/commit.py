#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import sys
import pathlib
import os
import re

curr = os.path.abspath(__file__)
root = pathlib.Path(curr).parent.parent.resolve()

file = sys.argv[1]
if not file.startswith("l") or len(file) < 5:
    raise Exception(
        "invalid file name, file name should start with l and followed by a 4 digit number"
    )

file = pathlib.Path(root, "rust", "src", file[0:3], file + ".rs")
if not file.exists():
    raise Exception("file does not exist", file)

text = file.read_text()

placeholder = ["type TreeNode = crate::common::TreeNode<[fiu]([0-9]+|size)>;\n",
               "type ListNode = crate::common::ListNode<[fiu]([0-9]+|size)>;\n"]
for r in placeholder:
    m = re.search(r, text)
    if m:
        text = text.replace(m.group(0), "")

# remove debug and test codes
solution_index = text.find("\n// debug and test")
if solution_index < 0:
    solution_index = text.find("\nimpl Debug for")
if solution_index < 0:
    solution_index = text.find("\n#[cfg(test)]")
if solution_index >= 0:
    text = text[0:solution_index]

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

class Replacement:
    placeholder = ""
    file_name = ""

    def __init__(self, placeholder, file_name):
        self.placeholder = placeholder
        self.file_name = file_name

replacements = [
    Replacement("use crate::common::SegmentTree;", "segment_tree.rs"),
    Replacement("use crate::common::Flyweight;", "flyweight.rs"),
    Replacement("use crate::common::DisjointSet;", "disjoin_set.rs"),
    Replacement("use crate::common::ExtendVec;", "extend_vec.rs"),
    Replacement("use crate::common::Dim2Array;", "array2d.rs"),
    
]

for r in replacements:
    seg_index = text.find(r.placeholder)
    if seg_index < 0:
        continue
    seg_path = pathlib.Path(root, "rust", "src", "common", r.file_name)
    seg_text = seg_path.read_text()
    seg_index = seg_text.find("// debug and test")
    if seg_index >= 0:
        seg_text = seg_text[0:seg_index]
    while seg_text.startswith("\n"):
        seg_text = seg_text.removeprefix("\n")
    while seg_text.endswith("\n"):
        seg_text = seg_text.removesuffix("\n")

    mod_name = r.file_name.removesuffix(".rs")
    struct_name = r.placeholder.removeprefix("use crate::common::").removesuffix(";")
    seg_text = (
        "mod {mod}{{\n".format(mod=mod_name)
        + "".join(["    " + v for v in seg_text.splitlines(True)])
        + "}\n"
        + "use {mod}::{struct};\n".format(mod=mod_name, struct=struct_name)
    )

    text = text.replace(r.placeholder, seg_text)

print(text, end="")
