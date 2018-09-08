extern crate semver;
extern crate regex;

#[cfg(test)]
mod tests;

mod version;
mod configfile;
pub use configfile::ConfigurationFormat;
pub use configfile::GradleFile;
mod buildgradle;
pub use buildgradle::BuildGradleContent;
mod properties;
pub use properties::PropertiesContent;

mod error;
pub use error::Error;

pub type BuildGradleFile = GradleFile<BuildGradleContent>;
pub type GradlePropertiesFile = GradleFile<PropertiesContent>;