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

- `-a` or `--all` to include hidden directories
- `-d` or `--directory` to display directories only
- `-l` or `--max-level`to specify a maximum level of depth
- `-s` or `--sort` to specify a sorting policy (between `none`, `name`, `modified`)
- `-r` or `--reverse` to reverse sorting order (should be used with the sort option)
- `--dir-first` to display directories first
- `-f` or `--file-filter` to filter file names. Filters should either be a complete file name, a prefix (e.g `prefix*`),
 a suffix (e.g `*suffix`), or a prefix and suffix (e.g `prefix*suffix`)
- `--help` to display help