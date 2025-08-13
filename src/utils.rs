use chrono::{Local, NaiveDate, NaiveDateTime};
use clap::ArgMatches;

use crate::model::Todo;

const DATETIME_FMT: &str = "%Y-%m-%d_%H:%M:%S";

#[inline]
pub fn now() -> NaiveDateTime {
    Local::now().naive_local()
}

#[inline]
pub fn parse_dt_opt(s: Option<&String>) -> Option<NaiveDateTime> {
    s.and_then(|v| NaiveDateTime::parse_from_str(v, DATETIME_FMT).ok())
}

#[inline]
pub fn parse_day_opt(s: Option<&String>) -> Option<NaiveDate> {
    match s {
        Some(v) if v.eq_ignore_ascii_case("today") => Some(Local::now().date_naive()),
        Some(v) => NaiveDate::parse_from_str(v, "%Y-%m-%d").ok(),
        None => None,
    }
}

#[inline]
pub fn next_id(tasks: &[Todo]) -> u32 {
    tasks.iter().map(|t| t.id).last().unwrap_or(0) + 1
}

#[inline]
pub fn get_id(sub_m: &ArgMatches) -> Option<u32> {
    sub_m.get_one::<u32>("id").copied()
}
