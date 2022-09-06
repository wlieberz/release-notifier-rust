use simple_error::SimpleError;
use std::error::Error;

pub fn get_changelog_content(changelog: &String) -> String {
    use std::fs;

    fs::read_to_string(changelog).expect("Error reading the changelog file")
}

pub fn get_latest_changelog_entry(changelog_content: &str) -> Result<&str, SimpleError> {
    use regex::Regex;

    // Compile regular expression:
    // This regex is just the changelog "header"
    // headers are expected to looke like this: `## [0.1.2] - 2022-06-15`
    // or: `## [v0.1.2] - 2022-06-15`, or: `## [V0.1.2] - 2022-06-15`
    let re = Regex::new(r"## \[[vV]?\d+\.\d+\.\d+\] - \d{4}-\d{2}-\d{2}").unwrap();

    // This vector will store the offsets of the start of each regex match:
    let mut match_start_offsets = Vec::new();
    // Populate the vector:
    for cap in re.find_iter(changelog_content) {
        match_start_offsets.push(cap.start())
    }

    if match_start_offsets.len() > 1 {
        // Handling 2 or more changelog "headers":
        let first_header = match_start_offsets[0];
        let second_header = match_start_offsets[1];
        let latest_changelog_entry = &changelog_content[first_header..second_header];
        Ok(latest_changelog_entry)
    } else if match_start_offsets.len() == 1 {
        // Handling 1 changelog "header":
        let first_header = match_start_offsets[0];
        let latest_changelog_entry = &changelog_content[first_header..];
        Ok(latest_changelog_entry)
    } else {
        Err(SimpleError::new(
            "No valid changelog headers found. Is the changelog in a supported format?",
        ))
    }
}

pub fn send_message_via_slack_webhook(
    message: &str,
    slack_webhook_url: &str,
) -> Result<(), Box<dyn Error>> {
    use std::collections::HashMap;

    let mut payload_data = HashMap::new();
    payload_data.insert("text", &message);

    let client = reqwest::blocking::Client::new();

    client
        .post(slack_webhook_url)
        .json(&payload_data)
        .send()
        .expect("Error sending POST to Slack url");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_entry_changelog() {
        let changelog_content = "\
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [1.2.26] - 2022-06-24
- Many bugfixes
- Much more stable. Use this version. 

## [1.0.0] - 2022-06-16
- Major breaking changes.
- Much better now.
- Many new features.

## [0.1.0] - 2022-06-15
- Initial release.";

        let expected = "\
## [1.2.26] - 2022-06-24
- Many bugfixes
- Much more stable. Use this version. 

";
        let result = get_latest_changelog_entry(&changelog_content);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_single_entry_changelog() {
        let changelog_content = "\
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2022-06-25
- Initial release.

End of File";

        let expected = "\
## [0.1.0] - 2022-06-25
- Initial release.

End of File";
        let result = get_latest_changelog_entry(&changelog_content);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    #[should_panic]
    fn test_invalid_format_changelog() {
        let changelog_content = "\
# Changelog

This project will fail to parse this changelog since it doesn't contain 
a valid header for each version.

- 0.1.7:
    - Minor changes and bugfixes.

- 0.0.1:
    - Initial release.
";

        get_latest_changelog_entry(&changelog_content).unwrap();
    }

#[test]
fn test_changelog_with_v_lowercase() {
    let changelog_content = "\
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [v1.2.26] - 2022-06-24
- Many bugfixes
- Much more stable. Use this version.

## [v1.0.0] - 2022-06-16
- Major breaking changes.
- Much better now.
- Many new features.

## [v0.1.0] - 2022-06-15
- Initial release.";

    let expected = "\
## [v1.2.26] - 2022-06-24
- Many bugfixes
- Much more stable. Use this version.

";
    let result = get_latest_changelog_entry(&changelog_content);

    assert_eq!(expected, result.unwrap());
}

#[test]
fn test_changelog_with_v_uppercase() {
    let changelog_content = "\
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [V1.2.26] - 2022-06-24
- Many bugfixes
- Much more stable. Use this version.

## [V1.0.0] - 2022-06-16
- Major breaking changes.
- Much better now.
- Many new features.

## [V0.1.0] - 2022-06-15
- Initial release.";

    let expected = "\
## [V1.2.26] - 2022-06-24
- Many bugfixes
- Much more stable. Use this version.

";
    let result = get_latest_changelog_entry(&changelog_content);

    assert_eq!(expected, result.unwrap());
}

}
