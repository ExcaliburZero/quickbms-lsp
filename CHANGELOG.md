# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Message header parsing.
- Parser support for ignoring block and line comments.
- Parser support for function definitions and function calls.
- Function go to definition.

### Changed
- Keyword documentation hover message type from markdown to plaintext.

## [0.0.1] - 2021-05-08
### Added
- Initial support for `initialize`, `textDocument/didOpen`, and `textDocument/hover`.