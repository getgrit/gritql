import subprocess
from typing import Any
import sys

from .installer import find_install

def run_cli(args: Any):
    """Runs the Grit CLI"""
    cli_path = find_install()
    code = subprocess.run([cli_path, *args])

    return code.returncode

def apply_pattern(pattern_file: str, args: Any):
    """Applies a GritQL pattern file to the Grit CLI"""
    return run_cli(["apply", pattern_file, *args])

if __name__ == "__main__":
    run_cli(sys.argv[1:])
