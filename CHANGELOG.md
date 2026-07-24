---
# If you are adopting EasyBuild.ShipIt from an existing project, it is recommended to fill in the hash
# of the last commit released below
# last_commit_released: ...
#
# Learn more about configuration at
# https://github.com/easybuild-org/EasyBuild.ShipIt#configuration

# On each release, keep the version in extension.toml (the one Zed publishes) in sync
# with the version ShipIt computes from the conventional commits.
updaters:
  - regex:
      file: extension.toml
      pattern: '(?<=version = ")[^"]*(?=")'
---

# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This changelog is generated using [EasyBuild.ShipIt](https://github.com/easybuild-org/EasyBuild.ShipIt).

⚠ Only edit the front matter metadata at the top of this file. All other changes will be overwritten when a new release is created.

## 0.0.0
