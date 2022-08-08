use bdays::HolidayCalendar;

pub type DateTime = chrono::DateTime<chrono::Utc>;

pub struct TimeRange {
    pub start: Option<DateTime>,
    pub end: Option<DateTime>,
}

fn day_start() -> chrono::NaiveTime {
    chrono::NaiveTime::from_hms(8, 0, 0)
}
fn day_end() -> chrono::NaiveTime {
    chrono::NaiveTime::from_hms(17, 0, 0)
}

fn count_hours(since: chrono::NaiveTime, till: chrono::NaiveTime) -> f64 {
    let dur = till.max(day_start()).min(day_end()) - since.max(day_start()).min(day_end());
    let h = dur.num_hours();
    let fixed_h = if h >= 9 { 8 } else { h };
    let m = (dur - chrono::Duration::hours(h)).num_minutes() as f64 / 60.0;
    return fixed_h as f64 + m;
}

pub fn count_work_houres(since: DateTime, till: DateTime) -> f64 {
    let calendar = bdays::calendars::WeekendsOnly;

    if since.date() == till.date() {
        if !calendar.is_bday(since.date()) {
            0.0
        } else {
            count_hours(since.time(), till.time())
        }
    } else {
        let since_time = since.time();
        let till_time = till.time();
        let since_day = since.date() + chrono::Duration::days(1);
        let till_day = till.date();
        let ends = count_hours(since_time, day_end()) + count_hours(day_start(), till_time);
        let days = calendar.bdays(since_day, till_day) as f64;
        let days = if days >= 0.0 { days } else { 0.0 };
        println!("{}-{} = {} * 8.0 + {}", since, till, days, ends);
        days * 8.0 + ends
    }
}

pub fn now() -> DateTime {
    chrono::Utc::now()
}

pub fn now_str() -> String {
    let now_value = now();
    now_value.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true)
}

#[cfg(test)]
mod tests {
    use super::count_work_houres;
    use bdays::HolidayCalendar;
    use chrono::DateTime;
    use chrono::Utc;
    use chrono::{NaiveDateTime, NaiveTime};

    #[test]
    fn count_work_houres_test() {
        let calendar = bdays::calendars::WeekendsOnly;
        let now = Utc::now();
        let mut now = DateTime::from_utc(
            NaiveDateTime::new(now.date_naive(), NaiveTime::from_hms(8, 0, 0)),
            Utc,
        );
        if !calendar.is_bday(now) {
            println!("today holyday");
            assert_eq!(
                count_work_houres(now, now + chrono::Duration::hours(1)),
                0.0
            );
            now = now + chrono::Duration::days(2);
        }
        println!("{} {}", now, now + chrono::Duration::hours(1));
        assert_eq!(
            count_work_houres(now, now + chrono::Duration::hours(1)),
            1.0
        );
        let mut tomorrow = now + chrono::Duration::days(1);
        if !calendar.is_bday(tomorrow) {
            tomorrow = tomorrow + chrono::Duration::days(2);
        }
        println!("{} {}", now, tomorrow);
        assert_eq!(count_work_houres(now, tomorrow), 8.0);
        assert_eq!(
            count_work_houres(now, tomorrow + chrono::Duration::hours(2)),
            10.0
        );
        assert_eq!(
            count_work_houres(
                now,
                tomorrow + chrono::Duration::days(1) + chrono::Duration::hours(2)
            ),
            18.0
        );
        assert_eq!(
            count_work_houres(
                now,
                tomorrow + chrono::Duration::days(2) + chrono::Duration::hours(2)
            ),
            26.0
        );
    }
}
