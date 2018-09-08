use semver::Version;
use regex::Regex;
use configfile::ConfigurationFormat;
use std::io::Read;
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
    fn lines(&self) -> Vec<String> {
        self.lines.iter().map(|line|{
            replace_project_version_from_line(line.clone(), self.version.version())
        }).collect()
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