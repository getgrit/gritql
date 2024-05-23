import subprocess
from typing import Any
import sys

from .installer import find_install

def run_cli(args: Any):
    """Runs the Grit CLI"""
    cli_path = find_install()
    print("Running GritQL pattern with args:", cli_path, args)

    code = subprocess.run([cli_path, *args])


    return code.returncode

def apply_pattern(pattern_or_name: str, args: Any, grit_dir: str = None):
    """Applies a GritQL pattern to the Grit CLI"""
    final_args = ["apply", pattern_or_name, *args]
    if grit_dir:
        final_args.append("--grit-dir")
        final_args.append(grit_dir)
    return run_cli(final_args)

if __name__ == "__main__":
    run_cli(sys.argv[1:])
