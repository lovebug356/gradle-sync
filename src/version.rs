use semver::Version;
use error::{Error, GradleResult};

pub struct GradleVersion {
    version_code: u32,
    version_name: Version
}

impl GradleVersion {
    pub fn new(version_code: u32, version_name: Version) -> Self {
        Self {
            version_code, version_name
        }
    }
    pub fn code(&self) -> u32 {self.version_code}
    pub fn version(&self) -> &Version {&self.version_name}

    pub fn synchronize_version(&mut self, new_version: &Version) -> GradleResult<bool> {
        if &self.version_name > new_version {
            let reason = format!(
                "version not increasing (old){} > (new){}",
                self.version_name.to_string(),
                new_version.to_string()
            );
            return Err(Error::VersionNotIncreasing(reason))
        }
        if &self.version_name < new_version {
            self.version_code += 1;
            self.version_name = new_version.clone();
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

pub fn sem_version_parse(version_string: &str) -> GradleResult<Version> {
    let version = Version::parse(version_string);
    match version {
        Err(_) => {
            let reason = format!("failed to parse version string '{}'", version_string);
            Err(Error::ParsingFailed(reason))
        },
        Ok(version) => Ok(version)
    }
}
