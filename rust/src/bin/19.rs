/// You are given the following information, but you may prefer to do some research for yourself.
///
/// 1 Jan 1900 was a Monday.
/// Thirty days has September,
/// April, June and November.
/// All the rest have thirty-one,
/// Saving February alone,
/// Which has twenty-eight, rain or shine.
/// And on leap years, twenty-nine.
/// A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
/// how many sundays fell on the first of the month during the twentieth century (1 jan 1901 to 31 dec 2000)?

fn main() {
    let start_date = Date::new(Month::January, 1, 1901, DayOfWeek::Tuesday);
    let end_date = Date::new(Month::December, 31, 2000, DayOfWeek::Sunday);
    let mut date = start_date.clone();
    let mut first_of_the_month_sundays: usize = 0;
    while date != end_date {
        date = date.next().unwrap();
        if date.day_of_week == DayOfWeek::Sunday && date.day_of_month == 1 {
            first_of_the_month_sundays += 1
        }
    };
    println!("{:?}", first_of_the_month_sundays);
}

#[derive(PartialEq, Debug, Clone)]
enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Iterator for DayOfWeek {
    type Item = DayOfWeek;

    fn next(&mut self) -> Option<Self::Item> {
        let day = match self {
            Self::Sunday => Self::Monday,
            Self::Monday => Self::Tuesday,
            Self::Tuesday => Self::Wednesday,
            Self::Wednesday => Self::Thursday,
            Self::Thursday => Self::Friday,
            Self::Friday => Self::Saturday,
            Self::Saturday => Self::Sunday,
        };
        Some(day)
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

trait MonthDays {
    fn days_in_month(&self, is_leap_year: bool) -> u8;
}

impl MonthDays for Month {
    /// The order matches the poem. Cute.
    fn days_in_month(&self, is_leap_year: bool) -> u8 {
        match self {
            Self::September => 30,
            Self::April => 30,
            Self::June => 30,
            Self::November => 30,
            Self::January => 31,
            Self::March => 31,
            Self::May => 31,
            Self::July => 31,
            Self::August => 31,
            Self::October => 31,
            Self::December => 31,
            Self::February => match is_leap_year {
                true => 29,
                false => 28,
            },
        }
    }
}
impl Iterator for Month {
    type Item = Month;

    fn next(&mut self) -> Option<Self::Item> {
        let month = match self {
            Self::January => Self::February,
            Self::February => Self::March,
            Self::March => Self::April,
            Self::April => Self::May,
            Self::May => Self::June,
            Self::June => Self::July,
            Self::July => Self::August,
            Self::August => Self::September,
            Self::September => Self::October,
            Self::October => Self::November,
            Self::November => Self::December,
            Self::December => Self::January,
        };
        Some(month)
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Date {
    month: Month,
    day_of_month: u8,
    year: Year,
    day_of_week: DayOfWeek,
}

impl Date {
    fn new(month: Month, day_of_month: u8, year: u16, day_of_week: DayOfWeek) -> Self {
        Self {
            month,
            day_of_month,
            year: Year(year),
            day_of_week,
        }
    }
}
impl Iterator for Date {
    type Item = Date;

    fn next(&mut self) -> Option<Self::Item> {
        let mut month = self.month.clone();
        let is_last_day_of_month =
            self.day_of_month == month.days_in_month(self.year.is_leap_year());
        let mut year = self.year.0;
        let mut day_of_month = self.day_of_month;
        match is_last_day_of_month {
            true => {
                day_of_month = 1;
                if month == Month::December {
                    year += 1;
                }
                month = month.next()?;
            }
            false => {
                day_of_month += 1;
            }
        };
        let day_of_week = self.day_of_week.next()?;
        Some(Self::new(month, day_of_month, year, day_of_week))
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Year(u16);
trait LeapYear {
    fn is_leap_year(&self) -> bool;
}
impl LeapYear for Year {
    fn is_leap_year(&self) -> bool {
        if self.0 % 400 == 0 {
            true
        } else if self.0 % 100 == 0 {
            false
        } else { self.0 % 4 == 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_day() {
        assert_eq!(DayOfWeek::Sunday.next().unwrap(), DayOfWeek::Monday);
        assert_eq!(DayOfWeek::Monday.next().unwrap(), DayOfWeek::Tuesday);
        assert_eq!(DayOfWeek::Tuesday.next().unwrap(), DayOfWeek::Wednesday);
        assert_eq!(DayOfWeek::Wednesday.next().unwrap(), DayOfWeek::Thursday);
        assert_eq!(DayOfWeek::Thursday.next().unwrap(), DayOfWeek::Friday);
        assert_eq!(DayOfWeek::Friday.next().unwrap(), DayOfWeek::Saturday);
        assert_eq!(DayOfWeek::Saturday.next().unwrap(), DayOfWeek::Sunday);
        assert_eq!(DayOfWeek::Saturday.next().unwrap(), DayOfWeek::Sunday);
    }

    #[test]
    fn test_next_month() {
        assert_eq!(Month::January.next().unwrap(), Month::February);
        assert_eq!(Month::February.next().unwrap(), Month::March);
        assert_eq!(Month::March.next().unwrap(), Month::April);
        assert_eq!(Month::April.next().unwrap(), Month::May);
        assert_eq!(Month::May.next().unwrap(), Month::June);
        assert_eq!(Month::June.next().unwrap(), Month::July);
        assert_eq!(Month::July.next().unwrap(), Month::August);
        assert_eq!(Month::August.next().unwrap(), Month::September);
        assert_eq!(Month::September.next().unwrap(), Month::October);
        assert_eq!(Month::October.next().unwrap(), Month::November);
        assert_eq!(Month::November.next().unwrap(), Month::December);
        assert_eq!(Month::December.next().unwrap(), Month::January);
    }

    #[test]
    fn test_next_date() {
        assert_eq!(
            Date::new(Month::January, 10, 1905, DayOfWeek::Tuesday)
                .next()
                .unwrap(),
            Date::new(Month::January, 11, 1905, DayOfWeek::Wednesday)
        );
        assert_eq!(
            Date::new(Month::January, 31, 1905, DayOfWeek::Tuesday)
                .next()
                .unwrap(),
            Date::new(Month::February, 1, 1905, DayOfWeek::Wednesday)
        );
        assert_eq!(
            Date::new(Month::February, 28, 1905, DayOfWeek::Tuesday)
                .next()
                .unwrap(),
            Date::new(Month::March, 1, 1905, DayOfWeek::Wednesday)
        );
        assert_eq!(
            Date::new(Month::February, 28, 1904, DayOfWeek::Sunday)
                .next()
                .unwrap(),
            Date::new(Month::February, 29, 1904, DayOfWeek::Monday)
        );
        assert_eq!(
            Date::new(Month::December, 31, 1999, DayOfWeek::Sunday)
                .next()
                .unwrap(),
            Date::new(Month::January, 1, 2000, DayOfWeek::Monday)
        );
    }

    #[test]
    fn test_is_leap_year() {
        assert!(!Year(1900).is_leap_year());
        assert!(Year(1904).is_leap_year());
        assert!(Year(1600).is_leap_year());
        assert!(!Year(1903).is_leap_year());
    }
}
