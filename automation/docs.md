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
1. Replace in and `.plist` and `.sh` files `USER` with your username
1. Copy `.sh` files to `~Scripts/`
1. Make the shell scripts executable using `chmod 755`
1. Copy `.plist` files to `~/Library/LaunchAgents/`
1. Load each job doing: `launchctl load ~/Library/LaunchAgents/com.everest.todoist_productivity_tracker.plist` as an example

If any issues are encountered check `/tmp/todoist-tracker.err`.