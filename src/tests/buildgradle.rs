use semver::Version;

use buildgradle::parse_version_code_line;
use buildgradle::parse_version_name_line;
use buildgradle::replace_version_code;
use buildgradle::replace_version_name;
use buildgradle::BuildGradleContent;
use configfile::ConfigurationFormat;
use error::Error;

#[test]
fn parse_version_code_in_line() {
    let line = "versionCode 2";
    let version_code = parse_version_code_line(line).unwrap();
    assert_eq!(version_code, 2);

    let line = "    versionCode 1234";
    let version_code = parse_version_code_line(line).unwrap();
    assert_eq!(version_code, 1234);
}

#[test]
fn parse_version_code_not_found() {
    let line = "hello world";
    assert!(parse_version_code_line(line).is_none());

    let line = "versionCode abc";
    assert!(parse_version_code_line(line).is_none());
}

#[test]
fn parse_version_name_in_line() {
    let line = "versionName \"1.0.0\"";
    let version_name = parse_version_name_line(line).unwrap();
    assert_eq!(version_name.to_string(), "1.0.0");

    let line = "versionName \"3.4.123\"";
    let version_name = parse_version_name_line(line).unwrap();
    assert_eq!(version_name.to_string(), "3.4.123")
}

#[test]
fn parse_version_name_failed() {
    let line = "hello world";
    assert!(parse_version_name_line(line).is_none());

    let line = "versionName \"1.0\"";
    assert!(parse_version_name_line(line).is_none());

    let line = "versionCode 2";
    assert!(parse_version_name_line(line).is_none());
}

#[test]
fn test_replace_version_code() {
    let line = "versionCode 2";
    let exp_line = "versionCode 3";
    let real_line = replace_version_code(line.to_string(), 3);
    assert_eq!(real_line, exp_line);

    let line = "    versionCode 12";
    let exp_line = "    versionCode 78";
    let real_line = replace_version_code(line.to_string(), 78);
    assert_eq!(real_line, exp_line);

    let line = "hello world";
    let exp_line = "hello world";
    let real_line = replace_version_code(line.to_string(), 78);
    assert_eq!(real_line, exp_line);
}

#[test]
fn test_replace_version_name() {
    let line = "versionName \"1.0.0\"";
    let exp_line = "versionName \"1.2.3\"";
    let real_line = replace_version_name(
        line.to_string(),
        &Version::parse("1.2.3").unwrap()
    );
    assert_eq!(real_line, exp_line);

    let line = "versionName \"123.456.789\"";
    let exp_line = "versionName \"1.2.3\"";
    let real_line = replace_version_name(
        line.to_string(),
        &Version::parse("1.2.3").unwrap()
    );
    assert_eq!(real_line, exp_line);

    let line = "hello world";
    let exp_line = "hello world";
    let real_line = replace_version_name(
        line.to_string(),
        &Version::parse("1.2.3").unwrap()
    );
    assert_eq!(real_line, exp_line);
}

#[test]
fn read_gradle_version_from_file_content() {
    let file_content ="
    android {
        defaultConfig {
            versionCode 2
            versionName \"1.1.2\"
        }
    }".as_bytes();
    let expected_code = 2;
    let expected_version_name = Version::parse("1.1.2").unwrap();

    let content = <BuildGradleContent as ConfigurationFormat>::from(file_content).unwrap();
    assert_eq!(content.is_modified(), false);
    let version = content.current_version().unwrap();
    assert_eq!(version.code(), expected_code);
    assert_eq!(version.version(), &expected_version_name);
}

#[test]
fn fail_if_gradle_file_does_not_contain_version_code () {
    let file_content ="
    android {
        defaultConfig {
            versionName \"1.1.2\"
        }
    }".as_bytes();

    let content = <BuildGradleContent as ConfigurationFormat>::from(file_content);
    assert_eq!(content.err().unwrap(), Error::VersionNotFound(
            "failed to find versionCode".to_string()));
}

#[test]
fn fail_if_gradle_file_does_not_contain_version_name () {
    let file_content ="
    android {
        defaultConfig {
            versionCode 2
        }
    }".as_bytes();

    let content = <BuildGradleContent as ConfigurationFormat>::from(file_content);
    assert_eq!(content.err().unwrap(), Error::VersionNotFound(
            "failed to find versionName".to_string()));
}

#[test]
fn content_should_only_be_modified_when_synchronized_with_higher_version_number() {
    let file_content ="
    android {
        defaultConfig {
            versionCode 2
            versionName \"1.1.2\"
        }
    }".as_bytes();
    let same_version = Version::parse("1.1.2").unwrap();
    let higher_version = Version::parse("1.2.0").unwrap();

    let mut content = <BuildGradleContent as ConfigurationFormat>::from(file_content).unwrap();
    content.sync_version(&same_version)
        .expect("failed to synchronize version");
    assert_eq!(content.is_modified(), false);

    content.sync_version(&higher_version)
        .expect("failed to synchronize version");
    assert_eq!(content.is_modified(), true);
}

#[test]
fn should_write_new_version_to_writer() {
    let file_content = "
    android {
        defaultConfig {
            versionCode 2
            versionName \"1.1.2\"
        }
    }".as_bytes();
    let new_version = Version::parse("1.2.0").unwrap();
    let expected_file_content = "
    android {
        defaultConfig {
            versionCode 3
            versionName \"1.2.0\"
        }
    }\n";

    let mut content = <BuildGradleContent as ConfigurationFormat>::from(file_content).unwrap();
    content.sync_version(&new_version)
        .expect("failed to syncrhonize version");
    let mut real_content: Vec<u8> = vec!();
    content.write(&mut real_content)
        .expect("failed to write content to writer");
    assert_eq!(&String::from_utf8(real_content).unwrap(), expected_file_content);
}
