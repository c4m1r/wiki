#!/usr/bin/env python3
"""Generate mdBook SUMMARY.md from Markdown files."""
from __future__ import annotations
import os
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / 'src'

header = '# Summary\n\n'
lines = []

def title_from_file(path: Path) -> str:
    try:
        with path.open('r', encoding='utf-8') as f:
            for line in f:
                m = re.match(r'^#\s+(.*)', line)
                if m:
                    return m.group(1).strip()
    except Exception:
        pass
    return path.stem.replace('_', ' ').title()

def add_file(path: Path, indent: int) -> None:
    title = title_from_file(path)
    rel = path.relative_to(SRC).as_posix()
    lines.append('    '*indent + f'- [{title}](./{rel})\n')

def walk(dir_path: Path, indent: int = 0) -> None:
    entries = sorted(dir_path.iterdir(), key=lambda p: (p.is_file(), p.name.lower()))
    intro = None
    if dir_path == SRC:
        intro = dir_path / 'introduction.md'
        if intro.exists():
            add_file(intro, indent)
    for entry in entries:
        if entry == intro or entry.name == 'SUMMARY.md' or entry.name.startswith('.'):
            continue
        if entry.is_dir():
            md_files = [f for f in sorted(entry.glob('*.md'))]
            if not md_files:
                walk(entry, indent)
                continue
            index = None
            for candidate in ['README.md', 'readme.md', 'index.md', 'overview.md']:
                cand = entry / candidate
                if cand.exists():
                    index = cand
                    break
            if index is None:
                index = md_files[0]
            add_file(index, indent)
            for f in md_files:
                if f == index:
                    continue
                add_file(f, indent+1)
            for sub in sorted(entry.iterdir()):
                if sub.is_dir():
                    walk(sub, indent+1)
        elif entry.suffix == '.md':
            add_file(entry, indent)

walk(SRC)
(SRC / 'SUMMARY.md').write_text(header + ''.join(lines), encoding='utf-8')
