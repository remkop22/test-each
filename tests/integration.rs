use std::{io::BufRead, path::PathBuf};

#[test_each::file(glob = "tests/data/*.txt", name(index))]
fn test_file(content: &str) {
    assert_eq!(Some("hello world"), content.lines().next())
}

#[test_each::file(glob = "tests/data/*.txt", name(segments = 2, extension))]
fn test_file_with_path(content: &str, path: PathBuf) {
    let mut lines = content.lines();
    assert_eq!(Some("hello world"), lines.next());
    assert_eq!(path.file_name().and_then(|s| s.to_str()), lines.next());
}

#[test_each::blob(glob = "tests/data/*.txt", name(segments = 3, extension, index))]
fn test_blob(content: &[u8]) {
    assert_eq!(
        Some(b"hello world".to_vec()),
        BufRead::split(content, b'\n').next().transpose().unwrap()
    )
}

#[test_each::path(glob = "tests/data/*.txt")]
fn test_path(path: PathBuf) {
    match path.file_name().and_then(|s| s.to_str()) {
        Some("foo.txt" | "bar.txt") => {}
        other => panic!("expected either `foo.txt` or `bar.txt` found: {:?}", other),
    }
}
