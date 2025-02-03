use std::fmt::Display;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

pub fn to_hours(hours: i32) -> i32 {
    hours % 12
}

pub fn to_minutes(min: i32) -> (i32, i32) {
    (min / 60, min % 60)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: to_hours(hours),
            minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (h, m) = to_minutes(self.minutes + minutes);
        Clock {
            hours: to_hours(h + self.hours),
            minutes: m,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn convert_to_digits(digit: i32) -> String {
            if digit >= 10 {
                digit.to_string()
            } else {
                format!("0{}", digit)
            }
        }
        write!(
            f,
            "{}:{}",
            convert_to_digits(self.hours),
            convert_to_digits(self.minutes)
        )
    }
}

pub mod test {
    use super::*;

    #[test]
    fn on_the_hour() {
        assert_eq!(Clock::new(8, 0).to_string(), "08:00");
    }

    #[test]
    fn past_the_hour() {
        assert_eq!(Clock::new(11, 9).to_string(), "11:09");
    }

    #[test]
    fn midnight_is_zero_hours() {
        assert_eq!(Clock::new(24, 0).to_string(), "00:00");
    }

    #[test]
    fn hour_rolls_over() {
        assert_eq!(Clock::new(25, 0).to_string(), "01:00");
    }

    #[test]
    fn hour_rolls_over_continuously() {
        assert_eq!(Clock::new(100, 0).to_string(), "04:00");
    }

    #[test]
    fn sixty_minutes_is_next_hour() {
        assert_eq!(Clock::new(1, 60).to_string(), "02:00");
    }

    #[test]
    #[ignore]
    fn minutes_roll_over() {
        assert_eq!(Clock::new(0, 160).to_string(), "02:40");
    }
}
