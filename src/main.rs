mod completed_fetch;
mod filter_tasks;
mod update_task;

use chrono::{Days, Local, NaiveDateTime, NaiveDate};
use clap::Parser;

// Command line arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The API key for Todoist
    #[arg(short, long)]
    key: String,

    /// Whether to return a status update
    #[arg(short, long)]
    status: bool,

    /// Postpone tasks assigned to today to tomorrow
    #[arg(short, long)]
    postpone: bool,

    /// Bring overdue to today
    #[arg(short, long)]
    overdue: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();

    let key: String = args.key;

    let today:NaiveDate = Local::now().naive_local().date();

    if args.status {
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
        if sum_of_tasks < (stats.goals.weekly_goal - stats.goals.daily_goal) {
            println!("Mode: Chores!")
        }
        else {
            println!("Mode: Meaingful!")
        }
    
        // Check whether to change daily goal
        let min_daily: i32 = stats.days_items.iter()
                .filter(|x| x.date != today.to_string()) // Filter out today's date
                .map(|x| x.total_completed)
                .min().unwrap();
        if min_daily == stats.goals.daily_goal {
            println!("Daily goal is right!")
        }
        else {
            println!("New daily goal should be {new}", new = min_daily)
        }

        // Check whether to increase daily goal (we don't include decrease due to holiday)
        let min_weekly: i32 = stats.week_items.iter()
                .map(|x| x.total_completed)
                .min().unwrap();
        if min_weekly <= stats.goals.weekly_goal {
            println!("Weekly goal is right!")
        }
        else {
            println!("New weekly goal should be {new}", new = min_weekly)
        }
    }

    if args.postpone {
        let todays_tasks = filter_tasks::get_todays_tasks(&key).await;
        for t in todays_tasks.iter() {
            // Update the date to tomorrow
            // If it contains a time, do the following:
            if t.due.date.contains("T") {
                let due_date : NaiveDateTime;
                if t.due.date.contains("Z") {
                due_date = NaiveDateTime::parse_from_str(&t.due.date.to_owned(), "%Y-%m-%dT%H:%M:%SZ").unwrap();
                }
                else {
                    due_date = NaiveDateTime::parse_from_str(&t.due.date.to_owned(), "%Y-%m-%dT%H:%M:%S").unwrap();
                }
                let tomorrow = due_date.checked_add_days(Days::new(1)).unwrap();
                update_task::update_task_due(&key, &t.id, tomorrow.format("%Y-%m-%dT%H:%M:%S").to_string(), t.due.lang.to_owned(), t.due.string.to_owned()).await;
            }
            // If it is only a date 
            else {
                let due_date = NaiveDate::parse_from_str(&t.due.date.to_owned(), "%Y-%m-%d").unwrap();
                let tomorrow = due_date.checked_add_days(Days::new(1)).unwrap();
                update_task::update_task_due(&key, &t.id, tomorrow.format("%Y-%m-%d").to_string(), t.due.lang.to_owned(), t.due.string.to_owned()).await;
            }
            println!("Rescheduled {content} to tomorrow", content = t.content)
        }
    }

    if args.overdue {
        let todays_tasks = filter_tasks::get_overdue_tasks(&key).await;
        for t in todays_tasks.iter() {
            // Update the date to today
            // If it contains a time, do the following:
            if t.due.date.contains("T") {
                update_task::update_task_due(&key, &t.id, today.format("%Y-%m-%dT%H:%M:%S").to_string(), t.due.lang.to_owned(), t.due.string.to_owned()).await;
            }
            // If it is only a date 
            else {
                update_task::update_task_due(&key, &t.id, today.format("%Y-%m-%d").to_string(), t.due.lang.to_owned(), t.due.string.to_owned()).await;
            }
            println!("Rescheduled {content} to today", content = t.content)
        }
    }

    Ok(())
}
