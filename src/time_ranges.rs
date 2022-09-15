use std::collections::HashMap;

use bdays::HolidayCalendar;
use chrono::{Datelike, NaiveDate, TimeZone};
use num_traits::FromPrimitive;

pub type DateTime = chrono::DateTime<chrono::Utc>;

struct LuxembourgHolidayCalender;
struct VacationsCalendar {
    vacations: Vec<(DateTime, DateTime)>,
}

pub struct CalendarCombination {
    calendars: Vec<Box<dyn HolidayCalendar<DateTime>>>,
}

impl CalendarCombination {
    pub fn holydays_and_vacations(vacations: Vec<(DateTime, DateTime)>) -> Self {
        Self {
            calendars: vec![
                Box::new(LuxembourgHolidayCalender),
                Box::new(VacationsCalendar { vacations }),
            ],
        }
    }
}

impl HolidayCalendar<DateTime> for CalendarCombination {
    fn is_holiday(&self, date: DateTime) -> bool {
        self.calendars.iter().any(|c| c.is_holiday(date))
    }
}

impl HolidayCalendar<DateTime> for VacationsCalendar {
    fn is_holiday(&self, date: DateTime) -> bool {
        self.vacations
            .iter()
            .any(|(start, end)| date >= *start && date <= *end)
    }
}

impl HolidayCalendar<DateTime> for LuxembourgHolidayCalender {
    fn is_holiday(&self, date: DateTime) -> bool {
        let (yy, mm, dd) = (date.year(), date.month(), date.day());

        let res = match (mm, dd) {
            (1, 1) => true,   // New Year
            (5, 1) => true,   // May Day
            (5, 9) => true,   // Europe Day
            (6, 23) => true,  // Grand Duke's Birthday
            (8, 15) => true,  // Assumption
            (11, 1) => true,  // All Saints
            (12, 25) => true, // Christmas Day
            (12, 26) => true, // Boxing Day
            _ => false,
        };

        if res {
            return res;
        }

        if mm == 4 || mm == 5 || mm == 6 {
            let easter = bdays::easter::easter_naive_date(yy).unwrap();
            let easter_monday = easter.succ();
            if easter_monday.month() == mm && easter_monday.day() == dd {
                return true;
            }
            let ascension_day = easter + chrono::Duration::days(39);
            if ascension_day.month() == mm && ascension_day.day() == dd {
                return true;
            }

            let whit_onday = easter + chrono::Duration::days(50);
            if whit_onday.month() == mm && whit_onday.day() == dd {
                return true;
            }
        }

        false
    }
}

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
    let result = fixed_h as f64 + m + s;
    return result;
}

pub fn count_work_houres(
    since: DateTime,
    till: DateTime,
    calendar: &impl HolidayCalendar<DateTime>,
) -> f64 {
    let result = if since.date() == till.date() {
        if !calendar.is_bday(since) {
            0.0
        } else {
            count_hours(since.time(), till.time())
        }
    } else {
        let days = calendar.bdays(since, till) as f64;
        let hours = days * 8.0;
        let left = if calendar.is_bday(since) {
            count_hours(day_start(), since.time())
        } else {
            0.0
        };
        let right = if calendar.is_bday(till) {
            count_hours(day_start(), till.time())
        } else {
            0.0
        };
        hours - left + right
    };

    result
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
    fn work_hours(
        &self,
        global_start: DateTime,
        global_end: DateTime,
        calendar: &impl HolidayCalendar<DateTime>,
    ) -> f64 {
        let since = self.start.unwrap_or(global_start).max(global_start);
        let till = self.end.unwrap_or(global_end).min(global_end);
        count_work_houres(since, till, calendar)
    }

    fn month_hours(
        &self,
        global_start: DateTime,
        global_end: DateTime,
        calendar: &impl HolidayCalendar<DateTime>,
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

            let wh = count_work_houres(s, e, calendar);

            result.insert(chrono::Month::from_u32(s.month()).unwrap(), wh);

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
    calendar: &impl HolidayCalendar<DateTime>,
) -> f64 {
    let global_start = global_start.unwrap_or(chrono::DateTime::from_utc(
        chrono::NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
        chrono::Utc,
    ));
    let global_end = global_end.unwrap_or(now()).min(now());
    ranges
        .iter()
        .map(|r| r.work_hours(global_start, global_end, calendar))
        .sum()
}

pub fn month_hours(
    ranges: &[TimeRange],
    global_start: Option<DateTime>,
    global_end: Option<DateTime>,
    calendar: &impl HolidayCalendar<DateTime>,
) -> HashMap<chrono::Month, f64> {
    let global_start = global_start.unwrap_or(chrono::DateTime::from_utc(
        chrono::NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0),
        chrono::Utc,
    ));
    let global_end = global_end.unwrap_or(now()).min(now());

    let mut result = HashMap::new();

    for r in ranges {
        let m_h = r.month_hours(global_start, global_end, calendar);
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

pub fn now_timestamp() -> i64 {
    to_timestamp(&now())
}

pub fn to_timestamp(dt: &DateTime) -> i64 {
    dt.timestamp_nanos()
}

pub fn from_timestamp(ts: i64) -> DateTime {
    const M: i64 = 1_000_000_000;
    let secs = ts / M;
    let nano = ts - secs * M;
    chrono::Utc.from_utc_datetime(&chrono::NaiveDateTime::from_timestamp(secs, nano as u32))
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
            assert_eq!(
                count_work_houres(now, now + chrono::Duration::hours(1), &calendar),
                0.0
            );
            now = now + chrono::Duration::days(2);
        }
        assert_eq!(
            count_work_houres(now, now + chrono::Duration::hours(1), &calendar),
            1.0
        );
        let mut tomorrow = now + chrono::Duration::days(1);
        if !calendar.is_bday(tomorrow) {
            tomorrow = tomorrow + chrono::Duration::days(2);
        }
        assert_eq!(count_work_houres(now, tomorrow, &calendar), 8.0);
        assert_eq!(
            count_work_houres(now, tomorrow + chrono::Duration::hours(2), &calendar),
            10.0
        );
        assert_eq!(
            count_work_houres(
                now,
                tomorrow + chrono::Duration::days(1) + chrono::Duration::hours(2),
                &calendar
            ),
            18.0
        );
        assert_eq!(
            count_work_houres(
                now,
                tomorrow + chrono::Duration::days(2) + chrono::Duration::hours(2),
                &calendar
            ),
            26.0
        );
    }
}
