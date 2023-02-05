use std::str::FromStr;

use ::libc;
use once_cell::sync::Lazy;

use crate::datetime::{
    FieldMask, FieldType, TokenFieldType, FIELD_MASK_ALL_SECS, FIELD_MASK_DATE, FIELD_MASK_TIME,
};

use crate::tz::pg_tz;

const HOURS_PER_DAY: i32 = 24;
const MINS_PER_HOUR: i32 = 60;
const SECS_PER_DAY: i32 = 86_400;
const SECS_PER_HOUR: i32 = 3600;
const SECS_PER_MINUTE: i32 = 60;
const USECS_PER_DAY: i64 = 86_400_000_000;
const USECS_PER_HOUR: i64 = 3_600_000_000;
const USECS_PER_MINUTE: i64 = 60_000_000;
const USECS_PER_SEC: i64 = 1_000_000;
const POSTGRES_EPOCH_JDATE: i64 = 2_451_545; /* == date2j(2000, 1, 1) */
const UNIX_EPOCH_JDATE: i64 = 2_440_588; /* == date2j(1970, 1, 1) */

static DateOrder: i32 = 0;

fn dt2time(jd: Timestamp, hour: &mut i32, min: &mut i32, sec: &mut i32, fsec: &mut fsec_t) {
    let mut time: TimeOffset;

    time = jd;

    *hour = (time / USECS_PER_HOUR).try_into().unwrap();
    time -= (*hour as i64) * USECS_PER_HOUR;
    *min = (time / USECS_PER_MINUTE).try_into().unwrap();
    time -= (*min as i64) * USECS_PER_MINUTE;
    *sec = (time / USECS_PER_SEC).try_into().unwrap();
    *fsec = (time - (*sec as i64 * USECS_PER_SEC)).try_into().unwrap();
}
fn GetCurrentTransactionStartTimestamp() -> TimestampTz {
    11223344
}

fn pg_localtime(_timep: *const pg_time_t, _tz: &pg_tz) -> Box<pg_tm> {
    Box::new(pg_tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: Some(false),
        tm_gmtoff: 0,
        tm_zone: None,
    })
}

fn pg_interpret_timezone_abbrev(
    _abbrev: &str,
    _timep: &pg_time_t,
    _gmtoff: &mut i64,
    _isdst: &mut bool,
    _tz: &pg_tz,
) -> bool {
    unimplemented!()
}

fn pg_next_dst_boundary(
    _timep: *const pg_time_t,
    _before_gmtoff: *mut i64,
    _before_isdst: &mut bool,
    _boundary: *mut pg_time_t,
    _after_gmtoff: *mut i64,
    _after_isdst: &mut bool,
    _tz: &pg_tz,
) -> i32 {
    0
}

fn pg_tzset(_tzname: *const libc::c_char) -> Option<&'static pg_tz> {
    None
}

fn pg_tzset_rust(_tzname: &str) -> Option<&'static pg_tz> {
    None
}

static session_timezone: Lazy<pg_tz> = Lazy::new(|| Default::default());

fn strtoint(str: *const libc::c_char, endptr: *mut *mut libc::c_char, base: i32) -> i32 {
    unsafe {
        let val = libc::strtol(str, endptr, base);
        val.try_into().unwrap()
    }
}

fn strtoint_rust<'a>(s: &'a str, end: &mut &'a str) -> Result<i32, ()> {
    let idx = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    *end = &s[idx..];
    i32::from_str(&s[..idx]).or(Err(()))
}

fn time_overflows(hour: i32, min: i32, sec: i32, fsec: fsec_t) -> bool {
    /* Range-check the fields individually. */
    if !(0..=HOURS_PER_DAY).contains(&hour)
        || !(0..MINS_PER_HOUR).contains(&min)
        || !(0..=SECS_PER_MINUTE).contains(&sec)
        || fsec < 0
        || fsec as i64 > USECS_PER_SEC
    {
        return true;
    }

    /*
     * Because we allow, eg, hour = 24 or sec = 60, we must check separately
     * that the total time value doesn't exceed 24:00:00.
     */
    if (((((hour as i64 * MINS_PER_HOUR as i64 + min as i64) * SECS_PER_MINUTE as i64)
        + sec as i64)
        * USECS_PER_SEC as i64)
        + fsec as i64)
        > USECS_PER_DAY
    {
        return true;
    }

    false
}

/// TMODULO()
/// Like FMODULO(), but work on the timestamp datatype (now always int64).
/// We assume that int64 follows the C99 semantics for division (negative
/// quotients truncate towards zero).
fn TMODULO(t: &mut i64, q: &mut i64, u: i64) {
    *q = *t / u;
    if *q != 0 {
        *t -= *q * u;
    }
}

fn timestamp2tm(
    mut dt: Timestamp,
    tzp: Option<&mut i32>,
    tm: &mut pg_tm,
    fsec: &mut fsec_t,
    tzn: Option<&mut Option<String>>,
    mut attimezone: Option<&pg_tz>,
) -> i32 {
    let mut date: Timestamp = 0;
    let mut time: Timestamp;

    /* Use session timezone if caller asks for default */
    if attimezone.is_none() {
        attimezone = Some(&session_timezone);
    }

    time = dt;
    TMODULO(&mut time, &mut date, USECS_PER_DAY);

    if time < 0 {
        time += USECS_PER_DAY;
        date -= 1;
    }

    /* add offset to go from J2000 back to standard Julian date */
    date += POSTGRES_EPOCH_JDATE;

    /* Julian day routine does not work for negative Julian days */
    if date < 0 || date > libc::INT_MAX.into() {
        eprintln!("Julian day routine does not work for negative Julian days");
        return -1;
    }

    j2date(
        date.try_into().unwrap(),
        &mut tm.tm_year,
        &mut tm.tm_mon,
        &mut tm.tm_mday,
    );
    dt2time(time, &mut tm.tm_hour, &mut tm.tm_min, &mut tm.tm_sec, fsec);

    /* Done if no TZ conversion wanted */
    match tzp {
        None => {
            tm.tm_isdst = None;
            tm.tm_gmtoff = 0;
            tm.tm_zone = None;
            if let Some(tzn) = tzn {
                *tzn = None;
            }
            return 0;
        }
        Some(tzp) => {
            /*
             * If the time falls within the range of pg_time_t, use pg_localtime() to
             * rotate to the local time zone.
             *
             * First, convert to an integral timestamp, avoiding possibly
             * platform-specific roundoff-in-wrong-direction errors, and adjust to
             * Unix epoch.  Then see if we can convert to pg_time_t without loss. This
             * coding avoids hardwiring any assumptions about the width of pg_time_t,
             * so it should behave sanely on machines without int64.
             */
            dt = (dt - *fsec as i64) / USECS_PER_SEC
                + (POSTGRES_EPOCH_JDATE - UNIX_EPOCH_JDATE) * SECS_PER_DAY as i64;
            let utime: pg_time_t = dt;
            if utime == dt {
                let tx = pg_localtime(&utime, attimezone.unwrap());

                tm.tm_year = tx.tm_year + 1900;
                tm.tm_mon = tx.tm_mon + 1;
                tm.tm_mday = tx.tm_mday;
                tm.tm_hour = tx.tm_hour;
                tm.tm_min = tx.tm_min;
                tm.tm_sec = tx.tm_sec;
                tm.tm_isdst = tx.tm_isdst;
                tm.tm_gmtoff = tx.tm_gmtoff;
                tm.tm_zone = tx.tm_zone;
                *tzp = (-tm.tm_gmtoff).try_into().unwrap();
                if let Some(tzn) = tzn {
                    *tzn = tm.tm_zone.clone();
                }
            } else {
                /*
                 * When out of range of pg_time_t, treat as GMT
                 */
                *tzp = 0;
                /* Mark this as *no* time zone available */
                tm.tm_isdst = None;
                tm.tm_gmtoff = 0;
                tm.tm_zone = None;
                if let Some(tzn) = tzn {
                    *tzn = None;
                }
            }
            0
        }
    }
}

extern "C" {
    #![allow(improper_ctypes)]
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strchr(_: *const libc::c_char, _: i32) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> u64;
    fn __errno_location() -> *mut i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}

type int32 = i32;
type int64 = i64;
const _ISblank: u32 = 1 << 0;
const _IScntrl: u32 = 1 << 1;
const _ISpunct: u32 = 1 << 2;
const _ISalnum: u32 = 1 << 3;
const _ISupper: u32 = 1 << 8;
const _ISlower: u32 = 1 << 9;
const _ISalpha: u32 = 1 << 10;
const _ISdigit: u32 = 1 << 11;
const _ISxdigit: u32 = 1 << 12;
const _ISspace: u32 = 1 << 13;
const _ISprint: u32 = 1 << 14;
const _ISgraph: u32 = 1 << 15;
type Timestamp = int64;
type TimestampTz = int64;
type TimeOffset = int64;
pub type fsec_t = int32;
pub type DateADT = int32;
pub(crate) type pg_time_t = int64;
#[derive(Debug, Clone)]
pub struct pg_tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: Option<bool>,
    pub tm_gmtoff: i64,
    pub tm_zone: Option<String>,
}

#[derive(Clone)]
struct TimeZoneAbbrevTable {
    abbrevs: &'static [DateToken],
    _dyn_abbrevs: &'static [DateToken],
}
#[derive(Clone)]
struct DynamicZoneAbbrev {
    _tz: *mut pg_tz,
    _zone: [libc::c_char; 0],
}

