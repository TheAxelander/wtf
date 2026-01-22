# CLAUDE.md

This file provides guidance to Claude Code when working with code in this repository.

## About wtf

`wtf` is a command line tool that displays cheatsheets, shortcuts, and other reference materials from a configured repository directly in the terminal.

The terminal output is primarily using the table formatting from `rich` module.

## Non Python Dependencies

The following tools need to be pre-installed on the system:
- `fzf`

## Project Structure

```
Source files
├── src/wtf/            # Main application logic and CLI handling
└── requirements.txt    # Development dependencies

Build files
└── pyproject.toml      # Build configuration for packaging

Documentation
├── README.md           # Main project overview and instructions
└── assets/             # Media assets used in README.md
```

## Code Style

Defined in `.editorconfig, main style rules are:
- 4-space indentation for Python, 2-space indentation for YAML, 120 char max line length
- UTF-8 encoding
- Unix-style line endings (LF)

## Config

The configuration file is expected at `~/.config/wtf/wtf.conf` and should be in INI format. Key configuration options include:
- `CHEATSHEET_REPO`: Path to the directory containing cheatsheet files
- `TABLE_DELIMITER`: Delimiter used in cheatsheet files for table formatting
- `PREVIEW_COMMAND`: Command used to preview files in fzf
