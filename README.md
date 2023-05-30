
# test-each

Generate tests at compile-time based on files and directories.

## Usage

This crate contains three attributes that all generate tests based on a file glob pattern. Each attribute generates tests with different argument types. The generated tests will be named after sanitized versions of the file names.

### Text files

Receive file contents as `&'static str` with `test_each::file`. This ignores any matched directories.

```rust
#[test_each::file("data/*.txt")]
fn test_file(content: &str) {
    // check contents
}
```

If data contains the files `foo.txt` and `bar.txt`, the following code will be generated:

```rust
#[test]
fn test_file_foo_txt_0() {
    test_file(include_str("data/foo.txt"))
}

#[test]
fn test_file_bar_txt_1() {
    test_file(include_str("data/bar.txt"))
}
```

### Binary files

Receive file contents as `&'static [u8]` with `test_each::blob`. This ignores any matched directories.

```rust
#[test_each::blob("data/*.bin")]
fn test_bytes(content: &[u8]) {
    // check contents
}
```

Declare a second parameter in order to additionally receive the path of file.

```rust
#[test_each::blob("data/*.bin")]
fn test_bytes(content: &[u8], path: PathBuf) {
    // check contents and path
}
```

### Paths to files and directories

Receive file path as `PathBuf` with `test_each::path`. This includes any matched directories.

```rust
#[test_each::path("data/*")]
fn test_bytes(path: PathBuf) {
    // check path
}
```

## Notes

Any change to an already included file will correctly trigger a recompilation, but creating a new file that matches the glob might not cause a recompilation.
To fix this issue add a build file that emits `cargo-rerun-if-changed={<glob directories>}`.

