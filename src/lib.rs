use simple_error::SimpleError;
use std::error::Error;

pub fn get_changelog_content(changelog: &String) -> String {
    use std::fs;

    let changelog_content_string =
        fs::read_to_string(changelog).expect("Error reading the changelog file");

    return changelog_content_string;
}

pub fn get_latest_changelog_entry(changelog_content: &str) -> Result<&str, SimpleError> {
    use regex::Regex;

    // Compile regular expression:
    // This regex is just the changelog "header"
    // headers are expected to looke like this: `## [0.0.1] - 2022-06-15`
    let re = Regex::new(r"## \[\d+\.\d+\.\d+\] - \d{4}-\d{2}-\d{2}").unwrap();

    // This vector will store the offsets of the start of each regex match:
    let mut match_start_offsets = Vec::new();
    // Populate the vector:
    for cap in re.find_iter(&changelog_content) {
        match_start_offsets.push(cap.start())
    }

    if match_start_offsets.len() > 1 {
        // Handling 2 or more changelog "headers":
        let first_header = match_start_offsets[0];
        let second_header = match_start_offsets[1];
        let latest_changelog_entry = &changelog_content[first_header..second_header];
        return Ok(latest_changelog_entry);
    } else if match_start_offsets.len() == 1 {
        // Handling 1 changelog "header":
        let first_header = match_start_offsets[0];
        let latest_changelog_entry = &changelog_content[first_header..];
        return Ok(latest_changelog_entry);
    } else {
        return Err(SimpleError::new(
            "No valid changelog headers found. Is the changelog in a supported format?",
        ));
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
