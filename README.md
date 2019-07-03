# Rust tree command

A rust implementation of the linux command tree.

## Build
You can compile this project with Cargo. Simply run
```bash
cargo build
```
That will generate the binary file on target/debug/file_tree

## Usage
To display the file tree of the current directory, run:
```bash
file_tree
```

You can also pass one (or many) other path(s) to display the file tree from:
```bash
file_tree ~/ ./target
```

### Optional Arguments

- `-a` to include hidden directories
- `-d` to only display directories
- `-l` to specify a maximum level of depth
- `--help` to display help