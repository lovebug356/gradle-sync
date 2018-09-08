use semver::Version;
use regex::{Regex, Captures};
use configfile::ConfigurationFormat;
use error::GradleResult;
use std::io::Read;
use std::io::{BufReader, BufRead};
use error::Error;
use version::GradleVersion;

pub struct BuildGradleContent {
    lines: Vec<String>,
    version: GradleVersion,
    modified: bool
}

impl ConfigurationFormat for BuildGradleContent {
    fn from<R: Read>(reader: R) -> GradleResult<Self>{
        let mut version_code: Option<u32> = None;
        let mut version_name: Option<Version> = None;

        let f = BufReader::new(reader);
        let mut lines: Vec<String> = vec!();
        for result_line in f.lines() {
            let line = result_line?;
            if version_code.is_none() {
                version_code = parse_version_code_line(&line);
            }
            if version_name.is_none() {
                version_name = parse_version_name_line(&line);
            }
            lines.push(line);
        }
        if version_code.is_none() {
            return Err(Error::VersionNotFound("failed to find versionCode".to_string()))
        }
        if version_name.is_none() {
            return Err(Error::VersionNotFound("failed to find versionName".to_string()))
        }
        Ok(Self{
            lines,
            version: GradleVersion::new(
                         version_code.unwrap(),
                         version_name.unwrap()
                         ),
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
            let line = replace_version_code(line.clone(), self.version.code());
            let line = replace_version_name(line, self.version.version());
            line
        }).collect()
    }
}

pub fn parse_version_code_line(line: &str) -> Option<u32> {
    let re = Regex::new(r"versionCode\s+(?P<code>\d+)").unwrap();
    let caps = re.captures(line);
    match caps {
        Some(caps) => {
            let code: u32 = (&caps["code"]).parse().unwrap();
            Some(code)
        },
        None => None
    }
}

pub fn parse_version_name_line(line: &str) -> Option<Version> {
    let re = Regex::new(r#"versionName\s+"(?P<name>[\d.]+)""#).unwrap();
    let caps = re.captures(line);
    match caps {
        Some(caps) => {
            match Version::parse(&caps["name"]) {
                Ok(version) => Some(version),
                Err(_) => None
            }
        },
        None => None
    }
}

pub fn replace_version_code(line: String, version_code: u32) -> String {
    let old_version_code = parse_version_code_line(&line);
    match old_version_code {
        Some(_) => {
            let re = Regex::new(r#"versionCode\s+\d+"#).unwrap();
            let new_line = re.replace(&line, |_: &Captures| {
                format!("versionCode {}", version_code)
            });
            new_line.to_string()
        },
        None => line
    }
}

pub fn replace_version_name(line: String, version: &Version) -> String {
    let old_version_name = parse_version_name_line(&line);
    match old_version_name {
        Some(_) => {
            let re = Regex::new(r#"versionName\s+"[\d.]+""#).unwrap();
            let new_line = re.replace(&line, |_: &Captures| {
                format!(r#"versionName "{}""#, version.to_string())
            });
            new_line.to_string()
        },
        None => line
    }
}
