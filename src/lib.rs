#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

#[doc(inline)]
/// Generate a series of tests that receive file contents as strings,
/// based on the result of a glob pattern.
///
/// This excludes any matched directories.
///
/// # Usage
/// ```rust
/// #[test_each::file("data/*.txt")]
/// fn test_file(content: &str) {
///     // test contents
/// }
/// ```
///
/// Add a second parameter of type `PathBuf` to receive the path of the file.
/// ```rust
/// #[test_each::file("data/*.txt")]
/// fn test_file(content: &str, path: PathBuf) {
///     // test contents
/// }
/// ```
pub use test_each_codegen::test_each_file as file;

#[doc(inline)]
/// Generate a series of tests that receive file contents as byte slices,
/// based on the result of a glob pattern.
///
/// This excludes any matched directories.
///
/// # Usage
/// ```rust
/// #[test_each::blob("data/*.bin")]
/// fn test_bytes(content: &[u8]) {
///     // test contents
/// }
/// ```
///
/// Add a second parameter of type `PathBuf` to receive the path of the file.
/// ```rust
/// #[test_each::blob("data/*.bin")]
/// fn test_bytes(content: &[u8], path: PathBuf) {
///     // test contents
/// }
/// ```
pub use test_each_codegen::test_each_blob as blob;

#[doc(inline)]
/// Generate a series of tests that receive file paths,
/// based on the result of a glob pattern.
///
/// This includes any matched directories.
///
/// # Usage
/// ```rust
/// #[test_each::path("data/*")]
/// fn test_paths(path: PathBuf) {
///     // test contents
/// }
/// ```
pub use test_each_codegen::test_each_path as path;
