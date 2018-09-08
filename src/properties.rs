use semver::Version;
use regex::Regex;
use configfile::ConfigurationFormat;
use std::io::{Read, Write};
use std::io::{BufRead, BufReader};
use error::GradleResult;
use error::Error;
use version::GradleVersion;

pub struct PropertiesContent {
    lines: Vec<String>,
    version: GradleVersion,
    modified: bool
}

impl ConfigurationFormat for PropertiesContent {
    fn from<R: Read>(reader: R) -> GradleResult<Self>{
        let mut project_version: Option<Version> = None;

        let f = BufReader::new(reader);
        let mut lines: Vec<String> = vec!();
        for result_line in f.lines() {
            let line = result_line?;
            if project_version.is_none() {
                project_version = parse_project_version_from_line(&line);
            }
            lines.push(line);
        }
        if project_version.is_none() {
            return Err(Error::VersionNotFound("failed to find projectVersion".to_string()))
        }
        Ok(Self{
            lines,
            version: GradleVersion::new(1, project_version.unwrap()),
            modified: false
        })
    }
    fn current_version(&self) -> GradleResult<&GradleVersion> {
        Ok(&self.version)
    }
    fn is_modified(&self) -> bool {
        self.modified
    }
    fn sync_version(&mut self, new_version: &Version) -> GradleResult<()> {
        if self.version.synchronize_version(new_version)? {
            self.modified = true
        }
        Ok(())
    }
    fn write<W: Write> (&self, writer: &mut W) -> GradleResult<()> {
        for line in self.lines.iter() {
            let mut line = line.clone();
            line = replace_project_version_from_line(line, self.version.version());
            writer.write(line.as_bytes())
                .map_err(|_err| {
                    Error::IoError("failed to write".to_string())
                })?;
            writer.write(b"\n")
                .map_err(|_err| {
                    Error::IoError("failed to write".to_string())
                })?;
        };
        Ok(())
    }
}

pub fn parse_project_version_from_line(line: &str) -> Option<Version> {
    let re = Regex::new(r"projectVersion\s*=\s*(?P<version>[\d.]+)").unwrap();
    let caps = re.captures(line);
    match caps {
        Some(caps) => {
            let version = Version::parse(&caps["version"]);
            match version {
                Ok(version) => Some(version),
                Err(_err) => None
            }
        },
        None => None
    }
}

pub fn replace_project_version_from_line(line: String, new_version: &Version) -> String {
    let version_found = parse_project_version_from_line(&line);
    match version_found {
        Some(old_version) => {
            line.replace(&old_version.to_string(), &new_version.to_string())
        },
        None => line
    }
}