static day_tab: [[i32; 13]; 2] = [
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 0],
    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 0],
];

const EPOCH: &str = "epoch";
const EARLY: &str = "-infinity";
const LATE: &str = "infinity";
const NOW: &str = "now";
const TODAY: &str = "today";
const TOMORROW: &str = "tomorrow";
const YESTERDAY: &str = "yesterday";
const DA_D: &str = "ad";
const DB_C: &str = "bc";

// Fundamental time field definitions for parsing.
//
// Meridian:  am, pm, or 24-hour style.
// Millennium: ad, bc
const AM: i32 = 0;
const PM: i32 = 1;

const AD: i32 = 0;
const BC: i32 = 1;

struct DateToken {
    token: &'static str,
    typ: FieldType,
    value: i32,
}
/// holds date/time keywords.
///
/// Note that this table must be strictly alphabetically ordered to allow an
/// O(ln(N)) search algorithm to be used.
///
/// The static table contains no TZ, DTZ, or DYNTZ entries; rather those
/// are loaded from configuration files and stored in ZONE_ABBREV_TABLE, whose
/// abbrevs[] field has the same format as the static DATE_TOKEN_TABLE.
static DATE_TOKEN_TABLE: &[DateToken] = &[
    // "-infinity" reserved for "early time"
    DateToken {
        token: EARLY,
        typ: FieldType::Reserved,
        value: TokenFieldType::Early as i32,
    },
    // "ad" for years > 0
    DateToken {
        token: DA_D,
        typ: FieldType::Adbc,
        value: AD,
    },
    // 00:00:00
    DateToken {
        token: "allballs",
        typ: FieldType::Reserved,
        value: TokenFieldType::Zulu as i32,
    },
    DateToken {
        token: "am",
        typ: FieldType::AmPm,
        value: AM,
    },
    DateToken {
        token: "apr",
        typ: FieldType::Month,
        value: 4,
    },
    DateToken {
        token: "april",
        typ: FieldType::Month,
        value: 4,
    },
    // "at" (throwaway)
    DateToken {
        token: "at",
        typ: FieldType::IgnoreDtf,
        value: 0,
    },
    DateToken {
        token: "aug",
        typ: FieldType::Month,
        value: 8,
    },
    DateToken {
        token: "august",
        typ: FieldType::Month,
        value: 8,
    },
    // "bc" for years <= 0
    DateToken {
        token: DB_C,
        typ: FieldType::Adbc,
        value: BC,
    },
    // "day of month" for ISO input
    DateToken {
        token: "d",
        typ: FieldType::Units,
        value: TokenFieldType::Day as i32,
    },
    DateToken {
        token: "dec",
        typ: FieldType::Month,
        value: 12,
    },
    DateToken {
        token: "december",
        typ: FieldType::Month,
        value: 12,
    },
    // day of week
    DateToken {
        token: "dow",
        typ: FieldType::Units,
        value: TokenFieldType::Dow as i32,
    },
    // day of year
    DateToken {
        token: "doy",
        typ: FieldType::Units,
        value: TokenFieldType::Doy as i32,
    },
    DateToken {
        token: "dst",
        typ: FieldType::DtzMod,
        value: SECS_PER_HOUR,
    },
    // "epoch" reserved for system epoch time
    DateToken {
        token: EPOCH,
        typ: FieldType::Reserved,
        value: TokenFieldType::Epoch as i32,
    },
    DateToken {
        token: "feb",
        typ: FieldType::Month,
        value: 2,
    },
    DateToken {
        token: "february",
        typ: FieldType::Month,
        value: 2,
    },
    DateToken {
        token: "fri",
        typ: FieldType::Dow,
        value: 5,
    },
    DateToken {
        token: "friday",
        typ: FieldType::Dow,
        value: 5,
    },
    // "hour"
    DateToken {
        token: "h",
        typ: FieldType::Units,
        value: TokenFieldType::Hour as i32,
    },
    // "infinity" reserved for "late time"
    DateToken {
        token: LATE,
        typ: FieldType::Reserved,
        value: TokenFieldType::Late as i32,
    },
    // ISO day of week, Sunday == 7
    DateToken {
        token: "isodow",
        typ: FieldType::Units,
        value: TokenFieldType::IsoDow as i32,
    },
    // year in terms of the ISO week date
    DateToken {
        token: "isoyear",
        typ: FieldType::Units,
        value: TokenFieldType::IsoYear as i32,
    },
    DateToken {
        token: "j",
        typ: FieldType::Units,
        value: TokenFieldType::Julian as i32,
    },
    DateToken {
        token: "jan",
        typ: FieldType::Month,
        value: 1,
    },
    DateToken {
        token: "january",
        typ: FieldType::Month,
        value: 1,
    },
    DateToken {
        token: "jd",
        typ: FieldType::Units,
        value: TokenFieldType::Julian as i32,
    },
    DateToken {
        token: "jul",
        typ: FieldType::Month,
        value: 7,
    },
    DateToken {
        token: "julian",
        typ: FieldType::Units,
        value: TokenFieldType::Julian as i32,
    },
    DateToken {
        token: "july",
        typ: FieldType::Month,
        value: 7,
    },
    DateToken {
        token: "jun",
        typ: FieldType::Month,
        value: 6,
    },
    DateToken {
        token: "june",
        typ: FieldType::Month,
        value: 6,
    },
    // "month" for ISO input
    DateToken {
        token: "m",
        typ: FieldType::Units,
        value: TokenFieldType::Month as i32,
    },
    DateToken {
        token: "mar",
        typ: FieldType::Month,
        value: 3,
    },
    DateToken {
        token: "march",
        typ: FieldType::Month,
        value: 3,
    },
    DateToken {
        token: "may",
        typ: FieldType::Month,
        value: 5,
    },
    // "minute" for ISO input
    DateToken {
        token: "mm",
        typ: FieldType::Units,
        value: TokenFieldType::Minute as i32,
    },
    DateToken {
        token: "mon",
        typ: FieldType::Dow,
        value: 1,
    },
    DateToken {
        token: "monday",
        typ: FieldType::Dow,
        value: 1,
    },
    DateToken {
        token: "nov",
        typ: FieldType::Month,
        value: 11,
    },
    DateToken {
        token: "november",
        typ: FieldType::Month,
        value: 11,
    },
    // current transaction time
    DateToken {
        token: NOW,
        typ: FieldType::Reserved,
        value: TokenFieldType::Now as i32,
    },
    DateToken {
        token: "oct",
        typ: FieldType::Month,
        value: 10,
    },
    DateToken {
        token: "october",
        typ: FieldType::Month,
        value: 10,
    },
    // "on" (throwaway)
    DateToken {
        token: "on",
        typ: FieldType::IgnoreDtf,
        value: 0,
    },
    DateToken {
        token: "pm",
        typ: FieldType::AmPm,
        value: PM,
    },
    // "seconds" for ISO input
    DateToken {
        token: "s",
        typ: FieldType::Units,
        value: TokenFieldType::Second as i32,
    },
    DateToken {
        token: "sat",
        typ: FieldType::Dow,
        value: 6,
    },
    DateToken {
        token: "saturday",
        typ: FieldType::Dow,
        value: 6,
    },
    DateToken {
        token: "sep",
        typ: FieldType::Month,
        value: 9,
    },
    DateToken {
        token: "sept",
        typ: FieldType::Month,
        value: 9,
    },
    DateToken {
        token: "september",
        typ: FieldType::Month,
        value: 9,
    },
    DateToken {
        token: "sun",
        typ: FieldType::Dow,
        value: 0,
    },
    DateToken {
        token: "sunday",
        typ: FieldType::Dow,
        value: 0,
    },
    // Filler for ISO time fields
    DateToken {
        token: "t",
        typ: FieldType::IsoTime,
        value: TokenFieldType::Time as i32,
    },
    DateToken {
        token: "thu",
        typ: FieldType::Dow,
        value: 4,
    },
    DateToken {
        token: "thur",
        typ: FieldType::Dow,
        value: 4,
    },
    DateToken {
        token: "thurs",
        typ: FieldType::Dow,
        value: 4,
    },
    DateToken {
        token: "thursday",
        typ: FieldType::Dow,
        value: 4,
    },
    // midnight
    DateToken {
        token: TODAY,
        typ: FieldType::Reserved,
        value: TokenFieldType::Today as i32,
    },
    // tomorrow midnight
    DateToken {
        token: TOMORROW,
        typ: FieldType::Reserved,
        value: TokenFieldType::Tomorrow as i32,
    },
    DateToken {
        token: "tue",
        typ: FieldType::Dow,
        value: 2,
    },
    DateToken {
        token: "tues",
        typ: FieldType::Dow,
        value: 2,
    },
    DateToken {
        token: "tuesday",
        typ: FieldType::Dow,
        value: 2,
    },
    DateToken {
        token: "wed",
        typ: FieldType::Dow,
        value: 3,
    },
    DateToken {
        token: "wednesday",
        typ: FieldType::Dow,
        value: 3,
    },
    DateToken {
        token: "weds",
        typ: FieldType::Dow,
        value: 3,
    },
    // "year" for ISO input
    DateToken {
        token: "y",
        typ: FieldType::Units,
        value: TokenFieldType::Year as i32,
    },
    // yesterday midnight
    DateToken {
        token: YESTERDAY,
        typ: FieldType::Reserved,
        value: TokenFieldType::Yesterday as i32,
    },
];

