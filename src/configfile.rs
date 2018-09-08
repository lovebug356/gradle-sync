use error::GradleResult;
use error::Error;
use std::io::{Read, Write};
use std::fs::OpenOptions;
use std::fs::File;
use std::env;
use semver::Version;
use version::GradleVersion;
use version::sem_version_parse;

pub struct GradleFile<T> {
    filename: String,
    content: T
}

impl<T> GradleFile<T> where T: ConfigurationFormat {
    pub fn new(filename: &str) -> GradleResult<GradleFile<T>> {
        let fd = File::open(filename.clone())
            .map_err(|_err| {
                let reason = format!("failed to read file: {}", filename);
                Error::IoError(reason)
            })?;
        Ok(Self {
            filename: filename.to_string(),
            content: T::from(fd)?
        })
    }

    pub fn sync_with_cargo(&mut self) -> GradleResult<()> {
        let pkg_version = env::var("CARGO_PKG_VERSION").unwrap();
        let pkg_version = sem_version_parse(&pkg_version)?;
        self.sync_version(&pkg_version)?;
        if self.content.is_modified() {
            self.write()?;
        }
        Ok(())
    }

    pub fn sync_version(&mut self, new_version: &Version) -> GradleResult<()> {
        self.content.sync_version(&new_version)
    }

    pub fn write(&self) -> GradleResult<()> {
        let mut fd = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.filename).map_err(|_err| {
                let reason = format!("failed to open file for reading '{}'", self.filename);
                Error::IoError(reason)
            })?;
        self.content.write(&mut fd)
    }
}

pub trait ConfigurationFormat where Self: Sized {
    fn from<R: Read>(reader: R) -> GradleResult<Self>;
    fn current_version(&self) -> GradleResult<&GradleVersion>;
    fn is_modified(&self) -> bool;
    fn sync_version(&mut self, new_version: &Version) -> GradleResult<()>;
    fn write<W: Write> (&self, writer: &mut W) -> GradleResult<()>;
}