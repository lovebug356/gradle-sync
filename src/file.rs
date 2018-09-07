use error::GradleResult;
use error::Error;
use buffer::GradleBuffer;
use std::fs::File;
use std::env;
use std::fs::OpenOptions;
use version::sem_version_parse;

pub struct GradleFile {
    filename: String,
    buffer: GradleBuffer
}

impl GradleFile {
    pub fn new(filename: &str) -> GradleResult<Self> {
        let fd = File::open(filename)
            .map_err(|_err| {
                let reason = format!("failed to read file: {}", filename);
                Error::IoError(reason)
            })?;
        let buffer = GradleBuffer::from(fd)?;
        Ok(Self {
            filename: filename.to_string(),
            buffer
        })
    }

    pub fn new_and_sync_with_cargo(filename: &str) -> GradleResult<()> {
        let pkg_version = env::var("CARGO_PKG_VERSION").unwrap();
        let mut gradle_file = GradleFile::new(filename)?;
        gradle_file.synchronize_version(&pkg_version)?;
        if gradle_file.buffer.is_modified() {
            gradle_file.write()?;
        }
        Ok(())
    }

    pub fn synchronize_version(&mut self, version_name: &str) -> GradleResult<()> {
        let version = sem_version_parse(version_name)?;
        self.buffer.synchronize_version(&version)?;
        Ok(())
    }

    pub fn write(&self) -> GradleResult<()> {
        let mut fd = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.filename).map_err(|_err| {
                let reason = format!("failed to open file for reading '{}'", self.filename);
                Error::IoError(reason)
            })?;
        self.buffer.write(&mut fd)
    }
}