static ZONE_ABBREV_TABLE: Option<TimeZoneAbbrevTable> = None;

fn date2j(mut y: i32, mut m: i32, d: i32) -> i32 {
    if m > 2 {
        m += 1;
        y += 4800;
    } else {
        m += 13;
        y += 4799;
    }
    let century = y / 100;
    let mut julian = y * 365 - 32167;
    julian += y / 4 - century + century / 4;
    julian += 7834 * m / 256 + d;
    julian
}

fn j2date(jd: i32, year: &mut i32, month: &mut i32, day: &mut i32) {
    let mut julian = jd as u32;
    julian = julian.wrapping_add(32044);
    let mut quad = julian.wrapping_div(146097);
    let extra = julian
        .wrapping_sub(quad.wrapping_mul(146097))
        .wrapping_mul(4)
        .wrapping_add(3);
    julian = julian.wrapping_add(
        (60_u32)
            .wrapping_add(quad.wrapping_mul(3))
            .wrapping_add(extra.wrapping_div(146097)),
    );
    quad = julian.wrapping_div(1461);
    julian = julian.wrapping_sub(quad.wrapping_mul(1461));
    let mut y = julian.wrapping_mul(4).wrapping_div(1461) as i32;
    julian = (if y != 0 {
        julian.wrapping_add(305).wrapping_rem(365)
    } else {
        julian.wrapping_add(306).wrapping_rem(366)
    })
    .wrapping_add(123);
    y = (y as u32).wrapping_add(quad.wrapping_mul(4)) as i32;
    *year = y - 4800;
    quad = julian.wrapping_mul(2141).wrapping_div(65536);
    *day = julian.wrapping_sub((7834_u32).wrapping_mul(quad).wrapping_div(256)) as i32;
    *month = quad.wrapping_add(10).wrapping_rem(12).wrapping_add(1) as i32;
}

fn GetCurrentDateTime(tm: &mut pg_tm) {
    let mut fsec: fsec_t = 0;
    GetCurrentTimeUsec(tm, &mut fsec, None);
}

fn GetCurrentTimeUsec(tm: &mut pg_tm, fsec: &mut fsec_t, tzp: Option<&mut i32>) {
    let cur_ts: TimestampTz = GetCurrentTransactionStartTimestamp();
    let mut tzp_local = 0;
    if timestamp2tm(
        cur_ts,
        Some(&mut tzp_local),
        tm,
        fsec,
        None,
        Some(&session_timezone),
    ) != 0
    {
        // ereport(ERROR,
        // 		(errcode(ERRCODE_DATETIME_VALUE_OUT_OF_RANGE),
        // 		 errmsg("timestamp out of range")));
    }
    if let Some(tzp) = tzp {
        *tzp = tzp_local;
    }
}

unsafe fn ParseFractionalSecond(cp: *mut libc::c_char, fsec: &mut fsec_t) -> i32 {
    let cp = std::ffi::CStr::from_ptr(cp).to_str().unwrap();
    ParseFractionalSecond_rust(cp, fsec)
}

fn ParseFractionalSecond_rust(cp: &str, fsec: &mut fsec_t) -> i32 {
    let frac = match f64::from_str(cp) {
        Ok(frac) => frac,
        Err(_) => return -1,
    };
    *fsec = (frac * 1_000_000.0) as fsec_t;
    0
}

/// Breaks string into tokens based on a date/time context.
/// Returns an list of (field, type) pairs if successful or an error if bogus input detected.
///
/// The fields extracted from the input are stored as separate strings in the returned vector. Any
/// text is converted to lower case.
///
/// Several field types are assigned:
///   * TokenFieldType::Number - digits and (possibly) a decimal point
///   * TokenFieldType::Date - digits and two delimiters, or digits and text
///   * TokenFieldType::Time - digits, colon delimiters, and possibly a decimal point
///   * TokenFieldType::String - text (no digits or punctuation)
///   * TokenFieldType::Special - leading "+" or "-" followed by text
///   * TokenFieldType::Tz - leading "+" or "-" followed by digits (also eats ':', '.', '-')
///
/// Note that some field types can hold unexpected items:
///   * TokenFieldType::Number can hold date fields (yy.ddd)
///   * TokenFieldType::String can hold months (January) and time zones (PST)
///   * TokenFieldType::Date can hold time zone names (America/New_York, GMT-8)
pub fn parse_datetime(input: &str) -> Result<Vec<(String, TokenFieldType)>, i32> {
    let mut ret = vec![];
    let mut chars = input.chars().peekable();

    // outer loop through fields
    while chars.peek().is_some() {
        // Ignore spaces between fields
        if chars.next_if(|c| c.is_ascii_whitespace()).is_some() {
            continue;
        }

        // Record start of current field
        let mut fdata = String::new();
        let mut ftype: TokenFieldType;

        // leading digit? then date or time
        if let Some(c) = chars.next_if(|c| c.is_ascii_digit()) {
            fdata.push(c);
            while let Some(c) = chars.next_if(|c| c.is_ascii_digit()) {
                fdata.push(c);
            }

            // time field?
            if let Some(c) = chars.next_if_eq(&':') {
                ftype = TokenFieldType::Time;

                fdata.push(c);
                while let Some(c) = chars.next_if(|&c| c.is_ascii_digit() || c == ':' || c == '.') {
                    fdata.push(c);
                }
            // date field? allow embedded text month
            } else if let Some(c) = chars.next_if(|&c| c == '-' || c == '/' || c == '.') {
                // save delimiting character to use later
                let delim = c;
                fdata.push(c);

                // second field is all digits? then no embedded text month
                if let Some(c) = chars.next_if(|c| c.is_ascii_digit()) {
                    ftype = match delim {
                        '.' => TokenFieldType::Number,
                        _ => TokenFieldType::Date,
                    };
                    fdata.push(c);
                    while let Some(c) = chars.next_if(|c| c.is_ascii_digit()) {
                        fdata.push(c);
                    }

                    // insist that the delimiters match to get a three-field date.
                    if let Some(c) = chars.next_if_eq(&delim) {
                        ftype = TokenFieldType::Date;

                        fdata.push(c);
                        while let Some(c) = chars.next_if(|&c| c.is_ascii_digit() || c == delim) {
                            fdata.push(c);
                        }
                    }
                } else {
                    ftype = TokenFieldType::Date;
                    while let Some(c) = chars.next_if(|&c| c.is_ascii_alphanumeric() || c == delim)
                    {
                        fdata.push(c.to_ascii_lowercase());
                    }
                }
            // otherwise, number only and will determine year, month, day, or concatenated fields
            // later..
            } else {
                ftype = TokenFieldType::Number;
            }
        // Leading decimal point? Then fractional seconds...
        } else if let Some(c) = chars.next_if_eq(&'.') {
            fdata.push(c);
            while let Some(c) = chars.next_if(|&c| c.is_ascii_digit()) {
                fdata.push(c);
            }
            ftype = TokenFieldType::Number;
        // text? then date string, month, day of week, special, or timezone
        } else if let Some(c) = chars.next_if(|c| c.is_ascii_alphabetic()) {
            ftype = TokenFieldType::String;

            fdata.push(c.to_ascii_lowercase());
            while let Some(c) = chars.next_if(|&c| c.is_ascii_alphabetic()) {
                fdata.push(c.to_ascii_lowercase());
            }
            // Dates can have embedded '-', '/', or '.' separators.  It could also be a timezone
            // name containing embedded '/', '+', '-', '_', or ':' (but '_' or ':' can't be the
            // first punctuation). If the next character is a digit or '+', we need to check
            // whether what we have so far is a recognized non-timezone keyword --- if so, don't
            // believe that this is the start of a timezone.
            let mut is_date = false;
            if matches!(*chars.peek().unwrap(), '-' | '/' | '.') {
                is_date = true;
            } else if *chars.peek().unwrap() == '+' || chars.peek().unwrap().is_ascii_digit() {
                // we need search only the core token table, not TZ names
                if DATE_TOKEN_TABLE
                    .binary_search_by(|tk| tk.token.cmp(&*fdata))
                    .is_err()
                {
                    is_date = true;
                }
            }
            if is_date {
                ftype = TokenFieldType::Date;
                fdata.push(chars.next().unwrap().to_ascii_lowercase());
                while let Some(c) = chars.next_if(|&c| {
                    c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '/' | '_' | '.' | ':')
                }) {
                    fdata.push(c.to_ascii_lowercase());
                }
            }
        // sign? then special or numeric timezone
        } else if let Some(c) = chars.next_if(|&c| c == '+' || c == '-') {
            fdata.push(c);
            // soak up leading whitespace
            while chars.next_if(|c| c.is_ascii_whitespace()).is_some() {}
            // numeric timezone?
            // note that "DTK_TZ" could also be a signed float or yyyy-mm
            if let Some(c) = chars.next_if(|c| c.is_ascii_digit()) {
                ftype = TokenFieldType::Tz;
                fdata.push(c);
                while let Some(c) =
                    chars.next_if(|&c| c.is_ascii_digit() || matches!(c, ':' | '.' | '-'))
                {
                    fdata.push(c.to_ascii_lowercase());
                }
            // special?
            } else if let Some(c) = chars.next_if(|c| c.is_ascii_alphabetic()) {
                ftype = TokenFieldType::Special;

                fdata.push(c.to_ascii_lowercase());
                while let Some(c) = chars.next_if(|&c| c.is_ascii_alphabetic()) {
                    fdata.push(c.to_ascii_lowercase());
                }
            // otherwise something wrong...
            } else {
                return Err(-1);
            }
        // ignore other punctuation but use as delimiter
        } else if chars.next_if(|c| c.is_ascii_punctuation()).is_some() {
            continue;
        // otherwise, something is not right...
        } else {
            return Err(-1);
        }
        ret.push((fdata, ftype));
    }
    Ok(ret)
}

