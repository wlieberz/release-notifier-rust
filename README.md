# release-notifier-rust

# About

This is a a toy project for me to practice Rust development.

This repo produces a Rust binary called: `release-notifier-rust` which can
be used in CI/CD pipelines to post a notification to a Slack channel when
a new release is available. It requires a Slack webhook to send to.

It parses the latest release notes from the CHANGELOG.md of your project 
and posts it to your Slack channel. 

Additionally, it can post optional arbitrary text before and after the 
latest release notes.

# Example Usage

**Help / usage:**

```bash
USAGE:
    release-notifier-rust [OPTIONS]

OPTIONS:
    -a, --after-message <AFTER_MESSAGE>
            Message which should appear after the parsed changelog entry (optional) [default: ]

    -b, --before-message <BEFORE_MESSAGE>
            Message which should appear before the parsed changelog entry (optional) [default: ]

    -c, --changelog <CHANGELOG>
            Path to changelog to parse. Defaults to CHANGELOG.md in the current directory [default:
            CHANGELOG.md]

    -h, --help
            Print help information

    -n, --no-send
            Skip sending notification, just print message to console. Useful for debugging

    -V, --version
            Print version information
```

**Send to Slack:**

```bash
# Slack webhook environment variable is required:
export SLACK_WEBHOOK_URL=http://127.0.0.1:3000/

./release-notifier-rust \
  --changelog ./test-files/single-entry_changelog.md \
  --before-message "$(cat test-files/before-message_sample.txt)" \
  --after-message "$(cat test-files/after-message_sample.txt)"

# Note: produces no output on the console but should exit 0 and you should see
# the notification posted to your Slack channel, provided that the environment
# variable for SLACK_WEBHOOK_URL is correct.
```

**Don't send to Slack, only print to console:**

```bash
./release-notifier-rust \
  --changelog ./test-files/single-entry_changelog.md \
  --before-message "$(cat test-files/before-message_sample.txt)" \
  --after-message "$(cat test-files/after-message_sample.txt)" \
  --no-send

# Output:
Announcement - There is a new release available of the: Example Project

Latest changelog entry, below:

## [0.1.0] - 2022-06-25
- Initial release.

End of File

Project URL: your-url-here

Did you find a problem with this automated notification?
Please don't hesitate to reach out!
```

## Docker images

For convenience, Debian-slim and and Alpine based Dockerfiles are provided.

I strongly reccomend using the Debian-slim version over Alpine, due to 
potential issues with Rust compilation against musl-libc libraries.  That being
said, the Alpine image should probably work fine. If you run into issues
with the Alpine version, try the Debian version.

## Testing sending a notification locally

Note: if you just want to preview the notification locally, and you don't care 
about actually sending a POST request, you can use the `--no-send` flag
to just print the result to the console. Formatting may differ slightly on
the console vs in an actual Slack message. 

For testing sending a notification to Slack, it is ideal to have a testing 
channel with a webhook that you can use. Nothing beats testing with the real
thing.

In case you don't have access to create a Slack testing channel and webhook,
and in case you don't want to spam your actual Slack channel with messages 
during development/testing I will describe an alternative option, below.

You can run `http-echo-server` which will dump any requests it 
receives to your terminal for inspection. You should be able to get some sense
of whether the request that was recieved by the echo-server looks reasonable.

### Install and start http-echo-server

```bash
# Instructions tested on Ubuntu 22.04
# without nvm or nodejs installed.

# You will be installing node packages into a directory
# change this path as you desire:
mkdir -p ~/projects/testing/http-echo-server
cd ~/projects/testing/http-echo-server

# Install nvm:
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash

# Load nvm without opening a new shell:
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"

# Install latest LTS version of node:
nvm install --lts

# Install the echo server:
npm install http-echo-server

# Run the echo server:
# it listens on port 3000 by default
node node_modules/http-echo-server/
```

### Send notification with release-notifier-rust to http-echo-server

```bash
# Slack webhook environment variable is required:
export SLACK_WEBHOOK_URL=http://127.0.0.1:3000/

./release-notifier-rust \
  --changelog ./test-files/single-entry_changelog.md \
  --before-message "$(cat test-files/before-message_sample.txt)" \
  --after-message "$(cat test-files/after-message_sample.txt)"
```

A reasonable request might look something like this:

```bash
[server] event: listening (port: 3000)
[server] event: connection (socket#1)
[socket#1] event: resume
[socket#1] event: data
--> POST / HTTP/1.1
--> content-type: application/json
--> content-length: 307
--> accept: */*
--> host: 127.0.0.1:3000
--> 
--> {"text":"Announcement - There is a new release available of the: Example Project\n\nLatest changelog entry, below:\n\n## [0.1.0] - 2022-06-25\n- Initial release.\n\nEnd of File\n\nProject URL: your-url-here\n\nDid you find a problem with this automated notification?\nPlease don't hesitate to reach out!\n"}
[socket#1] event: error (msg: read ECONNRESET)
[socket#1] event: close

```

Note: despite the error `event: error (msg: read ECONNRESET)` this type of 
response, as seen by the echo-server actually works OK when sending to a real
Slack channel. I'm not sure what this error is about - probably something silly
I'm doing wrong, but it isn't a show-stopper.