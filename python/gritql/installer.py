from __future__ import annotations

import os
import sys
import json
import shutil
import tarfile
import platform

from typing import TYPE_CHECKING, List
from pathlib import Path

import httpx

def _cache_dir() -> Path:
    xdg = os.environ.get("XDG_CACHE_HOME")
    if xdg is not None:
        return Path(xdg)

    return Path.home() / ".cache"


def _debug(message: str) -> None:
    if not os.environ.get("DEBUG"):
        return

    sys.stdout.write(f"[DEBUG]: {message}\n")


class CLIError(Exception):
    pass

def find_install() -> Path:
    """Installs the Grit CLI and returns the location of the binary"""
    if sys.platform == "win32":
        raise CLIError("Windows is not supported yet in the migration CLI")

    grit_path = shutil.which("grit")
    if grit_path:
        _debug(f"'grit' found in PATH at {grit_path}")
        return Path(grit_path)

    platform = "apple-darwin" if sys.platform == "darwin" else "unknown-linux-gnu"

    dir_name = _cache_dir() / "grit"
    install_dir = dir_name / ".install"
    target_dir = install_dir / "bin"

    target_path = target_dir / "grit"
    temp_file = target_dir / "grit.tmp"

    if target_path.exists():
        _debug(f"{target_path} already exists")
        sys.stdout.flush()
        return target_path

    _debug(f"Using Grit CLI path: {target_path}")

    target_dir.mkdir(parents=True, exist_ok=True)

    if temp_file.exists():
        temp_file.unlink()

    arch = _get_arch()
    _debug(f"Using architecture {arch}")

    arch = _get_arch()
    _debug(f"Using architecture {arch}")

    file_name = f"grit-{arch}-{platform}"
    download_url = f"https://github.com/getgrit/gritql/releases/latest/download/{file_name}.tar.gz"

    sys.stdout.write(f"Downloading Grit CLI from {download_url}\n")
    with httpx.Client() as client:
        download_response = client.get(download_url, follow_redirects=True)
        if download_response.status_code != 200:
            raise CLIError(f"Failed to download Grit CLI from {download_url}")
        with open(temp_file, "wb") as file:
            for chunk in download_response.iter_bytes():
                file.write(chunk)

    unpacked_dir = target_dir / "cli-bin"
    unpacked_dir.mkdir(parents=True, exist_ok=True)

    with tarfile.open(temp_file, "r:gz") as archive:
        if sys.version_info >= (3, 12):
            archive.extractall(unpacked_dir, filter="data")
        else:
            archive.extractall(unpacked_dir)

    _move_files_recursively(unpacked_dir, target_dir)

    shutil.rmtree(unpacked_dir)
    os.remove(temp_file)
    os.chmod(target_path, 0o755)

    sys.stdout.flush()

    return target_path

def _move_files_recursively(source_dir: Path, target_dir: Path) -> None:
    for item in source_dir.iterdir():
        if item.is_file():
            item.rename(target_dir / item.name)
        elif item.is_dir():
            _move_files_recursively(item, target_dir)

def _get_arch() -> str:
    architecture = platform.machine().lower()

    # Map the architecture names to Grit equivalents
    arch_map = {
        "x86_64": "x86_64",
        "amd64": "x86_64",
        "armv7l": "aarch64",
        "arm64": "aarch64",
    }

    return arch_map.get(architecture, architecture)
