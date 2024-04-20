#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import sys
import pathlib
import os

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

# remove content after "pub struct Solution;"
solution_index = text.find("pub struct Solution;")
if solution_index < 0:
    raise Exception("Solution struct not found in ", file)
text = text[0:solution_index]


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

print(text)
