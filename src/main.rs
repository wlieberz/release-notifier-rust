use clap::Parser;
use simple_error::SimpleError;
use std::env;
use std::error::Error;

fn main() {
    // Configure command line options and parse them:

    /// Helper app to send notifications about new project releases.
    #[derive(Parser, Debug)]
    #[clap(author="William Lieberz", version, about, long_about = None)]
    struct Args {
        /// Path to changelog to parse. Defaults to CHANGELOG.md in the current directory.
        #[clap(
            short = 'c',
            long = "changelog",
            default_value = "CHANGELOG.md",
            value_parser
        )]
        changelog: String,

        /// Message which should appear before the parsed changelog entry (optional).
        #[clap(short = 'b', long = "before-message", default_value = "", value_parser)]
        before_message: String,

        /// Message which should appear after the parsed changelog entry (optional).
        #[clap(short = 'a', long = "after-message", default_value = "", value_parser)]
        after_message: String,

        /// Skip sending notification, just print message to console. Useful for debugging.
        #[clap(short = 'n', long = "no-send", action)]
        no_send: bool,
    }

    let args = Args::parse();

    // Get Slack webhook url from environment variable:
    let key = "SLACK_WEBHOOK_URL";
    let slack_webhook_url = env::var(key).expect("ERROR Getting $SLACK_WEBHOOK_URL env var");

    // Read changelog file and store in memory as string:
    let changelog_content_string = get_changelog_content(&args.changelog);

    // Get the latest changelog entry by parsing the changelog string:
    // Note: we are assuming the latest entry is at the top.
    let latest_changelog_entry = get_latest_changelog_entry(&changelog_content_string)
        .expect("Error parsing the latest changelog entry");

    // Format the message:
    let message = format!(
        "{}\n\n{}\n\n{}\n",
        args.before_message, latest_changelog_entry, args.after_message
    );

    // Handle no-send mode, vs regular mode:
    if args.no_send {
        print!("{}", &message);
    } else {
        send_message_via_slack_webhook(&message, &slack_webhook_url)
            .expect("Error sending message to Slack");
    }
}

fn get_changelog_content(changelog: &String) -> String {
    use std::fs;

    let changelog_content_string =
        fs::read_to_string(changelog).expect("Error reading the changelog file");

    return changelog_content_string;
}

fn get_latest_changelog_entry(changelog_content: &str) -> Result<&str, SimpleError> {
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

fn send_message_via_slack_webhook(
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
