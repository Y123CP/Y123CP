#!/usr/bin/env python3
"""Restore the anonymized nested Git histories without replacing working files."""
import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import subprocess
import shutil
import tarfile
import tempfile

ROOT = Path(__file__).resolve().parents[1]
SUPPORT = ROOT / "artifact_support"


def git(repo, *args):
    return subprocess.check_output(
        ["git", "-C", str(repo), *args], text=True
    ).strip()


def safe_path(value):
    path = PurePosixPath(value)
    if path.is_absolute() or ".." in path.parts or not path.parts:
        raise ValueError("Invalid archive or repository path")
    return path


def verify_history(repo, entry):
    if git(repo, "rev-parse", "HEAD") != entry["head"]:
        raise ValueError("Existing HEAD differs: " + entry["path"])
    refs = git(repo, "for-each-ref", "--format=%(refname) %(objectname)")
    if refs != entry["refs"]:
        raise ValueError("Existing refs differ: " + entry["path"])
    subprocess.run(
        ["git", "-C", str(repo), "fsck", "--connectivity-only", "--no-dangling"],
        check=True, stdout=subprocess.DEVNULL,
    )


def restore(entry, check_only):
    repo = ROOT.joinpath(*safe_path(entry["path"]).parts)
    if not repo.is_dir() or ROOT not in repo.resolve().parents:
        raise ValueError("Repository directory is missing or escapes artifact")
    archive = SUPPORT.joinpath(*safe_path(entry["archive"]).parts)
    if hashlib.sha256(archive.read_bytes()).hexdigest() != entry["sha256"]:
        raise ValueError("Archive checksum differs: " + entry["archive"])
    dest = repo / ".git"
    if dest.is_symlink():
        raise ValueError("Refusing a symlink at " + entry["path"] + "/.git")
    if not dest.exists():
        if check_only:
            raise ValueError("History not restored: " + entry["path"])
        # Extract only directories and regular files into a temporary directory.
        # Rename the metadata into place; never check out or overwrite source files.
        with tempfile.TemporaryDirectory(prefix=".history-", dir=repo) as temp:
            with tarfile.open(archive, "r:gz") as bundle:
                for member in bundle:
                    rel = safe_path(member.name)
                    if rel.parts[0] != ".git":
                        raise ValueError("Unexpected archive member")
                    target = Path(temp).joinpath(*rel.parts)
                    if member.isdir():
                        target.mkdir(parents=True, exist_ok=True)
                    elif member.isfile():
                        target.parent.mkdir(parents=True, exist_ok=True)
                        source = bundle.extractfile(member)
                        with target.open("xb") as output:
                            shutil.copyfileobj(source, output)
                        target.chmod(member.mode & 0o777)
                    else:
                        raise ValueError("Unsupported archive member type")
            os.rename(Path(temp) / ".git", dest)
    elif not dest.is_dir():
        raise ValueError("Refusing to overwrite an existing .git file")
    verify_history(repo, entry)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="verify already restored histories")
    args = parser.parse_args()
    entries = json.loads((SUPPORT / "git_histories.json").read_text())["repositories"]
    for entry in entries:
        restore(entry, args.check)
    print("Verified {} nested Git histories; working files unchanged.".format(len(entries)))


if __name__ == "__main__":
    main()
