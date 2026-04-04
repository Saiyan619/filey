# FILEY 📁

A fast, lightweight CLI tool for managing files and folders directly from your terminal, no dragging, no clicking, no VSCode GUI stress.
Made with Love by ME - @saiyan619 

---

## Why FILEY?

Working inside VSCode and need to move, copy, delete, or inspect files? Instead of wrestling with the file explorer, just type a command. FILEY is built for developers who live in the terminal.

---

## Installation

### Prerequisites
- [Rust](https://rustup.rs/) must be installed on your machine.

### Build From Source

```bash
git clone https://github.com/yourusername/file-mgt.git
cd file-mgt
cargo build --release
```

Your binary will be at:
```
target/release/file-mgt        # Linux / macOS
target/release/file-mgt.exe    # Windows
```

---

## Commands

### Create a Folder
```bash
./file-mgt create <folder_name>
```
```bash
./file-mgt create my-project
# Folder created successfully
```

---

### Create a File
```bash
./file-mgt create-file <file_name>
```
```bash
./file-mgt create-file main.rs
# File created successfully
```

---

### Delete a File
```bash
./file-mgt delete-file <file_name>
```
```bash
./file-mgt delete-file old_notes.txt
# File deleted successfully
```

---

### Delete an Empty Folder
```bash
./file-mgt delete-folder <folder_name>
```
```bash
./file-mgt delete-folder empty-dir
# Folder deleted successfully
```

---

### Delete a Folder and Everything Inside It
> Use with caution. This is irreversible.

```bash
./file-mgt delete-folder-and-files <folder_name>
```
```bash
./file-mgt delete-folder-and-files old-project
# Folders and files deleted successfully
```

---

### Copy a File
```bash
./file-mgt copy-file <source> <destination>
```
```bash
./file-mgt copy-file src/main.rs backup/main.rs
# File copied
```

---

### Move a File
```bash
./file-mgt move-file <source> <destination>
```
```bash
./file-mgt move-file draft.txt final/draft.txt
# File moved successfully
```

---

### File Details
```bash
./file-mgt file-details <file_path>
```
```bash
./file-mgt file-details src/main.rs

# Created:       2026-03-29 16:30:19
# Last Modified: 2026-03-30 12:39:30
# Size:          4 KB
# Type:          File
```

---

### List Contents of a Folder
```bash
./file-mgt list <folder_path>
```
```bash
./file-mgt list ./src

# ./src\main.rs
# ./src\commands.rs
```

---

### Search for a File or Folder by Name
```bash
./file-mgt search-file <directory> <name>
```
```bash
./file-mgt search-file ./ main.rs

# found: main.rs, path - ./src\main.rs
```

---

## Help

```bash
./file-mgt --help
./file-mgt <command> --help
```

---

## Built With

- [Rust](https://www.rust-lang.org/)
- [clap](https://docs.rs/clap) — CLI argument parsing
- [walkdir](https://docs.rs/walkdir) — recursive directory walking
- [chrono](https://docs.rs/chrono) — human readable timestamps
- [figlet-rs](https://docs.rs/figlet-rs) — ASCII banner

---

## License

MIT
