The `com.everest.todoist_productivity_tracker.plist` runs the command:

```bash
todoist-tracker --postpone-to-goal --update-goals
```

every day at 3am. This will cause all tasks to be rescheduled for that day and set goals appropriately. 

`com.everest.todoist_tracker_weekly.plist` runs the command:

```bash
todoist-tracker --status --update-goals
```

every Monday at 1am. This will update the weekly goal automatically. It does this before the other command to allow the postpone command to run correctly.

To activate these follow these steps:
1. Replace in and `.plist` and `.sh` files all the `<WORD>` items in the file to ones for your system
1. Update the environment variable set to either be your key or be retrieved from a password manager, the existing ones use 1password.
1. Copy `.sh` files to `~Scripts/`
1. Make the shell scripts executable using `chmod 755`
1. Copy `.plist` files to `~/Library/LaunchAgents/`
1. Load each job doing: `launchctl load ~/Library/LaunchAgents/com.everest.todoist_productivity_tracker.plist` as an example

If any issues are encountered check `/tmp/todoist-tracker.err`.