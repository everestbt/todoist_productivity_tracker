mod api;
mod db;
mod productivity_mode;

use api::{completed_fetch, filter_tasks, update_task, update_goals};
use db::{exclude_days, exclude_weeks, key_store};
use chrono::{Datelike, Days, Local, NaiveDate, NaiveDateTime, Weekday};
use clap::Parser;
use std::string::ToString;

// Command line arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The API key for Todoist, only needs to be used once, and can be used to replace saved key
    #[arg(short, long)]
    key: Option<String>,

    /// Whether to return a status update
    #[arg(short, long)]
    status: bool,

    /// Whether to update the goals, must be used with the status flag
    #[arg(short, long)]
    update_goals: bool,

    /// Postpone tasks assigned to today to tomorrow
    #[arg(short, long)]
    postpone: bool,

    /// Bring overdue to today
    #[arg(short, long)]
    overdue: bool,

    /// A day you want to exclude from the daily goal calculation, in format YYYY-MM-DD
    #[arg(short, long)]
    exclude_day: Option<String>,

    /// Exclude the day shown as changing the target for daily goal calculation, must be used with status
    #[arg(short, long)]
    exclude_day_shown: bool,

    /// A week you want to exclude from the weekly goal calculation, should be date of Monday, in format YYYY-MM-DD
    #[arg(short, long)]
    exclude_week: Option<String>,

    /// Exclude the week shown as changing the target for weekly goal calculation, must be used with status
    #[arg(short, long)]
    exclude_week_shown: bool,

    /// Purge all the current saved data, useful to delete the saved api key and any excluded days/weeks
    #[arg(short, long)]
    purge: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();

    let key: String;
    if args.key.is_some() {
        key = args.key.unwrap();
        key_store::save_key(&key).expect("Failed to save the key");
        println!("Saved your key, no need to use --key each time now. You can replace it by using --key again.")
    }
    else {
        key = key_store::get_key().expect("Failed to load a key, use --key first");
    }

    let today:NaiveDate = Local::now().naive_local().date();

    if args.status {
        if args.update_goals && args.exclude_day_shown || args.update_goals && args.exclude_week_shown {
            panic!("Cannot use --update-goals with either exclude shown commands");
        }

        let stats = completed_fetch::get_completed_stats(&key).await;

        // Floating week progress
        let sum_of_tasks: i32 = stats.days_items.iter()
            .map(|x| x.total_completed)
            .sum();

        println!("Daily Progress: {done} / {goal}",
            done = stats.days_items.iter().find(|x| x.date == today.to_string()).unwrap().total_completed,
            goal = stats.goals.daily_goal);

        println!("Weekly Progress: {done} / {goal}",
            done = sum_of_tasks,
            goal = stats.goals.weekly_goal);

        // Check what mode you should be operating in
        let done_today = stats.days_items.iter().find(|x| x.date == today.format("%Y-%m-%d").to_string()).unwrap();
        let mode = productivity_mode::calculate_mode(sum_of_tasks, stats.goals.weekly_goal, stats.goals.daily_goal, done_today.total_completed);
        println!("Mode: {mode}!", mode = mode.to_string());

        // Load any days to exclude from daily goal calculation
        let days_result = exclude_days::get_excluded_days();
        match days_result.is_err() {
            true => panic!(),
            false => (),
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
        match weeks_result.is_err() {
            true => panic!(),
            false => (),
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

    if args.postpone {
        let todays_tasks = filter_tasks::get_todays_tasks(&key).await;
        println!("Found {} tasks to move to tomorrow", todays_tasks.len());
        for t in todays_tasks.iter() {
            // Update the date to tomorrow
            // If it contains a time then need to preserve that
            if t.due.date.contains("T") {
                let due_date_time : NaiveDateTime = parse_due_date_time(&t.due.date);
                let tomorrow = due_date_time.checked_add_days(Days::new(1)).unwrap();
                update_task::update_task_due(&key, &t.id, tomorrow.format("%Y-%m-%dT%H:%M:%S").to_string(), t.due.lang.to_owned(), t.due.string.to_owned()).await;
                println!("Rescheduled {content} to {due}", content = t.content, due = tomorrow)
            }
            // If it is only a date 
            else {
                let due_date = NaiveDate::parse_from_str(&t.due.date.to_owned(), "%Y-%m-%d").unwrap();
                let tomorrow = due_date.checked_add_days(Days::new(1)).unwrap();
                update_task::update_task_due(&key, &t.id, tomorrow.format("%Y-%m-%d").to_string(), t.due.lang.to_owned(), t.due.string.to_owned()).await;
                println!("Rescheduled {content} to tomorrow", content = t.content)
            }
        }
    }

    if args.overdue {
        let overdue_tasks = filter_tasks::get_overdue_tasks(&key).await;
        println!("Found {} tasks to move to today", overdue_tasks.len());
        for t in overdue_tasks.iter() {
            // Update the date to today
            // If it contains a time then need to preserve that
            if t.due.date.contains("T") {
                // Need to put the time on today
                let due_date_time = parse_due_date_time(&t.due.date);
                let today_with_time = today.and_time(due_date_time.time());
                update_task::update_task_due(&key, &t.id, today_with_time.format("%Y-%m-%dT%H:%M:%S").to_string(), t.due.lang.to_owned(), t.due.string.to_owned()).await;
                println!("Rescheduled {content} to {due}", content = t.content, due = today_with_time)
            }
            // If it is only a date 
            else {
                update_task::update_task_due(&key, &t.id, today.format("%Y-%m-%d").to_string(), t.due.lang.to_owned(), t.due.string.to_owned()).await;
                println!("Rescheduled {content} to today", content = t.content)
            }
        }
    }

    if args.exclude_day.is_some() {
        let day = NaiveDate::parse_from_str(&args.exclude_day.unwrap().to_owned(), "%Y-%m-%d").unwrap();
        let result = exclude_days::exclude_day(day);
        match result.is_err() {
            true => panic!(),
            false => (),
        }
        println!("Excluded day {day}", day = day)
    }

    if args.exclude_week.is_some() {
        let day = NaiveDate::parse_from_str(&args.exclude_week.unwrap().to_owned(), "%Y-%m-%d").unwrap();
        // Check that the day is a Monday
        if day.weekday() != Weekday::Mon {
            println!("An excluded week date must be a Monday");
        }
        else {
            let result = exclude_weeks::exclude_week(day);
            match result.is_err() {
                true => panic!(),
                false => (),
            }
            println!("Excluded week from {day}", day = day)
        }
    }

    if args.purge {
        key_store::purge().expect("Failed to purge key store");
        exclude_days::purge().expect("Failed to exclude days store");
        exclude_weeks::purge().expect("Failed to exclude weeks store");
    }

    Ok(())
}

fn parse_due_date_time(due : &String) -> NaiveDateTime {
    let due_date : NaiveDateTime;
    if due.contains("Z") {
        due_date = NaiveDateTime::parse_from_str(&due.to_owned(), "%Y-%m-%dT%H:%M:%SZ").unwrap();
    }
    else {
        due_date = NaiveDateTime::parse_from_str(&due.to_owned(), "%Y-%m-%dT%H:%M:%S").unwrap();
    }
    due_date
}
