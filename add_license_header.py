#!/usr/bin/env python3 

import os, sys

SPDX_LICENSE_ID = "AGPL-3.0-or-later"
SPDX_LICENSE_HEADER = "Copyright © 2024 The µCAD authors <info@ucad.xyz>"

def add_license_header(file_path, comment):
    with open(file_path, "r", encoding="utf-8") as f:
        lines = f.readlines()
    with open(file_path, "w", encoding="utf-8") as f:
        f.write(comment + " " + SPDX_LICENSE_HEADER + "\n")
        f.write(comment + " " + "SPDX-License-Identifier: " + SPDX_LICENSE_ID + "\n")
        f.write("\n")
        for line in lines:
            f.write(line)
        f.write("\n")

for root, dirs, files in os.walk("."):
    for file in files:
        abs_path = os.path.join(root, file)
        if file.endswith(".rs"):
            print(abs_path)
            add_license_header(abs_path, "//")
        elif file == "Cargo.toml":
            add_license_header(abs_path, "#")

