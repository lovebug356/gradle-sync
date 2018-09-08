use semver::Version;

use properties::parse_project_version_from_line;
use properties::replace_project_version_from_line;
use properties::PropertiesContent;

use configfile::ConfigurationFormat;

#[test]
fn should_parse_project_version_from_line() {
    let valid_version_lines = [
        "projectVersion=1.2.3",
        "projectVersion = 1.2.3",
        "   projectVersion = 1.2.3"
    ];
    valid_version_lines.iter().for_each(|line| {
        let version = parse_project_version_from_line(line).unwrap();
        assert_eq!(&version.to_string(), "1.2.3");
    });
}

#[test]
fn should_not_fail_when_parsing_invalid_lines() {
    let invalid_version_lines = [
        "project=1.2.3.4",
        "projectVersion=1.2",
        "hello world"
    ];
    invalid_version_lines.iter().for_each(|line| {
        let version = parse_project_version_from_line(line);
        assert!(version.is_none());
    })
}

#[test]
fn should_correctly_replace_version() {
    let version_lines = [
        ["projectVersion=1.2.3", "projectVersion=1.3.0"],
        ["projectVersion = 1.2.3", "projectVersion = 1.3.0"],
        ["  projectVersion = 1.2.3", "  projectVersion = 1.3.0"],
    ];
    let new_version = Version::parse("1.3.0").unwrap();

    version_lines.iter().for_each(|test| {
        let input_line = test[0];
        let expected_output_line = test[1];
        let real_output_line = replace_project_version_from_line(input_line.to_string(), &new_version);
        assert_eq!(expected_output_line, real_output_line);
    });
}

#[test]
fn should_read_version_from_reader() {
    let file_content = "
    projectVersion=1.2.0
    ".as_bytes();
    let expected_version = Version::parse("1.2.0").unwrap();

    let content =  <PropertiesContent as ConfigurationFormat>::from(file_content);
    assert!(content.is_ok());
    let content = content.unwrap();
    assert_eq!(
        content.current_version().unwrap().version().to_string(),
        expected_version.to_string()
    );
}

#[test]
fn should_fail_if_project_version_not_found() {
    let file_content = "
    hello world=12.3.4
    lsadkfjasdlfj
    ".as_bytes();

    let content =  <PropertiesContent as ConfigurationFormat>::from(file_content);
    assert!(content.is_err());
}

#[test]
fn should_write_new_version_to_writer() {
    let file_content = "
    projectVersion=1.2.0
    ".as_bytes();
    let new_version = Version::parse("1.3.4").unwrap();
    let expected_file_content = "
    projectVersion=1.3.4
    \n";

    let mut content = <PropertiesContent as ConfigurationFormat>::from(file_content).unwrap();
    content.sync_version(&new_version)
        .expect("failed to syncrhonize version");
    let mut real_content: Vec<u8> = vec!();
    content.write(&mut real_content)
        .expect("failed to write content to writer");
    assert_eq!(&String::from_utf8(real_content).unwrap(), expected_file_content);
}
