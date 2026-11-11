#!/usr/bin/env python3
"""The apps' names, read from their one owner, Code/Names.xcconfig (constitution article 15).

A tool that names a product -- the Mac app's bundle, its disk image's volume, its files, its release, the site's line -- reads it
here and never types it (decision 829 = a, 11.11.2026: the Mac app is named MT Protocol on the site and in its releases mac-v1.0-N,
as the site and the repository name it). It stands beside the published tools, so an independent builder reads the same line.
Usage: python3 tools/mt_names.py [KEY]   the line of KEY, the Mac product's name by default
       python3 tools/mt_names.py --stem  the Mac product's name in a file's name (spaces become hyphens)
"""
import os
import sys

NAMES = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "Code", "Names.xcconfig")


def name(key, path=NAMES):
    with open(path, encoding="utf-8") as fh:
        for line in fh:
            k, sep, value = line.partition("=")
            if sep and k.strip() == key:
                return value.strip()
    raise SystemExit("Code/Names.xcconfig states no " + key)


def mac():
    """The Mac product as a person reads it: the app under its icon, the disk image's volume, the release's title."""
    return name("MT_NAME_PROTOCOL_MAC")


def mac_stem():
    """The same name inside a file's name: STEM-N-COMMIT.dmg, STEM-N.xcarchive, STEM-1.0-N.dmg on the release."""
    return mac().replace(" ", "-")


if __name__ == "__main__":
    args = sys.argv[1:]
    print(mac_stem() if args == ["--stem"] else name(args[0]) if args else mac())
