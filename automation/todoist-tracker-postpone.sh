#!/bin/bash
TODOIST_API_KEY=$(op item get "<KEY_NAME>" --fields <FIELD_NAME> --reveal) /Users/USER/.cargo/bin/todoist-tracker --postpone-to-goal --update-goals -vv