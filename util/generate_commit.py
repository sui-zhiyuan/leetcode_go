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

file = pathlib.Path(root, "rust" , "src" ,file[0:3], file + ".rs")
if not file.exists():
    raise Exception("file does not exist", file)

text = file.read_text()

# remove content after "pub struct Solution();"
solution_index = text.find("pub struct Solution();")
if solution_index < 0:
    raise Exception("Solution struct not found in ", file)
text = text[0:solution_index]

# replace SegmentTree
placeholder = "type SegmentTree<T, F> = crate::common::SegmentTree<T, F>;"
seg_index = text.find(placeholder)
if seg_index >= 0:
    seg_path = pathlib.Path(root, "rust" , "src" , "common", "segment_tree.rs")
    seg_text = seg_path.read_text()
    seg_index = seg_text.find("// debug and test")
    if seg_index >=0 :
        seg_text = seg_text[0:seg_index]
    seg_text = seg_text.removeprefix("use std::fmt::Debug;")
    while seg_text.startswith("\n"):
        seg_text = seg_text.removeprefix("\n")
    while seg_text.endswith("\n"):
        seg_text = seg_text.removesuffix("\n")
    text = text.replace(placeholder, seg_text)

print(text)
