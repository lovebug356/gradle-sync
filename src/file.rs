use error::GradleResult;
use error::Error;
use buffer::GradleBuffer;
use std::fs::File;
use std::fs::OpenOptions;
use semver::Version;

pub struct GradleFile {
    filename: String,
    buffer: GradleBuffer
}

impl GradleFile {
    pub fn new(filename: String) -> GradleResult<Self> {
        let fd = File::open(&filename)
            .map_err(|_err| {
                let reason = format!("failed to read file: {}", filename);
                Error::IoError(reason)
            })?;
        let buffer = GradleBuffer::from(fd)?;
        Ok(Self {
            filename, buffer
        })
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

fn sem_version_parse(version_string: &str) -> GradleResult<Version> {
    let version = Version::parse(version_string);
    match version {
        Err(_) => {
            let reason = format!("failed to parse version string '{}'", version_string);
            Err(Error::ParsingFailed(reason))
        },
        Ok(version) => Ok(version)
    }
}
