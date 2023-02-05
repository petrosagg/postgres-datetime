#![allow(dead_code)]

use crate::datetime_raw::pg_time_t;

/// Time type information
struct TimeTypeInfo {
    /// UT offset in seconds
    utoff: i32,
    /// used to set tm_isdst
    isdst: bool,
    /// abbreviation list index
    desigidx: usize,
    /// transition is std time
    ttisstd: bool,
    /// transition is UT
    ttisut: bool,
}

/// Leap second information
struct LeapSecondInfo {
    /// transition time
    trans: pg_time_t,
    /// correction to apply
    corr: i64,
}

// In the current implementation, "tzset()" refuses to deal with files that
// exceed any of the limits below.

const TZ_MAX_TIMES: usize = 2000;

/// This must be at least 17 for Europe/Samara and Europe/Vilnius.  */
/// Limited by what (unsigned char)'s can hold */
const TZ_MAX_TYPES: usize = 256;

/// Maximum number of abbreviation characters */
/// (limited by what unsigned chars can hold) */
const TZ_MAX_CHARS: usize = 50;

/// Maximum number of leap second corrections */
const TZ_MAX_LEAPS: usize = 50;

/// Maximum length of a timezone name (not including trailing null)
const TZ_STRLEN_MAX: usize = 255;

const fn BIGGEST(a: usize, b: usize) -> usize {
    if a < b {
        b
    } else {
        a
    }
}

struct State {
    leapcnt: usize,
    timecnt: usize,
    typecnt: usize,
    charcnt: usize,
    goback: bool,
    goahead: bool,
    ats: [pg_time_t; TZ_MAX_TIMES],
    types: [u8; TZ_MAX_TIMES],
    ttis: [TimeTypeInfo; TZ_MAX_TYPES],
    chars: [u8; BIGGEST(
        BIGGEST(TZ_MAX_CHARS + 1, 4 /* sizeof gmt */),
        2 * (TZ_STRLEN_MAX + 1),
    )],
    lsis: [LeapSecondInfo; TZ_MAX_LEAPS],
    /// The time type to use for early times or if no transitions. It is always
    /// zero for recent tzdb releases. It might be nonzero for data from tzdb
    /// 2018e or earlier.
    defaulttype: usize,
}

pub(crate) struct pg_tz {
    /// TZname contains the canonically-cased name of the timezone
    TZname: [u8; TZ_STRLEN_MAX + 1],
    state: State,
}