/// Interprets previously parsed fields for general date and time.
/// Return 0 if full date, 1 if only time, and negative DTERR code if problems.
/// (Currently, all callers treat 1 as an error return too.)
///
///  External format(s):
///          "<weekday> <month>-<day>-<year> <hour>:<minute>:<second>"
///          "Fri Feb-7-1997 15:23:27"
///          "Feb-7-1997 15:23:27"
///          "2-7-1997 15:23:27"
///          "1997-2-7 15:23:27"
///          "1997.038 15:23:27"        (day of year 1-366)
///  Also supports input in compact time:
///          "970207 152327"
///          "97038 152327"
///          "20011225T040506.789-07"
///
/// Use the system-provided functions to get the current time zone
/// if not specified in the input string.
///
/// If the date is outside the range of pg_time_t (in practice that could only
/// happen if pg_time_t is just 32 bits), then assume UTC time zone - thomas
/// 1997-05-27

pub fn DecodeDateTime(
    field: &[&str],
    ftype: &[i32],
    nf: usize,
    dtype: &mut TokenFieldType,
    mut tm: &mut pg_tm,
    fsec: &mut fsec_t,
    mut tzp: Option<&mut i32>,
) -> i32 {
    let mut fmask = FieldMask::none();
    let mut tmask = FieldMask::none();
    let mut ptype = TokenFieldType::Number; // "prefix type" for ISO y2001m02d04 format
    let mut val: i32 = 0;
    let mut mer: i32 = 2;
    let mut haveTextMonth = false;
    let mut isjulian = false;
    let mut is2digits = false;
    let mut bc = false;
    let mut namedTz: Option<&pg_tz> = None;
    let mut abbrevTz: Option<&pg_tz> = None;
    let mut valtz: Option<&pg_tz> = None;
    let mut abbrev: Option<&str> = None;
    let mut cur_tm: pg_tm = pg_tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: Some(false),
        tm_gmtoff: 0,
        tm_zone: None,
    };

    // We'll insist on at least all of the date fields, but initialize the
    // remaining fields in case they are not set later...
    *dtype = TokenFieldType::Date;
    tm.tm_hour = 0;
    tm.tm_min = 0;
    tm.tm_sec = 0;
    *fsec = 0;

    // don't know daylight savings time status apriori
    tm.tm_isdst = None;
    if let Some(tzp) = tzp.as_mut() {
        **tzp = 0;
    }
    let mut current_block_236: u64;
    for i in 0..nf {
        match ftype[i] {
            2 => {
                if ptype == TokenFieldType::Julian {
                    let mut cp = field[i];
                    let tzp = match tzp.as_mut() {
                        Some(tzp) => tzp,
                        None => {
                            eprintln!("tzp is null");
                            return -1;
                        }
                    };
                    let val_0 = match strtoint_rust(field[i], &mut cp) {
                        Ok(val) => val,
                        Err(_) => return -2,
                    };
                    j2date(val_0, &mut tm.tm_year, &mut tm.tm_mon, &mut tm.tm_mday);
                    isjulian = true;
                    let dterr = DecodeTimezone_rust(cp, tzp);
                    if dterr != 0 {
                        return dterr;
                    }
                    tmask = *FIELD_MASK_DATE | *FIELD_MASK_TIME | FieldType::Tz;
                    ptype = TokenFieldType::Number;
                // Already have a date? Then this might be a time zone name
                // with embedded punctuation (e.g. "America/New_York") or a
                // run-together time with trailing time zone (e.g. hhmmss-zz).
                // - thomas 2001-12-25
                //
                // We consider it a time zone if we already have month & day.
                // This is to allow the form "mmm dd hhmmss tz year", which
                // we've historically accepted.
                } else if ptype != TokenFieldType::Number
                    || fmask.contains(FieldType::Month | FieldType::Day)
                {
                    // No time zone accepted? Then quit...
                    let tzp = match tzp.as_mut() {
                        Some(tzp) => tzp,
                        None => {
                            eprintln!("tzp is null");
                            return -1;
                        }
                    };
                    if field[i].starts_with(|c: char| c.is_ascii_digit())
                        || ptype != TokenFieldType::Number
                    {
                        if ptype != TokenFieldType::Number {
                            if ptype != TokenFieldType::Time {
                                eprintln!("ptype is not Time: {:?}", ptype);
                                return -1;
                            }
                            ptype = TokenFieldType::Number;
                        }
                        // Starts with a digit but we already have a time
                        // field? Then we are in trouble with a date and time
                        // already...
                        if fmask.contains(*FIELD_MASK_TIME) {
                            eprintln!("started with a digit but already have a time");
                            return -1;
                        }
                        let (prefix, cp_0) = match field[i].find('-') {
                            Some(idx) => field[i].split_at(idx),
                            None => {
                                eprintln!("couldn't find '-' character");
                                return -1;
                            }
                        };
                        let dterr = DecodeTimezone_rust(cp_0, tzp);
                        if dterr != 0 {
                            return dterr;
                        }
                        let dterr = DecodeNumberField_rust(
                            prefix,
                            fmask,
                            &mut tmask,
                            tm,
                            fsec,
                            &mut is2digits,
                        );
                        if dterr < 0 {
                            return dterr;
                        }
                        // modify tmask after returning from DecodeNumberField()
                        tmask.set(FieldType::Tz);
                    } else {
                        namedTz = pg_tzset_rust(field[i]);
                        if namedTz.is_none() {
                            // ereport(ERROR,
                            // 		(errcode(ERRCODE_INVALID_PARAMETER_VALUE),
                            // 		 errmsg("time zone \"%s\" not recognized",
                            // 				field[i])));
                        }
                        // we'll apply the zone setting below
                        tmask = FieldMask::from(FieldType::Tz);
                    }
                } else {
                    let dterr = DecodeDate_rust(field[i], fmask, &mut tmask, &mut is2digits, tm);
                    if dterr != 0 {
                        return dterr;
                    }
                }
                current_block_236 = 13797367574128857302;
            }
            3 => {
                if ptype != TokenFieldType::Number {
                    if ptype != TokenFieldType::Time {
                        eprintln!("ptype is not Time: {:?}", ptype);
                        return -1;
                    }
                    ptype = TokenFieldType::Number
                }
                let dterr = DecodeTime_rust(field[i], 0x7fff, &mut tmask, tm, fsec);
                if dterr != 0 {
                    return dterr;
                }
                if time_overflows(tm.tm_hour, tm.tm_min, tm.tm_sec, *fsec) {
                    return -2;
                }
                current_block_236 = 13797367574128857302;
            }
            4 => {
                let mut tz: i32 = 0;
                let tzp = match tzp.as_mut() {
                    Some(tzp) => tzp,
                    None => {
                        eprintln!("tzp is null");
                        return -1;
                    }
                };
                let dterr = DecodeTimezone_rust(field[i], &mut tz);
                if dterr != 0 {
                    return dterr;
                }
                **tzp = tz;
                tmask = FieldMask::from(FieldType::Tz);
                current_block_236 = 13797367574128857302;
            }
            0 => {
                if ptype != TokenFieldType::Number {
                    let mut cp_1 = field[i];
                    let val_1 = match strtoint_rust(field[i], &mut cp_1) {
                        Ok(val) => val,
                        Err(_) => return -2,
                    };
                    if cp_1.starts_with('.') {
                        match ptype {
                            TokenFieldType::Julian
                            | TokenFieldType::Time
                            | TokenFieldType::Second => {}
                            _ => {
                                eprintln!("ptype is not Julian, Time, or Second: {:?}", ptype);
                                return -1;
                            }
                        }
                    } else if !cp_1.is_empty() {
                        eprintln!("expected EOF");
                        return -1;
                    }
                    match ptype {
                        TokenFieldType::Year => {
                            tm.tm_year = val_1;
                            tmask = FieldMask::from(FieldType::Year);
                        }
                        TokenFieldType::Month => {
                            // already have a month and hour? then assume minutes
                            if fmask.contains(FieldType::Month | FieldType::Hour) {
                                tm.tm_min = val_1;
                                tmask = FieldMask::from(FieldType::Minute);
                            } else {
                                tm.tm_mon = val_1;
                                tmask = FieldMask::from(FieldType::Month);
                            }
                        }
                        TokenFieldType::Day => {
                            tm.tm_mday = val_1;
                            tmask = FieldMask::from(FieldType::Day);
                        }
                        TokenFieldType::Hour => {
                            tm.tm_hour = val_1;
                            tmask = FieldMask::from(FieldType::Hour);
                        }
                        TokenFieldType::Minute => {
                            tm.tm_min = val_1;
                            tmask = FieldMask::from(FieldType::Minute);
                        }
                        TokenFieldType::Second => {
                            tm.tm_sec = val_1;
                            tmask = FieldMask::from(FieldType::Second);
                            if cp_1.starts_with('.') {
                                let dterr = ParseFractionalSecond_rust(cp_1, fsec);
                                if dterr != 0 {
                                    return dterr;
                                }
                                tmask = *FIELD_MASK_ALL_SECS;
                            }
                        }
                        TokenFieldType::Tz => {
                            let tzp = match tzp.as_mut() {
                                Some(tzp) => tzp,
                                None => {
                                    eprintln!("tzp is null");
                                    return -1;
                                }
                            };
                            tmask = FieldMask::from(FieldType::Tz);
                            let dterr = DecodeTimezone_rust(field[i], tzp);
                            if dterr != 0 {
                                return dterr;
                            }
                        }
                        TokenFieldType::Julian => {
                            if val_1 < 0 {
                                return -2;
                            }
                            tmask = *FIELD_MASK_DATE;
                            j2date(val_1, &mut tm.tm_year, &mut tm.tm_mon, &mut tm.tm_mday);
                            isjulian = true;
                            if cp_1.starts_with('.') {
                                let mut time = match f64::from_str(cp_1) {
                                    Ok(val) => val,
                                    Err(_) => return -1,
                                };
                                time *= 86400000000.0;
                                dt2time(
                                    time as Timestamp,
                                    &mut tm.tm_hour,
                                    &mut tm.tm_min,
                                    &mut tm.tm_sec,
                                    fsec,
                                );
                                tmask.set(*FIELD_MASK_TIME);
                            }
                        }
                        TokenFieldType::Time => {
                            let dterr = DecodeNumberField_rust(
                                field[i],
                                fmask | *FIELD_MASK_DATE,
                                &mut tmask,
                                tm,
                                fsec,
                                &mut is2digits,
                            );
                            if dterr < 0 {
                                return dterr;
                            }
                            if tmask != *FIELD_MASK_TIME {
                                eprintln!("tmask is not FIELD_MASK_TIME");
                                return -1;
                            }
                        }
                        typ => {
                            eprintln!("unexpected ptype: {:?}", typ);
                            return -1;
                        }
                    }
                    ptype = TokenFieldType::Number;
                    *dtype = TokenFieldType::Date;
                } else {
                    let flen = field[i].len();
                    let cp_2 = field[i].find('.').map(|idx| &field[i][idx..]);
                    // Embedded decimal and no date yet?
                    if !cp_2.is_none() && !fmask.intersects(*FIELD_MASK_DATE) {
                        let dterr =
                            DecodeDate_rust(field[i], fmask, &mut tmask, &mut is2digits, tm);
                        if dterr != 0 {
                            return dterr;
                        }
                    // embedded decimal and several digits before?
                    } else if !cp_2.is_none()
                        && (flen as u64).wrapping_sub(cp_2.unwrap().len() as u64) > 2
                    {
                        // Interpret as a concatenated date or time Set the type field to allow
                        // decoding other fields later.
                        // Example: 20011223 or 040506
                        let dterr = DecodeNumberField_rust(
                            field[i],
                            fmask,
                            &mut tmask,
                            tm,
                            fsec,
                            &mut is2digits,
                        );
                        if dterr < 0 {
                            return dterr;
                        }
                    // Is this a YMD or HMS specification, or a year number? YMD and HMS are
                    // required to be six digits or more, so if it is 5 digits, it is a year.  If
                    // it is six or more digits, we assume it is YMD or HMS unless no date and no
                    // time values have been specified.  This forces 6+ digit years to be at the
                    // end of the string, or to use the ISO date specification.
                    } else if flen >= 6
                        && (!fmask.intersects(*FIELD_MASK_DATE)
                            || !fmask.intersects(*FIELD_MASK_TIME))
                    {
                        let dterr = DecodeNumberField_rust(
                            field[i],
                            fmask,
                            &mut tmask,
                            tm,
                            fsec,
                            &mut is2digits,
                        );
                        if dterr < 0 {
                            return dterr;
                        }
                    // otherwise it is a single date/time field...
                    } else {
                        let dterr = DecodeNumber_rust(
                            field[i],
                            haveTextMonth,
                            fmask,
                            &mut tmask,
                            tm,
                            fsec,
                            &mut is2digits,
                        );
                        if dterr != 0 {
                            return dterr;
                        }
                    }
                }
                current_block_236 = 13797367574128857302;
            }
            1 | 6 => {
                let mut type_0 = DecodeTimezoneAbbrev_rust(field[i], &mut val, &mut valtz);
                if type_0 == FieldType::UnknownField {
                    type_0 = DecodeSpecial_rust(field[i], &mut val);
                }
                if type_0 == FieldType::IgnoreDtf {
                    current_block_236 = 12209867499936983673;
                } else {
                    tmask = FieldMask::from(type_0);
                    match type_0 {
                        FieldType::Reserved => match val {
                            12 => {
                                tmask = *FIELD_MASK_DATE | *FIELD_MASK_TIME | FieldType::Tz;
                                *dtype = TokenFieldType::Date;
                                let tzp = match tzp {
                                    Some(ref mut tzp) => Some(&mut **tzp),
                                    None => None,
                                };
                                GetCurrentTimeUsec(tm, fsec, tzp);
                            }
                            13 => {
                                tmask = *FIELD_MASK_DATE;
                                *dtype = TokenFieldType::Date;
                                GetCurrentDateTime(&mut cur_tm);
                                j2date(
                                    date2j(cur_tm.tm_year, cur_tm.tm_mon, cur_tm.tm_mday) - 1,
                                    &mut tm.tm_year,
                                    &mut tm.tm_mon,
                                    &mut tm.tm_mday,
                                );
                            }
                            14 => {
                                tmask = *FIELD_MASK_DATE;
                                *dtype = TokenFieldType::Date;
                                GetCurrentDateTime(&mut cur_tm);
                                tm.tm_year = cur_tm.tm_year;
                                tm.tm_mon = cur_tm.tm_mon;
                                tm.tm_mday = cur_tm.tm_mday;
                            }
                            15 => {
                                tmask = *FIELD_MASK_DATE;
                                *dtype = TokenFieldType::Date;
                                GetCurrentDateTime(&mut cur_tm);
                                j2date(
                                    date2j(cur_tm.tm_year, cur_tm.tm_mon, cur_tm.tm_mday) + 1,
                                    &mut tm.tm_year,
                                    &mut tm.tm_mon,
                                    &mut tm.tm_mday,
                                );
                            }
                            16 => {
                                tmask = *FIELD_MASK_TIME | FieldType::Tz;
                                *dtype = TokenFieldType::Date;
                                tm.tm_hour = 0;
                                tm.tm_min = 0;
                                tm.tm_sec = 0;
                                if let Some(tzp) = tzp.as_mut() {
                                    **tzp = 0;
                                }
                            }
                            _ => {
                                *dtype = val.try_into().unwrap();
                            }
                        },
                        FieldType::Month => {
                            // already have a (numeric) month? then see if we can substitute...
                            if fmask.contains(FieldType::Month)
                                && !haveTextMonth
                                && !fmask.contains(FieldType::Day)
                                && tm.tm_mon >= 1
                                && tm.tm_mon <= 31
                            {
                                tm.tm_mday = tm.tm_mon;
                                tmask = FieldMask::from(FieldType::Day);
                            }
                            haveTextMonth = true;
                            tm.tm_mon = val;
                        }
                        FieldType::DtzMod => {
                            tmask.set(FieldType::DTz);
                            tm.tm_isdst = Some(true);
                            let tzp = match tzp.as_mut() {
                                Some(tzp) => tzp,
                                None => {
                                    return -1;
                                }
                            };
                            **tzp -= val;
                        }
                        FieldType::DTz => {
                            tmask.set(FieldType::Tz);
                            tm.tm_isdst = Some(true);
                            let tzp = match tzp.as_mut() {
                                Some(tzp) => tzp,
                                None => {
                                    return -1;
                                }
                            };
                            **tzp = -val;
                        }
                        FieldType::Tz => {
                            tm.tm_isdst = Some(false);
                            let tzp = match tzp.as_mut() {
                                Some(tzp) => tzp,
                                None => {
                                    return -1;
                                }
                            };
                            **tzp = -val;
                        }
                        FieldType::DynTz => {
                            tmask.set(FieldType::Tz);
                            if tzp.is_none() {
                                return -1;
                            }
                            abbrevTz = valtz;
                            abbrev = Some(field[i]);
                        }
                        FieldType::AmPm => {
                            mer = val;
                        }
                        FieldType::Adbc => {
                            bc = val == 1;
                        }
                        FieldType::Dow => {
                            tm.tm_wday = val;
                        }
                        FieldType::Units => {
                            tmask = FieldMask::none();
                            ptype = val.try_into().unwrap();
                        }
                        FieldType::IsoTime => {
                            // This is a filler field "t" indicating that the next
                            // field is time. Try to verify that this is sensible.
                            tmask = FieldMask::none();

                            // No preceding date? Then quit...
                            if !fmask.contains(*FIELD_MASK_DATE) {
                                eprintln!("no preceding date");
                                return -1;
                            }

                            // We will need one of the following fields:
                            //    DTK_NUMBER should be hhmmss.fff
                            //    DTK_TIME should be hh:mm:ss.fff
                            //    DTK_DATE should be hhmmss-zz
                            if i >= nf - 1
                                || ftype[i + 1] != 0 && ftype[i + 1] != 3 && ftype[i + 1] != 2
                            {
                                eprintln!("next field are not the right type");
                                return -1;
                            }
                            ptype = val.try_into().unwrap();
                        }
                        FieldType::UnknownField => {
                            // Before giving up and declaring error, check to see
                            // if it is an all-alpha timezone name.
                            namedTz = pg_tzset_rust(field[i]);
                            if namedTz.is_none() {
                                eprintln!("namedTz is null");
                                return -1;
                            }
                            tmask = FieldMask::from(FieldType::Tz);
                        }
                        typ => {
                            eprintln!("unexpected field type: {:?}", typ);
                            return -1;
                        }
                    }
                    current_block_236 = 13797367574128857302;
                }
            }
            _ => return -1,
        }
        if current_block_236 == 13797367574128857302 {
            if tmask.intersects(fmask) {
                return -1;
            }
            fmask |= tmask;
        }
    }
    // do final checking/adjustment of Y/M/D fields
    let dterr = ValidateDate(fmask, isjulian, is2digits, bc, tm);
    if dterr != 0 {
        return dterr;
    }
    // handle AM/PM
    if mer != 2 && tm.tm_hour > 24 / 2 {
        return -2;
    }
    if mer == 0 && tm.tm_hour == 24 / 2 {
        tm.tm_hour = 0;
    } else if mer == 1 && tm.tm_hour != 24 / 2 {
        tm.tm_hour += 24 / 2;
    }
    // do additional checking for full date specs...
    if *dtype == TokenFieldType::Date {
        if !fmask.contains(*FIELD_MASK_DATE) {
            if fmask.contains(*FIELD_MASK_TIME) {
                // TODO(petrosagg): this is actually success, as noted in the function doc
                return 1;
            }
            return -1;
        }
        // If we had a full timezone spec, compute the offset (we could not do
        // it before, because we need the date to resolve DST status).
        if let Some(namedTz) = namedTz {
            // daylight savings time modifier disallowed with full TZ
            if fmask.contains(FieldType::DtzMod) {
                return -1;
            }
            let tzp = match tzp.as_mut() {
                Some(tzp) => tzp,
                None => {
                    return -1;
                }
            };
            **tzp = DetermineTimeZoneOffset(tm, namedTz);
        }
        // Likewise, if we had a dynamic timezone abbreviation, resolve it now.
        if let Some(abbrevTz) = abbrevTz {
            if fmask.contains(FieldType::DtzMod) {
                return -1;
            }
            let tzp = match tzp.as_mut() {
                Some(tzp) => tzp,
                None => {
                    return -1;
                }
            };
            **tzp = DetermineTimeZoneAbbrevOffset(tm, abbrev.unwrap(), abbrevTz);
        }
        // timezone not specified? then use session timezone
        if let Some(tzp) = tzp {
            if !fmask.contains(FieldType::Tz) {
                // daylight savings time modifier but no standard timezone? then error
                if fmask.contains(FieldType::DtzMod) {
                    return -1;
                }
                *tzp = DetermineTimeZoneOffset(tm, &session_timezone);
            }
        }
    }
    0
}

