The `plist` runs the command:

```
todoist-tracker --postpone-to-goal --update-goals
```

every day at 3am. This will cause all tasks to be rescheduled for that day and set goals appropriately. 

To activate this follow these steps:
1. Replace in and `.plist` and `.sh` files `USER` with your username
1. Copy `automation/todoist-tracker-postpone.sh` to `~Scripts/todoist-tracker-postpone.sh`
1. Make the shell script executable using chmod 755
1. Copy `automation/com.everest.todoist_productivity_tracker.plist` to `~/Library/LaunchAgents/`
1. Load the job doing: `launchctl load ~/Library/LaunchAgents/com.everest.todoist_productivity_tracker.plist`

If any issues are encountered check `/tmp/todoist-tracker.err`.