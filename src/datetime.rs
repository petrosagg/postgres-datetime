use std::ffi::CString;

use once_cell::sync::Lazy;
use bitmask::bitmask;

use crate::datetime_raw::{fsec_t, pg_tm, DateADT, DecodeDateTime};

#[derive(Debug)]
#[repr(i32)]
pub enum TokenFieldType {
    Number = 0,
    String = 1,
    Date = 2,
    Time = 3,
    Tz = 4,
    Ago = 5,
    Special = 6,
    Early = 9,
    Late = 10,
    Epoch = 11,
    Now = 12,
    Yesterday = 13,
    Today = 14,
    Tomorrow = 15,
    Zulu = 16,
    Delta = 17,
    Second = 18,
    Minute = 19,
    Hour = 20,
    Day = 21,
    Week = 22,
    Month = 23,
    Quarter = 24,
    Year = 25,
    Decade = 26,
    Century = 27,
    Millennium = 28,
    Millisec = 29,
    Microsec = 30,
    Julian = 31,
    Dow = 32,
    Doy = 33,
    TzHour = 34,
    TzMinute = 35,
    IsoYear = 36,
    IsoDow = 37,
}

// Field types for time decoding.
//
// Can't have more of these than there are bits in an unsigned int since these are turned into bit
// masks during parsing and decoding.
//
// Furthermore, the values for YEAR, MONTH, DAY, HOUR, MINUTE, SECOND must be in the range 0..14 so
// that the associated bitmasks can fit into the left half of an INTERVAL's typmod value.  Since
// those bits are stored in typmods, you can't change them without initdb!
bitmask! {
    pub mask FieldMask: i32 where flags RealFieldType {
        Reserved = 0,
        Month = 1,
        Year = 2,
        Day = 3,
        Julian = 4,
        Tz = 5,               /* fixed-offset timezone abbreviation */
        DTz = 6,               /* fixed-offset timezone abbrev, DST */
        DynTz = 7,               /* dynamic timezone abbreviation */
        IgnoreDtf = 8,
        AmPm = 9,
        Hour = 10,
        Minute = 11,
        Second = 12,
        Millisecond = 13,
        Microsecond = 14,
        Doy = 15,
        Dow = 16,
        Units = 17,
        Adbc = 18,
        /* these are only for relative dates */
        Ago = 19,
        AbsBefore = 20,
        AbsAfter = 21,
        /* generic fields to help with parsing */
        IsoDate = 22,
        IsoTime = 23,
        /* these are only for parsing intervals */
        Week = 24,
        Decade = 25,
        Century = 26,
        Millennium = 27,
        /* hack for parsing two-word timezone specs "MET DST" etc */
        DtzMod = 28,              /* "DST" as a separate word */
        /* reserved for unrecognized string values */
        UnknownField = 31,
    }
}

pub static FIELD_MASK_ALL_SECS: Lazy<FieldMask> = Lazy::new(|| RealFieldType::Second | RealFieldType::Millisecond | RealFieldType::Microsecond);
pub static FIELD_MASK_DATE: Lazy<FieldMask> = Lazy::new(|| RealFieldType::Year | RealFieldType::Month | RealFieldType::Day);
pub static FIELD_MASK_TIME: Lazy<FieldMask> = Lazy::new(|| RealFieldType::Hour | RealFieldType::Minute | *FIELD_MASK_ALL_SECS);

pub fn decode(fields: Vec<(String, TokenFieldType)>) -> Result<(pg_tm, fsec_t, i32, i32), i32> {
    let nf = fields.len() as i32;
    let mut field = vec![];
    let mut ftype = vec![];
    for (data, typ) in fields {
        field.push(CString::new(data).unwrap().into_raw());
        ftype.push(typ as i32);
    }

    let date: DateADT = 0;
    let mut fsec: fsec_t = 0;
    let mut dterr = 0i32;
    let mut tt = pg_tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut tzp: i32 = 0;
    let mut dtype: i32 = 0;
    unsafe {
        dterr = DecodeDateTime(
            field.as_mut_ptr(),
            ftype.as_mut_ptr(),
            nf,
            &mut dtype as *mut _,
            &mut tt as *mut _,
            &mut fsec as *mut _,
            &mut tzp as *mut _,
        );
    }
    if dterr != 0 {
        return Err(dterr);
    }

    Ok((tt, fsec, tzp, dtype))
}
