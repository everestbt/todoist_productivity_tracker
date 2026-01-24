mod productivity_mode;

use api::{completed_fetch, filter_tasks, update_task, update_goals};
use db::{exclude_days, exclude_weeks};
use chrono::{Datelike, Days, Local, NaiveDate, NaiveDateTime, Weekday};
use clap::Parser;
use std::string::ToString;
use std::cmp;
use std::env;

// Command line arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Whether to return a status update
    #[arg(short, long)]
    status: bool,

    /// Whether to update the goals, must be used with the status flag OR postpone-to-goal flag. When used with status it will be based off daily target achieved over last week. When used with postpone-to-goal it will set a goal based off what is needed to reach the weekly goal (or 1 if already achieved), with a maximum of the daily average required to meet the weekly goal to avoid over subsribed days following breaks.
    #[arg(short, long)]
    update_goals: bool,

    /// Postpone tasks assigned to today to tomorrow
    #[arg(long)]
    postpone: bool,

    /// Postpone tasks assigned to today to tomorrow, leaving behind those with a specified time, any of higher priority, and then enough to meet the rolling weekly goal. Overdue tasks are also moved forward.
    #[arg(long)]
    postpone_to_goal: bool,

    /// Postpone all low priority tomorrow tasks by a number of days.
    #[arg(long)]
    postpone_by_days: Option<i8>,

    /// Bring overdue to today
    #[arg(short, long)]
    overdue: bool,

    /// A day you want to exclude from the daily goal calculation, in format YYYY-MM-DD
    #[arg(long)]
    exclude_day: Option<String>,

    /// Exclude the day shown as changing the target for daily goal calculation, must be used with status
    #[arg(long)]
    exclude_day_shown: bool,

    /// A week you want to exclude from the weekly goal calculation, should be date of Monday, in format YYYY-MM-DD
    #[arg(long)]
    exclude_week: Option<String>,

    /// Exclude the week shown as changing the target for weekly goal calculation, must be used with status
    #[arg(long)]
    exclude_week_shown: bool,

    /// Purge all the current saved data, useful to delete the saved api key and any excluded days/weeks
    #[arg(long)]
    purge: bool,

    /// Sets the verobosity of the logs to output
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbosity.into())
        .init();
    
    let key_var = env::var("TODOIST_API_KEY");
    if key_var.is_err() {
        panic!("You need to set the environment variable TODOIST_API_KEY with your API key")
    }
    let key = key_var.unwrap();

    let today:NaiveDate = Local::now().naive_local().date();

    if args.status {
        if args.update_goals && (args.exclude_day_shown || args.exclude_week_shown) {
            panic!("Cannot use --update-goals with either exclude shown commands");
        }

        let stats: completed_fetch::CompletedStats = completed_fetch::get_completed_stats(&key).await;

        // Floating week progress
        let sum_of_tasks: i32 = calculate_progress_on_floating_week(&stats);

        println!("Daily Progress: {done} / {goal}",
            done = stats.days_items.iter().find(|x| x.date == today.to_string()).unwrap().total_completed,
            goal = stats.goals.daily_goal);

        println!("Weekly Progress: {done} / {goal}",
            done = sum_of_tasks,
            goal = stats.goals.weekly_goal);

        // Check what mode you should be operating in
        let done_today = stats.days_items.iter().find(|x| x.date == today.format("%Y-%m-%d").to_string()).unwrap();
        let mode = productivity_mode::calculate_mode(sum_of_tasks, stats.goals.weekly_goal, stats.goals.daily_goal, done_today.total_completed);
        println!("Mode: {mode}!", mode = mode);

        // Load any days to exclude from daily goal calculation
        let days_result = exclude_days::get_excluded_days();
        if days_result.is_err() {
            panic!()
        }
        let days : Vec<String> = days_result.unwrap().iter().map(|d| d.format("%Y-%m-%d").to_string()).collect();

        // Check whether to change daily goal
        let min_daily_option = stats.days_items.iter()
                .filter(|x| x.date != today.format("%Y-%m-%d").to_string()) // Filter out today's date
                .filter(|x| !days.contains(&x.date)) // Filter out any excluded days 
                .min_by_key(|x| x.total_completed);
        if min_daily_option.is_none() {
            println!("All days excluded, just keep going!")
        }
        else {
            let min_daily = min_daily_option.unwrap();
            if min_daily.total_completed == stats.goals.daily_goal {
                println!("Daily goal is right!")
            }
            else {
                println!("New daily goal should be {new}, from {day}", new = min_daily.total_completed, day = min_daily.date);
                if args.update_goals {
                    update_goals::update_daily_goal(&key, &min_daily.total_completed).await;
                    println!("Updated daily goal to {new}", new = min_daily.total_completed);
                }
                if args.exclude_day_shown {
                    exclude_days::exclude_day(NaiveDate::parse_from_str(&min_daily.date, "%Y-%m-%d").expect("Date is in the wrong format")).expect("Failed to write excluded day");
                    println!("Excluded day {day}", day = min_daily.date)
                }
            }
        }

        // Calculate date of this week to filter out
        let this_week_start_day = today.week(Weekday::Mon).first_day();

        // Load any weeks to filter out from weekly goal calculation
        let weeks_result = exclude_weeks::get_excluded_weeks();
        if weeks_result.is_err() {
            panic!()
        }
        let weeks : Vec<String> = weeks_result.unwrap().iter().map(|d| d.format("%Y-%m-%d").to_string()).collect();

        // Check whether to increase weekly goal
        let min_weekly = stats.week_items.iter()
                .filter(|x| x.from != this_week_start_day.format("%Y-%m-%d").to_string()) // Filter out this week's day
                .filter(|x| !weeks.contains(&x.from)) // Filter out any excluded weeks 
                .min_by_key(|x| x.total_completed).unwrap();
        if min_weekly.total_completed == stats.goals.weekly_goal {
            println!("Weekly goal is right!")
        }
        else {
            println!("New weekly goal should be {new}, from {day}", new = min_weekly.total_completed, day = min_weekly.from);
            if args.update_goals {
                update_goals::update_weekly_goal(&key, &min_weekly.total_completed).await;
                println!("Updated weekly goal to {new}", new = min_weekly.total_completed);
            }
            if args.exclude_week_shown {
                exclude_weeks::exclude_week(NaiveDate::parse_from_str(&min_weekly.from, "%Y-%m-%d").expect("Date is in the wrong format")).expect("Failed to write excluded week");
                println!("Excluded week from {day}", day = min_weekly.from)
            }
        }
    }
    else if args.postpone {
        let todays_tasks = filter_tasks::get_todays_tasks(&key).await;
        println!("Found {} tasks to move to tomorrow", todays_tasks.len());
        for t in todays_tasks.iter() {
            postpone_task_to_tomorrow(&key, t).await;
        }
    }
    else if args.postpone_to_goal {
        // First reshedule all overdue tasks
        overdue(&key).await;
        // Get today tasks
        let todays_tasks: Vec<filter_tasks::Task> = filter_tasks::get_todays_tasks(&key).await;
        let total_today_tasks = todays_tasks.len() as i32;
        println!("Found {} tasks for today", total_today_tasks);
        // Check if any need to be rescheduled
        let stats: completed_fetch::CompletedStats = completed_fetch::get_completed_stats(&key).await;
        let sum_of_tasks: i32 = calculate_progress_on_floating_week(&stats);
        // Take remaining tasks for week or maximum 2* daily required to meet weekly goal to avoid over logging days
        let remaining_tasks_for_week = cmp::min(stats.goals.weekly_goal - sum_of_tasks, stats.goals.weekly_goal/7);
        if remaining_tasks_for_week >= total_today_tasks {
            println!("The number of tasks is below or equal to the number needed to complete your week so not rescheduling any");
        }
        else {
            // Filter out any tasks that have a higher priority + have a time to be done
            let filter_tasks: Vec<&filter_tasks::Task>  = todays_tasks.iter()
                .filter(|t| t.priority == 1)
                .filter(|t| t.duration.is_none())
                .collect();
            let low_priority_total = filter_tasks.len() as i32;
            // If no needed remaining tasks for the week then just move all filtered tasks OR if the remaining tasks is satisfied by the higher priority items
            if remaining_tasks_for_week <= 0 || remaining_tasks_for_week <= total_today_tasks - low_priority_total {
                println!("Rescheduling all lower priority tasks");
                for t in filter_tasks.iter() {
                    postpone_task_to_tomorrow(&key, t).await;
                }
            }
            else {
                // Calculate the max to reschedule and then take that number of first set of elements
                let max_to_reschedule: usize = (total_today_tasks - remaining_tasks_for_week) as usize;
                println!("Rescheduling at most {num} lower priority tasks", num = max_to_reschedule);
                for t in filter_tasks.iter().take(max_to_reschedule) {
                    postpone_task_to_tomorrow(&key, t).await;
                }
            }
        }
        if args.update_goals {
            // Add on the number already achieved today
            let today = stats.days_items.iter()
                .find(|x| x.date == today.format("%Y-%m-%d").to_string()).expect("Today should always exist"); // Find today's date
            let remaining_for_week_including_today = remaining_tasks_for_week + today.total_completed;
            if remaining_for_week_including_today <=0 {
                println!("At the target! Setting a goal of 1");
                update_goals::update_daily_goal(&key, &1).await;
            }
            else {
                println!("The number of tasks to aim for today is: {num}", num = remaining_for_week_including_today);
                update_goals::update_daily_goal(&key, &remaining_for_week_including_today).await;
            }
        }
    }
    else if args.postpone_by_days.is_some() {
        // Get all tasks due tomorrow
        let todays_tasks: Vec<filter_tasks::Task> = filter_tasks::get_tomorrow_tasks(&key).await;
        // Filter to low priority tasks
        let filter_tasks: Vec<&filter_tasks::Task>  = todays_tasks.iter()
                .filter(|t| t.priority == 1)
                .filter(|t| t.duration.is_none())
                .collect();
        for t in filter_tasks.iter() {
            postpone_task_by_days(&key, t, args.postpone_by_days.unwrap()).await;
        }
    }
    else if args.overdue {
        overdue(&key).await;
    }
    else if args.exclude_day.is_some() {
        let day = NaiveDate::parse_from_str(&args.exclude_day.unwrap().to_owned(), "%Y-%m-%d").unwrap();
        let result = exclude_days::exclude_day(day);
        if result.is_err() {
            panic!()
        }
        println!("Excluded day {day}", day = day)
    }
    else if args.exclude_week.is_some() {
        let day = NaiveDate::parse_from_str(&args.exclude_week.unwrap().to_owned(), "%Y-%m-%d").unwrap();
        // Check that the day is a Monday
        if day.weekday() != Weekday::Mon {
            println!("An excluded week date must be a Monday");
        }
        else {
            let result = exclude_weeks::exclude_week(day);
            if result.is_err() {
                panic!()
            }
            println!("Excluded week from {day}", day = day)
        }
    }
    else if args.purge {
        exclude_days::purge().expect("Failed to exclude days store");
        exclude_weeks::purge().expect("Failed to exclude weeks store");
    }

    Ok(())
}