fn DetermineTimeZoneOffset(tm: &mut pg_tm, tzp: &pg_tz) -> i32 {
    let mut t: pg_time_t = 0;
    DetermineTimeZoneOffsetInternal(tm, tzp, &mut t)
}

fn DetermineTimeZoneOffsetInternal(mut tm: &mut pg_tm, tzp: &pg_tz, tp: &mut pg_time_t) -> i32 {
    let mut boundary: pg_time_t = 0;
    let mut before_gmtoff: i64 = 0;
    let mut after_gmtoff: i64 = 0;
    let mut before_isdst = false;
    let mut after_isdst = false;
    if (tm.tm_year > -4713 || tm.tm_year == -4713 && tm.tm_mon >= 11)
        && (tm.tm_year < 5874898 || tm.tm_year == 5874898 && tm.tm_mon < 6)
    {
        let date = date2j(tm.tm_year, tm.tm_mon, tm.tm_mday) - 2440588;
        let day = date as pg_time_t * 86400;
        if day / 86400 == date as i64 {
            let sec = tm.tm_sec + (tm.tm_min + tm.tm_hour * 60) * 60;
            let mytime = day + sec as i64;
            if !(mytime < 0 && day > 0) {
                let prevtime = mytime - 86400;
                if !(mytime < 0 && prevtime > 0) {
                    let res = pg_next_dst_boundary(
                        &prevtime,
                        &mut before_gmtoff,
                        &mut before_isdst,
                        &mut boundary,
                        &mut after_gmtoff,
                        &mut after_isdst,
                        tzp,
                    );
                    if res >= 0 {
                        if res == 0 {
                            tm.tm_isdst = Some(before_isdst);
                            *tp = mytime - before_gmtoff;
                            return -(before_gmtoff as i32);
                        }
                        let beforetime = mytime - before_gmtoff;
                        if !(before_gmtoff > 0 && mytime < 0 && beforetime > 0
                            || before_gmtoff <= 0 && mytime > 0 && beforetime < 0)
                        {
                            let aftertime = mytime - after_gmtoff;
                            if !(after_gmtoff > 0 && mytime < 0 && aftertime > 0
                                || after_gmtoff <= 0 && mytime > 0 && aftertime < 0)
                            {
                                if beforetime < boundary && aftertime < boundary {
                                    tm.tm_isdst = Some(before_isdst);
                                    *tp = beforetime;
                                    return -(before_gmtoff as i32);
                                }
                                if beforetime > boundary && aftertime >= boundary {
                                    tm.tm_isdst = Some(after_isdst);
                                    *tp = aftertime;
                                    return -(after_gmtoff as i32);
                                }
                                if beforetime > aftertime {
                                    tm.tm_isdst = Some(before_isdst);
                                    *tp = beforetime;
                                    return -(before_gmtoff as i32);
                                }
                                tm.tm_isdst = Some(after_isdst);
                                *tp = aftertime;
                                return -(after_gmtoff as i32);
                            }
                        }
                    }
                }
            }
        }
    }
    tm.tm_isdst = Some(false);
    *tp = 0 as pg_time_t;
    0
}

