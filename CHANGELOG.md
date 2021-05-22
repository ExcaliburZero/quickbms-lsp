# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Parser support for `goto` statements.
- Parser support for `continue` statements.
- Parser support for `break` statements.
- Parser support for `for` loops.
- Parser support for negative integer literals.

## [0.1.1] - 2021-05-21
### Added
- Parser support for `endian` statements.
- Parser support for `idstring` statements.
- Parser support for `if`, `elif`, and `else` statements.
- Document symbols list.

## [0.1.0] - 2021-05-16
### Added
- Message header parsing.
- Parser support for ignoring block and line comments.
- Parser support for function definitions and function calls.
- Parser support for hex literals.
- Parser support for variables expressions.
- Function go to definition.
- Function go to references. (#2, #3)
- Reparsing of document on changes. (#1, #8)

### Changed
- Keyword documentation hover message type from markdown to plaintext.
- Parser generator library from Antlr to tree-sitter. (#5, #6)

## [0.0.1] - 2021-05-08
### Added
- Initial support for `initialize`, `textDocument/didOpen`, and `textDocument/hover`.