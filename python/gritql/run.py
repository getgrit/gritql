import subprocess
from typing import Any
import sys

from .installer import find_install

def run_cli(args: Any):
    """Runs the Grit CLI"""
    cli_path = find_install()
    subprocess.check_call([cli_path, *args])

if __name__ == "__main__":
    run_cli(sys.argv[1:])
