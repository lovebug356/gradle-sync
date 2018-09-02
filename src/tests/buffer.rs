use error::Error;
use semver::Version;
use buffer::GradleBuffer;
use buffer::parse_version_code_line;
use buffer::parse_version_name_line;
use buffer::replace_version_code;
use buffer::replace_version_name;

#[test]
fn test_parse_version_code_in_line() {
    let line = "versionCode 2";
    let version_code = parse_version_code_line(line).unwrap();
    assert_eq!(version_code, 2);

    let line = "    versionCode 1234";
    let version_code = parse_version_code_line(line).unwrap();
    assert_eq!(version_code, 1234);
}

#[test]
fn test_parse_version_code_not_found() {
    let line = "hello world";
    assert!(parse_version_code_line(line).is_none());

    let line = "versionCode abc";
    assert!(parse_version_code_line(line).is_none());
}

#[test]
fn test_parse_version_name_in_line() {
    let line = "versionName \"1.0.0\"";
    let version_name = parse_version_name_line(line).unwrap();
    assert_eq!(version_name.to_string(), "1.0.0");

    let line = "versionName \"3.4.123\"";
    let version_name = parse_version_name_line(line).unwrap();
    assert_eq!(version_name.to_string(), "3.4.123")
}

#[test]
fn test_parse_version_name_failed() {
    let line = "hello world";
    assert!(parse_version_name_line(line).is_none());

    let line = "versionName \"1.0\"";
    assert!(parse_version_name_line(line).is_none());

    let line = "versionCode 2";
    assert!(parse_version_name_line(line).is_none());
}

#[test]
fn test_read_version_from_gradle_file() {
    let file_content ="
    android {
        defaultConfig {
            versionCode 2
            versionName \"1.1.2\"
        }
    }".as_bytes();
    let expected_code = 2;
    let expected_version_name = Version::parse("1.1.2").unwrap();

    let file = GradleBuffer::from(file_content).unwrap();
    let version = file.version();
    assert_eq!(version.code(), expected_code);
    assert_eq!(version.version(), &expected_version_name);
}

#[test]
fn test_fail_if_gradle_file_does_not_contain_version_code () {
    let file_content ="
    android {
        defaultConfig {
            versionName \"1.1.2\"
        }
    }".as_bytes();

    let file = GradleBuffer::from(file_content);
    match file {
        Err(Error::VersionNotFound(_)) => {},
        _ => {unreachable!()}
    }
}

#[test]
fn test_fail_if_gradle_file_does_not_contain_version_name () {
    let file_content ="
    android {
        defaultConfig {
            versionCode 2
        }
    }".as_bytes();

    let file = GradleBuffer::from(file_content);
    match file {
        Err(Error::VersionNotFound(_)) => {},
        _ => {unreachable!()}
    }
}

#[test]
fn test_synchronize_version() {
    let file_content ="
    android {
        defaultConfig {
            versionCode 2
            versionName \"1.1.2\"
        }
    }".as_bytes();
    let new_version = Version::parse("1.2.0").unwrap();
    let expected_code = 3;

    let mut file = GradleBuffer::from(file_content).unwrap();
    file.synchronize_version(&new_version)
        .expect("failed to synchronize version");
    let version = file.version();
    assert_eq!(version.code(), expected_code);
    assert_eq!(version.version(), &new_version);
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
fn test_generate_synchronized_buffer() {
    let buffer_content = "
    android {
        defaultConfig {
            versionCode 2
            versionName \"1.1.2\"
        }
    }".as_bytes();
    let new_version = Version::parse("1.2.0").unwrap();
    let expected_buffer_content = "
    android {
        defaultConfig {
            versionCode 3
            versionName \"1.2.0\"
        }
    }\n";

    let mut buffer = GradleBuffer::from(buffer_content).unwrap();
    buffer.synchronize_version(&new_version)
        .expect("failed to syncrhonize version");
    let mut real_content: Vec<u8> = vec!();
    buffer.write(&mut real_content)
        .expect("failed to write content to writer");
    assert_eq!(&String::from_utf8(real_content).unwrap(), expected_buffer_content);
}