fn DetermineTimeZoneAbbrevOffset(mut tm: &mut pg_tm, abbr: &str, tzp: &pg_tz) -> i32 {
    let mut t: pg_time_t = 0;
    let mut abbr_offset: i32 = 0;
    let mut abbr_isdst = false;
    let zone_offset = DetermineTimeZoneOffsetInternal(tm, tzp, &mut t);
    if DetermineTimeZoneAbbrevOffsetInternal(t, abbr, tzp, &mut abbr_offset, &mut abbr_isdst) {
        tm.tm_isdst = Some(abbr_isdst);
        return abbr_offset;
    }
    zone_offset
}

fn DetermineTimeZoneAbbrevOffsetInternal(
    t: pg_time_t,
    abbr: &str,
    tzp: &pg_tz,
    offset: &mut i32,
    isdst: &mut bool,
) -> bool {
    // TODO: the original code re-used stack space to store the temporary uppercased abbreviation
    let upabbr = abbr.to_uppercase();
    let mut gmtoff: i64 = 0;
    if pg_interpret_timezone_abbrev(&upabbr, &t, &mut gmtoff, isdst, tzp) {
        *offset = -gmtoff as i32;
        return true;
    }
    false
}

unsafe fn DecodeDate(
    str: *mut libc::c_char,
    fmask: FieldMask,
    tmask: &mut FieldMask,
    is2digits: &mut bool,
    tm: &mut pg_tm,
) -> i32 {
    let str = std::ffi::CStr::from_ptr(str).to_str().unwrap();
    DecodeDate_rust(str, fmask, tmask, is2digits, tm)
}

fn DecodeDate_rust(
    mut str: &str,
    mut fmask: FieldMask,
    tmask: &mut FieldMask,
    is2digits: &mut bool,
    mut tm: &mut pg_tm,
) -> i32 {
    let mut fsec: fsec_t = 0;
    let mut nf: usize = 0;
    let mut haveTextMonth: bool = false;
    let mut val: i32 = 0;
    let mut dmask = FieldMask::none();
    let mut fields: [Option<&str>; 25] = [None; 25];
    *tmask = FieldMask::none();
    while !str.is_empty() && nf < 25 {
        while !str.is_empty() && !str.starts_with(|c: char| c.is_ascii_alphanumeric()) {
            str = &str[1..];
        }
        if str.is_empty() {
            return -1;
        }
        fields[nf] = Some(str);
        if str.starts_with(|c: char| c.is_ascii_digit()) {
            while str.starts_with(|c: char| c.is_ascii_digit()) {
                str = &str[1..];
            }
        } else if str.starts_with(|c: char| c.is_ascii_alphabetic()) {
            while str.starts_with(|c: char| c.is_ascii_alphabetic()) {
                str = &str[1..];
            }
        }
        if !str.is_empty() {
            let field = fields[nf].unwrap();
            fields[nf] = Some(&field[0..field.len() - str.len()]);
            str = &str[1..];
        }
        nf += 1;
    }
    for i in 0..nf {
        if fields[i]
            .unwrap()
            .starts_with(|c: char| c.is_ascii_alphabetic())
        {
            let type_0 = DecodeSpecial_rust(fields[i].unwrap(), &mut val);
            if type_0 != FieldType::IgnoreDtf {
                dmask = FieldMask::from(type_0);
                match type_0 {
                    FieldType::Month => {
                        tm.tm_mon = val;
                        haveTextMonth = true;
                    }
                    typ => {
                        eprintln!("unexpected field type: {:?}", typ);
                        return -1;
                    }
                }
                if fmask.intersects(dmask) {
                    return -1;
                }
                fmask |= dmask;
                *tmask |= dmask;
                fields[i] = None;
            }
        }
    }
    for i in 0..nf {
        if let Some(field) = fields[i] {
            let len = field.len() as i32;
            if len <= 0 {
                return -1;
            }
            let dterr = DecodeNumber_rust(
                field,
                haveTextMonth,
                fmask,
                &mut dmask,
                tm,
                &mut fsec,
                is2digits,
            );
            if dterr != 0 {
                return dterr;
            }
            if fmask.intersects(dmask) {
                return -1;
            }
            fmask |= dmask;
            *tmask |= dmask;
        }
    }
    if fmask & !(FieldType::Doy | FieldType::Tz) != *FIELD_MASK_DATE {
        return -1;
    }

    // validation of the field values must wait until ValidateDate()
    0
}

