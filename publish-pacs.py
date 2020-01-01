#!/usr/bin/env python3

"""Script to automate the publication of PAC subcrates to crates.io
"""

import argparse
import pathlib
import subprocess
import sys

def build_cargo_publish(dry_run, allow_dirty):
    def _call(crate_path):
        cmd = [
            "cargo",
            "publish",
            "--manifest-path", pathlib.Path(crate_path) / "Cargo.toml",
            "--target", "thumbv7em-none-eabihf"
        ]
        if dry_run:
            cmd.append("--dry-run")
        if allow_dirty:
            cmd.append("--allow-dirty")
        subprocess.run(cmd, check=True)
    return _call

parser = argparse.ArgumentParser(description="Automate the publishing of crates from this repo")
parser.add_argument(
    "--only-pac-subcrates",
    help="Only publish all of the PAC subcrates",
    action="store_true",
)
parser.add_argument(
    "--dry-run",
    help="Perform all checks without uploading",
    action="store_true",
)
parser.add_argument(
    "--allow-dirty",
    help="Allow dirty working directories to be packaged",
    action="store_true"
)

args = parser.parse_args()
cargo_publish = build_cargo_publish(args.dry_run, args.allow_dirty)

# PAC subcrates
PAC_PATH = pathlib.Path("imxrt1062-pac")
subcrates = [
    imxrt1062_dir
    for imxrt1062_dir in PAC_PATH.iterdir()
    if "imxrt1062-" in str(imxrt1062_dir.name) and imxrt1062_dir.is_dir()
]
for subcrate in subcrates:
    cargo_publish(subcrate)