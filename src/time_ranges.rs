use std::collections::HashMap;

use bdays::HolidayCalendar;
use chrono::{Datelike, NaiveDate};
use num_traits::FromPrimitive;

pub type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Debug)]
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
    let dur = dur - chrono::Duration::hours(h);
    let m = dur.num_minutes() as f64 / 60.0;
    let dur = dur - chrono::Duration::minutes(dur.num_minutes());
    let s = dur.num_seconds() as f64 / (60.0 * 60.0);
    return fixed_h as f64 + m + s;
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
        days * 8.0 + ends
    }
}

fn end_of_month(d: DateTime) -> DateTime {
    let date = NaiveDate::from_ymd(d.year(), d.month(), 1).and_hms(0, 0, 0);
    let date = DateTime::from_utc(date, chrono::Utc);
    let date = chronoutil::shift_months(date, 1);
    date - chrono::Duration::seconds(1)
}

pub fn month_range(since: DateTime, till: DateTime) -> Vec<chrono::Month> {
    let mut since = NaiveDate::from_ymd(since.year(), since.month(), 1);
    let till = NaiveDate::from_ymd(till.year(), till.month(), 1);

    let mut result = Vec::new();

    while since != till {
        result.push(chrono::Month::from_u32(since.month()).unwrap());
        since = chronoutil::shift_months(since, 1);
    }

    result
}

impl TimeRange {
    fn work_hours(&self, global_start: DateTime, global_end: DateTime) -> f64 {
        let since = self.start.unwrap_or(global_start).max(global_start);
        let till = self.end.unwrap_or(global_end).min(global_end);
        count_work_houres(since, till)
    }

    fn month_hours(
        &self,
        global_start: DateTime,
        global_end: DateTime,
    ) -> HashMap<chrono::Month, f64> {
        let since = self.start.unwrap_or(global_start).max(global_start);
        let till = self.end.unwrap_or(global_end).min(global_end);

        let mut result = HashMap::new();

        let mut m_since = since;
        loop {
            let s = m_since;
            let mut e = end_of_month(m_since);
            let mut stop = false;
            if e > till {
                e = till;
                stop = true
            }

            result.insert(
                chrono::Month::from_u32(s.month()).unwrap(),
                count_work_houres(s, e),
            );
            if stop {
                break;
            } else {
                m_since = e + chrono::Duration::seconds(1);
            }
        }

        result
    }
}

pub fn working_houres_from_ranges(
    ranges: &[TimeRange],
    global_start: Option<DateTime>,
    global_end: Option<DateTime>,
) -> f64 {
    let global_start = global_start.unwrap_or(chrono::DateTime::from_utc(
        chrono::NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
        chrono::Utc,
    ));
    let global_end = global_end.unwrap_or(now()).min(now());
    ranges
        .iter()
        .map(|r| r.work_hours(global_start, global_end))
        .sum()
}

pub fn month_hours(
    ranges: &[TimeRange],
    global_start: Option<DateTime>,
    global_end: Option<DateTime>,
) -> HashMap<chrono::Month, f64> {
    let global_start = global_start.unwrap_or(chrono::DateTime::from_utc(
        chrono::NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
        chrono::Utc,
    ));
    let global_end = global_end.unwrap_or(now()).min(now());

    let mut result = HashMap::new();

    for r in ranges {
        let m_h = r.month_hours(global_start, global_end);
        for (k, v) in m_h.iter() {
            if result.contains_key(k) {
                let old_v = result.get_mut(k).unwrap();
                *old_v += *v;
            } else {
                result.insert(*k, *v);
            }
        }
    }

    result
}

pub fn now() -> DateTime {
    chrono::Utc::now()
}

pub fn now_str() -> String {
    let now_value = now();
    dt_to_str(&now_value)
}

pub fn dt_to_str(dt: &DateTime) -> String {
    dt.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true)
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
