use clap::Parser;
use std::env;
use std::process;

use release_notifier_rust::*;

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
        let key = "SLACK_WEBHOOK_URL";
        let slack_webhook_get_result = env::var(key);

        let slack_webhook_url = match slack_webhook_get_result {
            Ok(value) => value,
            Err(error) => {
                eprintln!("[ERROR] error accessing environment variable '{key}': '{error}'.");
                process::exit(1);
            }
        };

        let send_result = send_message_via_slack_webhook(&message, &slack_webhook_url);

        match send_result {
            Ok(()) => {
                println!("[INFO] sucessfully sent to Slack.");
            }
            Err(error) => {
                eprintln!("[ERROR] error sending to Slack: '{error}'.");
                process::exit(2);
            }
        };
    }
}
