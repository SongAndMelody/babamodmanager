use std::{fs, io, path::Path};

use serde_json::Value;

/// Writes a structure into a file.
pub trait WriteInto {
    /// The desired file name for the structure.
    /// You can use [`WriteInto::write_into_as_is`] if you want to
    /// ignore this constant.
    const FILE_NAME: &str;
    /// Returns the inner contents of the desired file.
    fn as_file(&self) -> String;

    /// Writes the contents of the structure into a file, using the [WriteInto::FILE_NAME] as a suffixed name.
    fn write_into(&self, path: &Path) -> Result<(), io::Error> {
        self.write_into_using(path, Self::FILE_NAME)
    }

    /// Writes the contents of the structure into a file, ignoring the desired filename.
    fn write_into_as_is(&self, path: &Path) -> Result<(), io::Error> {
        let this_file = self.as_file();
        fs::write(path, this_file)
    }

    /// Writes the contents of the structure into a file, using a custom filename.
    fn write_into_using(&self, path: &Path, name: &str) -> Result<(), io::Error> {
        let this_file = self.as_file();
        let mut pathbuf = path.to_owned();
        pathbuf.push(name);
        fs::write(pathbuf, this_file)
    }
}

impl WriteInto for String {
    const FILE_NAME: &str = "file.txt";

    fn as_file(&self) -> String {
        self.clone()
    }
}

impl<'a> WriteInto for &'a str {
    const FILE_NAME: &'static str = "file.txt";

    fn as_file(&self) -> String {
        self.to_owned().to_owned()
    }
}

impl WriteInto for Value {
    const FILE_NAME: &str = "file.json";

    fn as_file(&self) -> String {
        self.to_string()
    }
}
