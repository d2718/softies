/*!
For filtering by time.
*/
use std::time::SystemTime;

use time::{Date, format_description::FormatItem, macros::format_description, OffsetDateTime, Time, UtcOffset};

static ERR_MSG: &str = r#"illegal date/time format; try one of:
    "2021-01-27 7:20:35"
    "1/27/2021 7:20:35"
    2021-01-27
    1/27/2021
    7:20:35
    7:20
"#;

/*
The various supported date/time formats to try when parsing.
*/
const ISO_DATE: &[FormatItem] = format_description!(
    "[year]-[month padding:none]-[day padding:none]"
);
const USA_DATE: &[FormatItem] = format_description!(
    "[month padding:none]/[day padding:none]/[year]"
);
const TIME: &[FormatItem] = format_description!(
    "[hour padding:none]:[minute][optional [:[second]]]"
);

fn try_date(datestr: &str) -> Result<Date, &'static str> {
    if let Ok(d) = Date::parse(datestr, ISO_DATE) {
        return Ok(d)
    } else {
        Date::parse(datestr, USA_DATE).map_err(|_| ERR_MSG)
    }
}

fn try_time(timestr: &str) -> Result<Time, &'static str> {
    Time::parse(timestr, TIME).map_err(|_| ERR_MSG)
}

/**
Attempt to parse a string into an `OffsetDateTime`.

The user should be able to enter the timestamp in a number of
mutually-unambiguous formats (see `ERR_MSG`, above). This essentially
tries all of them until one works, or it gives up and returns an error.

Dates with no times default to midnight (the earliest time in the given
day); times with no dates default to the current day; seconds are
optional (defaults to 0).
*/
pub fn parse_time(tstr: &str) -> Result<SystemTime, &'static str> {
    // If we can't determine the timezone, we just pretend it's UTC.
    let tz_offs = UtcOffset::current_local_offset()
        .unwrap_or(UtcOffset::UTC);

    let chunks: Vec<&str> = tstr.split_ascii_whitespace().collect();
    
    let tstamp = match &chunks[..] {
        [datestr, timestr] => {
            let d = try_date(datestr)?;
            let t = try_time(timestr)?;
            d.with_time(t).assume_offset(tz_offs)
        }
        [onestr] => {
            if let Ok(d) = try_date(onestr) {
                // This unwrap should be okay; midnight is in range.
                d.with_hms(0, 0, 0).unwrap().assume_offset(tz_offs)
            } else {
                let t = try_time(onestr)?;
                let today_time: OffsetDateTime = SystemTime::now().into();
                today_time.replace_time(t).replace_offset(tz_offs)
            }
        },
        _ => return Err(ERR_MSG),
    };

    Ok(tstamp.into())
}