# mdvim — terminal markdown viewer with file tree

mdvim is a fast TUI viewer for Markdown. It shows a file tree on the left and a rendered, syntax-highlighted preview on the right. You can drive it with Vim-like keys, mouse scroll, and even resize the tree divider with the mouse.

## Features
- File tree + preview layout with focus highlighting.
- Markdown rendering (tables, lists, code fences, math, links, images) via `pulldown-cmark`.
- Inline code highlighting for common languages (Rust, JS/TS, Python, Go, C/C++).
- Vim-style navigation for tree selection and preview scrolling.
- Toggle and resize file tree (keyboard or mouse drag on the divider).
- Mouse wheel scrolling in the preview.

## Quick start
```bash
cargo run --release
```
The app starts in the current directory, selects the first entry, and loads `README.md` if present.

## Controls
- Quit: `q`
- Toggle file tree visibility: `Ctrl+n` or `t` (focus moves to tree when shown)
- Focus:
  - Tree: `Ctrl+h`
  - Preview: `Ctrl+l`
  - Toggle focus (Vim-style): `Ctrl+w` then `Ctrl+w`
- File tree selection (when tree is focused): `j`/`k` or `↓`/`↑`
- Open selected file into preview: `Enter` or `o` (focus moves to preview)
- Preview scroll: `j`/`k` or arrows
- Faster scroll: `Ctrl+d` / `Ctrl+u`, `PageDown` / `PageUp`
- Jump: `g` then `g` (top), `G` (bottom), `Home`/`End`
- Resize tree:
  - Keyboard: `Ctrl+Left` (narrower), `Ctrl+Right` (wider)
  - Mouse: click near the divider between tree and preview and drag left/right
- Mouse wheel: scroll preview

## Notes
- Tree width is clamped between 10% and 80% of the terminal width.
- When the tree is hidden, focus is forced to the preview.
- The renderer supports many GFM extras (tables, strikethrough, task lists, links, footnotes, math, etc.).

## Building
- Rust stable toolchain recommended.
- Release build: `cargo build --release`

