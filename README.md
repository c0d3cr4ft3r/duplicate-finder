# 🔁 Duplicate Finder

A high-performance CLI tool written in Rust to scan, detect, preview, and clean up duplicate files using content-based hashing (BLAKE3). Works seamlessly across macOS, Linux, and Windows.

---

## ⚙️ Features

- ✅ Multi-threaded scanning using Rayon
- ✅ File grouping by size and content hash (BLAKE3)
- ✅ Smart deletion policies:
  - Keep newest / oldest by modification time
  - Keep file with the shortest path
  - Keep file matching a regex pattern
- ✅ Interactive file deletion mode
- ✅ Preview files using system viewer

---

## 🚀 Usage

```bash
cargo run --release -- <path> [OPTIONS]
```

### Required Argument

- `<path>` — Directory to scan for duplicates

### Options

| Flag                 | Description                              |
| -------------------- | ---------------------------------------- |
| `--open`             | Preview one file per duplicate set       |
| `--interactive`      | Interactive mode to manually keep/delete |
| `--keep-newest`      | Auto-keep the newest file in each group  |
| `--keep-oldest`      | Auto-keep the oldest file in each group  |
| `--keep-shortest`    | Auto-keep the shortest path file         |
| `--keep-regex <PAT>` | Auto-keep file matching a regex pattern  |

> Only one `--keep-*` policy can be used at a time.

---

## 🖥️ Examples

```bash
# Basic scan
duplicate_finder ~/Downloads

# Smart deletion by newest modified file
duplicate_finder ~/Downloads --keep-newest

# Interactive cleanup
duplicate_finder ~/Downloads --interactive

# Keep files with 'important' in the name
duplicate_finder ~/Downloads --keep-regex "important"
```

---

## 🧪 Installation

```bash
git clone https://github.com/c0d3cr4ft3r/duplicate-finder.git
cd duplicate-finder
cargo build --release
./target/release/duplicate_finder --help
```

---

## 🧠 How it works

1. Scans all files and groups them by size
2. Computes BLAKE3 hashes in parallel
3. Groups files with identical hashes
4. Runs the chosen cleanup strategy

---

## 🧰 Dependencies

- `walkdir`
- `rayon`
- `blake3`
- `tabled`
- `clap`
- `dialoguer`
- `regex`

---

## 📝 License

MIT — do whatever you want, just don't blame me.
