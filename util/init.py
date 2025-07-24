#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import pathlib
import os
import subprocess
import sys

curr = os.path.abspath(__file__)
root = pathlib.Path(curr).parent.parent.resolve()

file = sys.argv[1]
if not file.startswith("l") or len(file) < 5:
    raise Exception(
        "invalid file name, file name should start with l and followed by a 4 digit number"
    )

mod = file[0:3]

lib_file = pathlib.Path(root, "rust", "src", "lib.rs")
lib_content = lib_file.read_text()
lib_content = lib_content.splitlines(True)
new_mod = f"pub mod {mod};\n"
if new_mod not in lib_content:
    lib_content.append(new_mod)
    lib_content.sort()
    lib_file.write_text("".join(lib_content))

folder = pathlib.Path(root, "rust", "src", mod)
if not folder.exists():
    folder.mkdir(parents=True)


mod_file = pathlib.Path(folder, "mod.rs")
if not mod_file.exists():
    mod_file.touch()

mode_file_content = mod_file.read_text()
file_mod = f"pub mod {file};\n"
if file_mod not in mode_file_content:
    mode_file_content += file_mod
    mode_file_content = "".join(sorted(mode_file_content.splitlines(True)))
    mod_file.write_text(mode_file_content)

src_file = pathlib.Path(root, "rust", "src", mod, file + ".rs")
if not src_file.exists():
    src_file.touch()

subprocess.run(["code", str(src_file)], shell=True)
