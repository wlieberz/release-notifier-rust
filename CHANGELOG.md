# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2023-09-26
### Changed
- Dependencies switched to [Rustls](https://github.com/rustls/rustls).

- Container OSes updated:
    - alpine:3.16.0 -> alpine: 3.18.3
    - Debian slim-bullseye -> slim-bookworm

- Dependencies updated.

- Project updates:
    - Bump github actions checkout@v3 -> v4.
    - Drop Github actions-rs/toolchain (looks abandoned).
        - Moved rust fmt check and unit-tests into Dockerfiles.
    - Drop ekidd/rust-musl-builder:1.57.0 (not updated since March 2022).
        - Instead, Cross-compile to musl (Alpine) from Debian.

## [0.5.0] - 2022-11-09
### Changed
- `reqwest` call has been changed from blocking to async.

   The call to the `reqwest` library within the
   `send_message_via_slack_webhook()` function is is now using the asynchronous
   variant of the library. This paves the way for expected future enhancements.

   External behavior should be completely unchanged.

## [0.4.0] - 2022-09-05
- The SLACK_WEBHOOK_URL environment var is no longer required when using the
  `--no-send` option.

## [0.3.0] - 2022-09-05
- Added ability to parse changelog headers which include the prefix "v"
  - i.e., this header is now handled correctly: `## [v0.3.0] - 2022-09-05`.
  - The "v" is case insensitive, so: `## [V0.3.0] - 2022-09-05` is also valid.

## [0.2.1] - 2022-09-04
- Added CI pipeline to project to run unit-tests.
- Added job to CI pipeline to fail if cargo fmt needed.
- Misc. non-functional code cleanup.
- Added job to pipeline to release dev versions of Docker images
- Added job to pipeline to release prod versions of Docker images

## [0.2.0] - 2022-07-17
- Split codebase into main and and library for easier maintenance.
- Added some unit tests.

## [0.1.1] - 2022-07-17
- Add Dockerfile.debian-slim
- Add Dockerfile.alpine

## [0.1.0] - 2022-07-16
- Initial dev release.
