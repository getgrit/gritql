import subprocess
from pathlib import Path
from typing import Sequence
from .installer import find_install

def run_cli(args: Sequence[str]) -> int:
    """Executes the Grit CLI with the provided arguments.

    Args:
        args: A sequence of command-line arguments to pass to the Grit CLI.

    Returns:
        The exit code returned by the Grit CLI process.

    Raises:
        FileNotFoundError: If the Grit CLI binary cannot be located.
    """
    binary = find_install()
    cmd = [str(binary)] + list(args)
    return subprocess.run(cmd).returncode

def apply_pattern(pattern_or_name: str, args: Sequence[str], grit_dir: str | None = None) -> int:
    """Applies a specific GritQL pattern using the Grit CLI.

    Args:
        pattern_or_name: The name or path of the GritQL pattern to apply.
        args: A sequence of additional command-line arguments.
        grit_dir: Optional path to the directory containing the grit configuration.

    Returns:
        The exit code returned by the Grit CLI process.
    """
    cmd = ["apply", pattern_or_name] + list(args)
    if grit_dir:
        cmd.extend(["--grit_dir", grit_dir])
    return run_cli(cmd)
