use version::GradleVersion;
use semver::Version;
use error::Error;

#[test]
fn test_new_gradle_version() {
    let version_code = 1;
    let version_name = Version::parse("0.1.0").unwrap();

    let version = GradleVersion::new(version_code, version_name);
    assert_eq!(version.code(), version_code);
    assert_eq!(version.version().to_string(), "0.1.0");
}

#[test]
fn test_increase_version_code_on_version_name_bump() {
    let initial_code = 1;
    let old_version_name = Version::parse("0.1.0").unwrap();
    let new_version_name = Version::parse("0.1.1").unwrap();

    let mut version = GradleVersion::new(initial_code, old_version_name);
    let res = version.synchronize_version(&new_version_name);
    assert!(res.is_ok());
    let new_code = version.code();
    assert!(new_code > initial_code);
    assert_eq!(version.version(), &new_version_name);
}

#[test]
fn test_no_version_code_change_when_synchronized_with_same_version_name() {
    let initial_code = 1;
    let old_version_name = Version::parse("0.1.0").unwrap();

    let mut version = GradleVersion::new(initial_code, old_version_name.clone());
    let res = version.synchronize_version(&old_version_name);
    assert!(res.is_ok());
    let new_code = version.code();
    assert!(new_code == initial_code);
    assert_eq!(version.version(), &old_version_name);
}

#[test]
fn test_fail_to_decrease_version_name() {
    let initial_code = 1;
    let old_version_name = Version::parse("0.2.0").unwrap();
    let new_version_name = Version::parse("0.1.2").unwrap();

    let mut version = GradleVersion::new(initial_code, old_version_name);
    let res = version.synchronize_version(&new_version_name);
    assert!(res.is_err());
    let version_err = res.err().unwrap();
    assert_eq!(version_err, Error::VersionNotIncreasing(
            "version not increasing (old)0.2.0 > (new)0.1.2".to_string()));
}