/// Check valid year/month/day values, handle BC and DOY cases Return 0 if okay, a DTERR code if not.

fn ValidateDate(
    fmask: FieldMask,
    isjulian: bool,
    is2digits: bool,
    bc: bool,
    mut tm: &mut pg_tm,
) -> i32 {
    if fmask.contains(FieldType::Year) && !isjulian {
        if bc {
            if tm.tm_year <= 0 {
                return -2;
            }
            tm.tm_year = -(tm.tm_year - 1);
        } else if is2digits {
            if tm.tm_year < 0 {
                return -2;
            }
            if tm.tm_year < 70 {
                tm.tm_year += 2000;
            } else if tm.tm_year < 100 {
                tm.tm_year += 1900;
            }
        } else if tm.tm_year <= 0 {
            return -2;
        }
    }
    // now that we have correct year, decode DOY
    if fmask.contains(FieldType::Doy) {
        j2date(
            date2j(tm.tm_year, 1, 1) + tm.tm_yday - 1,
            &mut tm.tm_year,
            &mut tm.tm_mon,
            &mut tm.tm_mday,
        );
    }
    // check for valid month
    if fmask.contains(FieldType::Month) && (tm.tm_mon < 1 || tm.tm_mon > 12) {
        return -3;
    }
    // minimal check for valid day
    if fmask.contains(FieldType::Day) && (tm.tm_mday < 1 || tm.tm_mday > 31) {
        return -3;
    }
    if fmask.contains(*FIELD_MASK_DATE)
        && tm.tm_mday
            > day_tab
                [(tm.tm_year % 4 == 0 && (tm.tm_year % 100 != 0 || tm.tm_year % 400 == 0)) as usize]
                [(tm.tm_mon - 1) as usize]
    {
        return -2;
    }
    0
}

/// Decode time string which includes delimiters.
/// Return 0 if okay, a DTERR code if not.
///
/// Only check the lower limit on hours, since this same code can be
/// used to represent time spans.
unsafe fn DecodeTime(
    str: *mut libc::c_char,
    _fmask: FieldMask,
    range: i32,
    tmask: &mut FieldMask,
    tm: &mut pg_tm,
    fsec: &mut fsec_t,
) -> i32 {
    let str = std::ffi::CStr::from_ptr(str).to_str().unwrap();
    DecodeTime_rust(str, range, tmask, tm, fsec)
}

fn DecodeTime_rust(
    str: &str,
    range: i32,
    tmask: &mut FieldMask,
    mut tm: &mut pg_tm,
    fsec: &mut fsec_t,
) -> i32 {
    let mut cp = str;
    *tmask = *FIELD_MASK_TIME;
    tm.tm_hour = match strtoint_rust(str, &mut cp) {
        Ok(val) => val,
        Err(_) => return -2,
    };
    if !cp.starts_with(':') {
        return -1;
    }
    tm.tm_min = match strtoint_rust(&cp[1..], &mut cp) {
        Ok(val) => val,
        Err(_) => return -2,
    };
    if cp.is_empty() {
        tm.tm_sec = 0;
        *fsec = 0;
        if range == 1 << 11 | 1 << 12 {
            tm.tm_sec = tm.tm_min;
            tm.tm_min = tm.tm_hour;
            tm.tm_hour = 0;
        }
    } else if cp.starts_with('.') {
        let dterr = ParseFractionalSecond_rust(cp, fsec);
        if dterr != 0 {
            return dterr;
        }
        tm.tm_sec = tm.tm_min;
        tm.tm_min = tm.tm_hour;
        tm.tm_hour = 0;
    } else if cp.starts_with(':') {
        tm.tm_sec = match strtoint_rust(&cp[1..], &mut cp) {
            Ok(val) => val,
            Err(_) => return -2,
        };
        if cp.is_empty() {
            *fsec = 0;
        } else if cp.starts_with('.') {
            let dterr = ParseFractionalSecond_rust(cp, fsec);
            if dterr != 0 {
                return dterr;
            }
        } else {
            return -1;
        }
    } else {
        return -1;
    }
    if tm.tm_hour < 0
        || tm.tm_min < 0
        || tm.tm_min > 60 - 1
        || tm.tm_sec < 0
        || tm.tm_sec > 60
        || (*fsec as i64) < 0
        || *fsec as i64 > 1000000
    {
        return -2;
    }
    0
}

#[allow(clippy::too_many_arguments)]
unsafe fn DecodeNumber(
    flen: i32,
    str: *mut libc::c_char,
    haveTextMonth: bool,
    fmask: FieldMask,
    tmask: &mut FieldMask,
    tm: &mut pg_tm,
    fsec: &mut fsec_t,
    is2digits: &mut bool,
) -> i32 {
    let str = std::slice::from_raw_parts(str as *const u8, flen as usize);
    let str = std::str::from_utf8(str).unwrap();
    DecodeNumber_rust(str, haveTextMonth, fmask, tmask, tm, fsec, is2digits)
}

fn DecodeNumber_rust(
    str: &str,
    haveTextMonth: bool,
    fmask: FieldMask,
    tmask: &mut FieldMask,
    mut tm: &mut pg_tm,
    fsec: &mut fsec_t,
    is2digits: &mut bool,
) -> i32 {
    let mut cp = str;
    *tmask = FieldMask::none();
    let val = match strtoint_rust(str, &mut cp) {
        Ok(val) => val,
        Err(_) => return -2,
    };
    if cp == str {
        return -1;
    }
    if cp.starts_with('.') {
        if str.len() - cp.len() > 2 {
            let dterr =
                DecodeNumberField_rust(str, fmask | *FIELD_MASK_DATE, tmask, tm, fsec, is2digits);
            if dterr < 0 {
                return dterr;
            }
            return 0;
        }
        let dterr = ParseFractionalSecond_rust(cp, fsec);
        if dterr != 0 {
            return dterr;
        }
    } else if !cp.is_empty() {
        return -1;
    }
    /* Special case for day of year */
    if str.len() == 3
        && fmask & *FIELD_MASK_DATE == FieldMask::from(FieldType::Year)
        && (1..=366).contains(&val)
    {
        *tmask = FieldType::Doy | FieldType::Month | FieldType::Day;
        tm.tm_yday = val;
        // tm_mon and tm_mday can't actually be set yet ...
        return 0;
    }
    // Switch based on what we have so far
    match *(fmask & *FIELD_MASK_DATE) {
        0 => {
            if str.len() >= 3 || DateOrder == 0 {
                *tmask = FieldMask::from(FieldType::Year);
                tm.tm_year = val;
            } else if DateOrder == 1 {
                *tmask = FieldMask::from(FieldType::Day);
                tm.tm_mday = val;
            } else {
                *tmask = FieldMask::from(FieldType::Month);
                tm.tm_mon = val;
            }
        }
        4 => {
            // Must be at second field of YY-MM-DD
            *tmask = FieldMask::from(FieldType::Month);
            tm.tm_mon = val;
        }
        2 => {
            if haveTextMonth {
                if str.len() >= 3 || DateOrder == 0 {
                    *tmask = FieldMask::from(FieldType::Year);
                    tm.tm_year = val;
                } else {
                    *tmask = FieldMask::from(FieldType::Day);
                    tm.tm_mday = val;
                }
            } else {
                *tmask = FieldMask::from(FieldType::Day);
                tm.tm_mday = val;
            }
        }
        6 => {
            if haveTextMonth {
                // Need to accept DD-MON-YYYY even in YMD mode
                if str.len() >= 3 && *is2digits as i32 != 0 {
                    // Guess that first numeric field is day was wrong
                    // YEAR is already set
                    *tmask = FieldMask::from(FieldType::Day);
                    tm.tm_mday = tm.tm_year;
                    tm.tm_year = val;
                    *is2digits = false;
                } else {
                    *tmask = FieldMask::from(FieldType::Day);
                    tm.tm_mday = val;
                }
            } else {
                // Must be at third field of YY-MM-DD
                *tmask = FieldMask::from(FieldType::Day);
                tm.tm_mday = val;
            }
        }
        8 => {
            // Must be at second field of DD-MM-YY
            *tmask = FieldMask::from(FieldType::Month);
            tm.tm_mon = val;
        }
        10 => {
            *tmask = FieldMask::from(FieldType::Year);
            tm.tm_year = val;
        }
        14 => {
            let dterr = DecodeNumberField_rust(str, fmask, tmask, tm, fsec, is2digits);
            if dterr < 0 {
                return dterr;
            }
            return 0;
        }
        _ => return -1,
    }
    // When processing a year field, mark it for adjustment if it's only one or two digits.
    if *tmask == FieldMask::from(FieldType::Year) {
        *is2digits = str.len() <= 2;
    }
    0
}

