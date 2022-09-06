# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2022-09-05
- Added ability to parse changelog headers which include the prefix "v"
  - i.e., this header is now handled correctly: `## [v0.3.0] - 2022-09-05`

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
