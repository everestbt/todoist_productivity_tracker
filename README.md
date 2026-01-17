# Todoist Productivity Tracker 

A CLI tool to help track todoist productivity and move tasks around using automated commands.

This program uses your own API key (rather than OAuth) to simplify the structure.
Your API key is passed in as an environment variable: `TODOIST_API_KEY`.
It also persists data for certain functions between runs. It does this is in the operating systems standard data directory. 

## Installation

The suggested approach is after cloning the repo to use:

```bash
cargo install --path .
```

## Usage

Once installed then you can use:

```bash
todoist-tracker --help
```

to list all commands.