fn parse_due_date_time(due : &String) -> NaiveDateTime {
    let due_date : NaiveDateTime = 
    if due.contains("Z") {
        NaiveDateTime::parse_from_str(&due.to_owned(), "%Y-%m-%dT%H:%M:%SZ").unwrap()
    }
    else {
        NaiveDateTime::parse_from_str(&due.to_owned(), "%Y-%m-%dT%H:%M:%S").unwrap()
    };
    due_date
}

fn calculate_progress_on_floating_week(stats: &completed_fetch::CompletedStats) -> i32 {
    stats.days_items.iter()
            .map(|x| x.total_completed)
            .sum()
}

async fn postpone_task_to_tomorrow(key: &str, t: &filter_tasks::Task) {
    postpone_task_by_days(key, t, 1).await;
}

async fn postpone_task_by_days(key: &str, t: &filter_tasks::Task, days: i8) {
    // If it contains a time then need to preserve that
    if t.due.date.contains("T") {
        let due_date_time : NaiveDateTime = parse_due_date_time(&t.due.date);
        let new_due_date = due_date_time.checked_add_days(Days::new(days as u64)).unwrap();
        update_task::update_task_due(key, &t.id, new_due_date.format("%Y-%m-%dT%H:%M:%S").to_string(), t.due.lang.to_owned(), t.due.string.to_owned()).await;
        println!("Rescheduled {content} to {due}", content = t.content, due = new_due_date)
    }
    // If it is only a date 
    else {
        let due_date = NaiveDate::parse_from_str(&t.due.date.to_owned(), "%Y-%m-%d").unwrap();
        let new_due_date = due_date.checked_add_days(Days::new(days as u64)).unwrap();
        update_task::update_task_due(key, &t.id, new_due_date.format("%Y-%m-%d").to_string(), t.due.lang.to_owned(), t.due.string.to_owned()).await;
        println!("Rescheduled {content} to {due}", content = t.content, due = new_due_date)
    }
}

async fn overdue(key: &str) {
    let today:NaiveDate = Local::now().naive_local().date();
    let overdue_tasks = filter_tasks::get_overdue_tasks(key).await;
    println!("Found {} tasks to move to today", overdue_tasks.len());
    for t in overdue_tasks.iter() {
        // Update the date to today
        // If it contains a time then need to preserve that
        if t.due.date.contains("T") {
            // Need to put the time on today
            let due_date_time = parse_due_date_time(&t.due.date);
            let today_with_time = today.and_time(due_date_time.time());
            update_task::update_task_due(key, &t.id, today_with_time.format("%Y-%m-%dT%H:%M:%S").to_string(), t.due.lang.to_owned(), t.due.string.to_owned()).await;
            println!("Rescheduled {content} to {due}", content = t.content, due = today_with_time)
        }
        // If it is only a date 
        else {
            update_task::update_task_due(key, &t.id, today.format("%Y-%m-%d").to_string(), t.due.lang.to_owned(), t.due.string.to_owned()).await;
            println!("Rescheduled {content} to today", content = t.content)
        }
    }
}
