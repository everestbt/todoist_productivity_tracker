mod completed_fetch;

use chrono::{Local, DateTime};
use clap::Parser;

// Command line arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Whether to return a status update
    #[arg(short, long)]
    status: bool,

    // The API key for Todoist
    #[arg(short, long)]
    key: String,

}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();

    let key: String = args.key;
    if args.status {
        let stats = completed_fetch::get_completed_stats(&key).await;

        // Floating week progress
        let sum_of_tasks: i32 = stats.days_items.iter()
            .map(|x| x.total_completed)
            .sum();

        println!("Progress: {done} / {goal}",
            done = sum_of_tasks,
            goal = stats.goals.weekly_goal);

        // Check what mode you should be operating in
        if sum_of_tasks < (stats.goals.weekly_goal - stats.goals.daily_goal) {
            println!("Mode: Chores!")
        }
        else {
            println!("Mode: Meaingful!")
        }

        // Get today's date
        let now: DateTime<Local> = Local::now();
    
        // Check whether to change daily goal
        let min_daily: i32 = stats.days_items.iter()
                .filter(|x| x.date != now.naive_local().date().to_string()) // Filter out today's date
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

    Ok(())
}
