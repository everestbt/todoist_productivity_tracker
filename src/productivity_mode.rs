use strum::Display;

#[derive(Display, Debug)]
pub enum ProductivityMode {
    Chores,
    Meaningful
}

pub fn calculate_mode(sum_of_tasks: i32, weekly_goal: i32, daily_goal: i32, done_today: i32) -> ProductivityMode {
    if sum_of_tasks >= weekly_goal {
        ProductivityMode::Meaningful
    }
    else if (sum_of_tasks-done_today) < (weekly_goal - daily_goal) {
        ProductivityMode::Chores
    }
    else {
        ProductivityMode::Meaningful
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sum_of_tasks_below_weekly_and_daily() {
        assert!(matches!(calculate_mode(10, 20, 5, 0), ProductivityMode::Chores));
    }

    #[test]
    fn test_sum_of_tasks_above_weekly_and_daily() {
        assert!(matches!(calculate_mode(25, 20, 5, 0), ProductivityMode::Meaningful));
    }

    #[test]
    fn test_sum_of_tasks_in_range_of_weekly() {
        assert!(matches!(calculate_mode(15, 20, 5, 0), ProductivityMode::Meaningful));
    }

    #[test]
    fn test_sum_of_tasks_off_by_one() {
        assert!(matches!(calculate_mode(14, 20, 5, 0), ProductivityMode::Chores));
    }

    #[test]
    fn test_done_today_does_not_change_from_chores_until_exceed() {
        assert!(matches!(calculate_mode(14, 20, 5, 0), ProductivityMode::Chores));
        assert!(matches!(calculate_mode(15, 20, 5, 1), ProductivityMode::Chores));
        assert!(matches!(calculate_mode(18, 20, 5, 4), ProductivityMode::Chores));
        assert!(matches!(calculate_mode(19, 20, 5, 5), ProductivityMode::Chores));
        assert!(matches!(calculate_mode(20, 20, 5, 6), ProductivityMode::Meaningful));
    }
}