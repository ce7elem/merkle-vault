use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

/// Reads lines from a file and returns them as a vector of strings.
///
/// # Arguments
///
/// * `filename` - A path-like object representing the name of the file to read.
///
/// # Returns
///
/// An `Result<Vec<String>>` containing a vector of strings, where each
/// string represents a line from the file. If an error occurs while reading
/// the file or collecting the lines, it is returned as an `io::Error`.
pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
