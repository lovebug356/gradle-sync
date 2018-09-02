use error::GradleResult;
use error::Error;
use version::GradleVersion;
use std::io::Read;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;
use regex::Captures;
use semver::Version;

pub struct GradleBuffer {
    lines: Vec<String>,
    version: GradleVersion
}

impl GradleBuffer {
    pub fn from<R: Read>(reader: R) -> GradleResult<Self> {
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
        Ok(GradleBuffer{
            lines,
            version: GradleVersion::new(
                         version_code.unwrap(),
                         version_name.unwrap()
                         )
        })
    }

    #[cfg(test)]
    pub fn version(&self) -> &GradleVersion {
        &self.version
    }

    pub fn synchronize_version(&mut self, new_version: &Version) -> GradleResult<()> {
        self.version.synchronize_version(&new_version)?;
        Ok(())
    }

    pub fn write<W: Write> (&self, writer: &mut W) -> GradleResult<()> {
        for line in self.lines.iter() {
            let mut line = line.clone();
            line = replace_version_code(line, self.version.code());
            line = replace_version_name(line, self.version.version());
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

