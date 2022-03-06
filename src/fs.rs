use anyhow::Result;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

// @see: https://github.com/rust-lang/mdBook/blob/2213312938/src/utils/fs.rs#L61-L72
// This function creates a file and returns it. But before creating the file
// it checks every directory in the path to see if it exists,
// and if it does not it will be created.
pub fn create_file(path: &Path) -> Result<File> {
    // Construct path
    if let Some(p) = path.parent() {
        fs::create_dir_all(p)?;
    }

    File::create(path).map_err(Into::into)
}

// @see: https://github.com/rust-lang/mdBook/blob/2213312938/src/utils/fs.rs#L16-L20
// Write the given data to a file, creating it first if necessary
pub fn write_file<P: AsRef<Path>>(build_dir: &Path, filename: P, content: &[u8]) -> Result<()> {
    let path = build_dir.join(filename);
    create_file(&path)?.write_all(content).map_err(Into::into)
}

// recursive copy folder
pub fn copy_dirs<P: AsRef<Path>>(source: P, target: P) -> std::io::Result<()> {
    fs::create_dir_all(&target)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let entry_target = target.as_ref().join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dirs(entry.path(), entry_target)?;
        } else {
            fs::copy(entry.path(), entry_target)?;
        }
    }
    Ok(())
}
