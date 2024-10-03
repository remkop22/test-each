
# test-each

Generate tests at compile-time based on files and directories.

## Usage

This crate contains three attributes that all generate tests based on a file glob pattern. Each attribute generates tests with different argument types. The generated tests will be named after sanitized versions of the file names.

### Text files

Receive file contents as [`&'static str`](std::str) with [`test_each::file`](crate::file). This ignores any matched directories.

```rust
#[test_each::file(glob = "data/*.txt")]
fn test_file(content: &str) {
    // check contents
}
```

If `data` contains the files `foo.txt` and `bar.txt`, the following code will be generated:

```rust
#[test]
fn test_file_foo() {
    test_file(include_str("data/foo.txt"))
}

#[test]
fn test_file_bar() {
    test_file(include_str("data/bar.txt"))
}
```

### Binary files

Receive file contents as [`&'static [u8]`](std::slice) with [`test_each::blob`](crate::file). This ignores any matched directories.

```rust
#[test_each::blob(glob = "data/*.bin")]
fn test_bytes(content: &[u8]) {
    // check contents
}
```

Declare a second parameter in order to additionally receive the path of file.

```rust
#[test_each::blob(glob = "data/*.bin")]
fn test_bytes(content: &[u8], path: &Path) {
    // check contents and path
}
```

### Paths to files and directories

Receive file path as a reference to [`Path`](std::path::Path) with [`test_each::path`](crate::path). This includes any matched directories.

```rust
#[test_each::path(glob = "data/*")]
fn test_bytes(path: &Path) {
    // check path
}
```

### Customizing the function name

By default the name of the generated test will consist of the escaped file name without extension. Use the `name` attribute to change how the function names are formatted. 

Use `name(segments = <n>)` to add `n` amount of path segments (from right to left) to the name.

Use `name(index)` to add a unique index to the end of the test name. This will prevent name collisions.

Use `name(extension)` to include the file extension the end of the test name.

```rust
/// The generated function name will be `test_file_bar_baz_data_txt_0`
#[test_each::file(glob = "foo/bar/baz/data.txt", name(segments = 3, index, extension))]
fn test_file(_: &str) {}
```

## Notes

Any change to an already included file will correctly trigger a recompilation, but creating a new file that matches the glob might not cause a recompilation.
To fix this issue add a build file that emits `cargo-rerun-if-changed={<glob directories>}`.