unsafe fn DecodeNumberField(
    len: i32,
    str: *mut libc::c_char,
    fmask: FieldMask,
    tmask: &mut FieldMask,
    tm: &mut pg_tm,
    fsec: &mut fsec_t,
    is2digits: &mut bool,
) -> i32 {
    let str = std::slice::from_raw_parts(str as *const u8, len as usize);
    let str = std::str::from_utf8(str).unwrap();
    DecodeNumberField_rust(str, fmask, tmask, tm, fsec, is2digits)
}

fn DecodeNumberField_rust(
    mut str: &str,
    fmask: FieldMask,
    tmask: &mut FieldMask,
    mut tm: &mut pg_tm,
    fsec: &mut fsec_t,
    is2digits: &mut bool,
) -> i32 {
    match str.find('.') {
        // Have a decimal point? Then this is a date or something with a seconds field...
        Some(idx) => {
            // Can we use ParseFractionalSecond here?  Not clear whether trailing
            // junk should be rejected ...
            let frac = match f64::from_str(&str[idx..]) {
                Ok(frac) => frac,
                Err(_) => return -1,
            };
            *fsec = (frac * 1_000_000.0) as fsec_t;
            // Now truncate off the fraction for further processing
            str = &str[..idx];
        }
        // No decimal point and no complete date yet?
        None => {
            // Start from end and consider first 2 as Day, next 2 as Month, and the rest as Year.
            if !fmask.contains(*FIELD_MASK_DATE) && str.len() >= 6 {
                *tmask = *FIELD_MASK_DATE;
                tm.tm_mday = i32::from_str(&str[str.len() - 2..]).unwrap();
                tm.tm_mon = i32::from_str(&str[str.len() - 4..str.len() - 2]).unwrap();
                tm.tm_year = i32::from_str(&str[..str.len() - 4]).unwrap();
                if str.len() - 4 == 2 {
                    *is2digits = true;
                }
                return 2;
            }
        }
    }
    // not all time fields are specified?
    if !fmask.contains(*FIELD_MASK_TIME) {
        // hhmmss
        if str.len() == 6 {
            *tmask = *FIELD_MASK_TIME;
            tm.tm_sec = i32::from_str(&str[4..]).unwrap();
            tm.tm_min = i32::from_str(&str[2..4]).unwrap();
            tm.tm_hour = i32::from_str(&str[0..2]).unwrap();
            return 3;
        // hhmm?
        } else if str.len() == 4 {
            *tmask = *FIELD_MASK_TIME;
            tm.tm_sec = 0;
            tm.tm_min = i32::from_str(&str[2..]).unwrap();
            tm.tm_hour = i32::from_str(&str[0..2]).unwrap();
            return 3;
        }
    }
    -1
}

unsafe fn DecodeTimezone(str: *mut libc::c_char, tzp: &mut i32) -> i32 {
    let str = std::ffi::CStr::from_ptr(str).to_str().unwrap();
    DecodeTimezone_rust(str, tzp)
}

fn DecodeTimezone_rust(str: &str, tzp: &mut i32) -> i32 {
    let mut tz: i32;
    let min: i32;
    let mut sec: i32 = 0;
    let mut cp = str;
    if !str.starts_with('+') && !str.starts_with('-') {
        return -1;
    }
    let mut hr = match strtoint_rust(&str[1..], &mut cp) {
        Ok(hr) => hr,
        Err(_) => return -5,
    };
    if cp.starts_with(':') {
        min = match strtoint_rust(&cp[1..], &mut cp) {
            Ok(min) => min,
            Err(_) => return -5,
        };
        if cp.starts_with(':') {
            sec = match strtoint_rust(&cp[1..], &mut cp) {
                Ok(sec) => sec,
                Err(_) => return -5,
            };
        }
    } else if cp.is_empty() && str.len() > 3 {
        min = hr % 100;
        hr /= 100;
    } else {
        min = 0;
    }
    if !(0..=15).contains(&hr) {
        return -5;
    }
    if !(0..60).contains(&min) {
        return -5;
    }
    if !(0..60).contains(&sec) {
        return -5;
    }
    tz = (hr * 60 + min) * 60 + sec;
    if str.starts_with('-') {
        tz = -tz;
    }
    *tzp = -tz;
    if !cp.is_empty() {
        return -1;
    }
    0
}

unsafe fn DecodeTimezoneAbbrev(
    lowtoken: *mut libc::c_char,
    offset: &mut i32,
    tz: &mut Option<&pg_tz>,
) -> FieldType {
    let lowtoken = std::ffi::CStr::from_ptr(lowtoken).to_str().unwrap();
    let mut local_tz = None;
    let ret = DecodeTimezoneAbbrev_rust(lowtoken, offset, &mut local_tz);
    match local_tz {
        Some(t) => *tz = Some(t),
        None => *tz = None,
    };
    ret
}

fn DecodeTimezoneAbbrev_rust(
    lowtoken: &str,
    offset: &mut i32,
    tz: &mut Option<&pg_tz>,
) -> FieldType {
    match &ZONE_ABBREV_TABLE {
        Some(table) => match table.abbrevs.binary_search_by(|tk| tk.token.cmp(lowtoken)) {
            Ok(idx) => {
                let token = &table.abbrevs[idx];
                match token.typ {
                    FieldType::DynTz => {
                        *offset = 0;
                        *tz = FetchDynamicTimeZone(&table, token);
                    }
                    _ => {
                        *offset = token.value;
                        *tz = None;
                    }
                };
                token.typ
            }
            Err(_) => {
                *offset = 0;
                *tz = None;
                FieldType::UnknownField
            }
        },
        None => {
            *offset = 0;
            *tz = None;
            FieldType::UnknownField
        }
    }
}

unsafe fn DecodeSpecial(lowtoken: *mut libc::c_char, val: &mut i32) -> FieldType {
    let lowtoken = std::ffi::CStr::from_ptr(lowtoken).to_str().unwrap();
    DecodeSpecial_rust(lowtoken, val)
}

fn DecodeSpecial_rust(lowtoken: &str, val: &mut i32) -> FieldType {
    match DATE_TOKEN_TABLE.binary_search_by(|tk| tk.token.cmp(lowtoken)) {
        Ok(idx) => {
            let token = &DATE_TOKEN_TABLE[idx];
            *val = token.value;
            token.typ
        }
        Err(_) => {
            *val = 0;
            FieldType::UnknownField
        }
    }
}

fn FetchDynamicTimeZone<'a>(_tbl: &'a TimeZoneAbbrevTable, _tp: &DateToken) -> Option<&'a pg_tz> {
    // This is unimplemented because the C code was doing pointer weird pointer arithmetic to
    // relate the value of the token to an offset of a dynamic timezone definition in the zone
    // table.
    //
    // Revisit once the rest of the structure has been cleaned up
    unimplemented!()
    // let dtza = (tbl as *mut libc::c_char).offset((*tp).value as isize) as *mut DynamicZoneAbbrev;
    // if ((*dtza).tz).is_null() {
    //     (*dtza).tz = pg_tzset(((*dtza).zone).as_mut_ptr());
    //     if ((*dtza).tz).is_null() {
    //         let mut __errno_location_0: i32 = 0;
    //         if if 0 != 0 && 21 as i32 >= 21 as i32 {
    //             errstart_cold(21 as i32, 0 as *const libc::c_char) as i32
    //         } else {
    //             errstart(21 as i32, 0 as *const libc::c_char) as i32
    //         } != 0
    //         {
    //             errcode(
    //                 ('F' as i32 - '0' as i32 & 0x3f as i32)
    //                     + (('0' as i32 - '0' as i32 & 0x3f as i32) << 6 as i32)
    //                     + (('0' as i32 - '0' as i32 & 0x3f as i32) << 12 as i32)
    //                     + (('0' as i32 - '0' as i32 & 0x3f as i32) << 18 as i32)
    //                     + (('0' as i32 - '0' as i32 & 0x3f as i32) << 24 as i32),
    //             );
    //             errmsg(
    //                 b"time zone \"%s\" not recognized\0" as *const u8 as *const libc::c_char,
    //                 ((*dtza).zone).as_mut_ptr() as *mut _,
    //             );
    //             errdetail(
    //                 b"This time zone name appears in the configuration file for time zone abbreviation \"%s\".\0"
    //                     as *const u8 as *const libc::c_char,
    //                 ((*tp).token).as_ptr() as *mut _,
    //             );
    //             errfinish(
    //                 b"/home/petrosagg/projects/postgres-datetime/src/datetime.c\0" as *const u8
    //                     as *const libc::c_char,
    //                 4647 as i32,
    //                 (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
    //                     b"FetchDynamicTimeZone\0",
    //                 ))
    //                 .as_ptr(),
    //             );
    //         }
    //         if 0 != 0 && 21 as i32 >= 21 as i32 {
    //             unreachable!();
    //         }
    //     }
    // }
    // return (*dtza).tz;
}
