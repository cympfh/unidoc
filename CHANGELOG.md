# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0] - 2026-05-13

### Added
- Footnote syntax: `[^label]` inline references and `[^label]: text` block definitions, rendered as `<section class="footnotes">` at the position where definitions appear
- Inline HyperLink (`[[url]]`) now shows favicon icon alongside the link text

### Fixed
- Lifetime elision warnings in `parser.rs` cleaned up

(git commit: 01232d4)

## [0.8.2] - 2026-05-13

### Added
- `--show-default-template` CLI flag to print the built-in Handlebars template

(git commit: cd4ac55)

## [0.8.1] - 2026-05-13

### Changed
- Refactored `parse_paragraph_line` into a dedicated function

(git commit: 17bdc11)

## [0.8.0]

### Added
- Initial public release with Markdown-to-HTML conversion
- Extended syntax: `@(path)` imports, `@[lang](path)` code imports, `[[url]]` inline hyperlinks, `{{url}}` blogcards
- Executor code blocks: `@bash`, `@dot`/`@graphviz`, `@gnuplot`
- MathJax support (`$...$` and `$$...$$`)
- Emoji shortcode support (`:shortcode:`)
- Handlebars templating with `--standalone`

(git commit: 3e3e5de)
