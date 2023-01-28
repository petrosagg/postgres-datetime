use std::ffi::CString;

use bitmask::bitmask;
use once_cell::sync::Lazy;

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
    pub mask FieldMask: u32 where

    #[derive(Debug)]
    flags RealFieldType {
        Reserved = 1 << 0,
        Month = 1 << 1,
        Year = 1 << 2,
        Day = 1 << 3,
        Julian = 1 << 4,
        Tz = 1 << 5,               /* fixed-offset timezone abbreviation */
        DTz = 1 << 6,               /* fixed-offset timezone abbrev, DST */
        DynTz = 1 << 7,               /* dynamic timezone abbreviation */
        IgnoreDtf = 1 << 8,
        AmPm = 1 << 9,
        Hour = 1 << 10,
        Minute = 1 << 11,
        Second = 1 << 12,
        Millisecond = 1 << 13,
        Microsecond = 1 << 14,
        Doy = 1 << 15,
        Dow = 1 << 16,
        Units = 1 << 17,
        Adbc = 1 << 18,
        /* these are only for relative dates */
        Ago = 1 << 19,
        AbsBefore = 1 << 20,
        AbsAfter = 1 << 21,
        /* generic fields to help with parsing */
        IsoDate = 1 << 22,
        IsoTime = 1 << 23,
        /* these are only for parsing intervals */
        Week = 1 << 24,
        Decade = 1 << 25,
        Century = 1 << 26,
        Millennium = 1 << 27,
        /* hack for parsing two-word timezone specs "MET DST" etc */
        DtzMod = 1 << 28,              /* "DST" as a separate word */
        /* reserved for unrecognized string values */
        UnknownField = 1 << 31,
    }
}

impl From<u32> for RealFieldType {
    fn from(n: u32) -> Self {
        match n {
            0b00000000_00000000_00000000_00000001 => Self::Reserved,
            0b00000000_00000000_00000000_00000010 => Self::Month,
            0b00000000_00000000_00000000_00000100 => Self::Year,
            0b00000000_00000000_00000000_00001000 => Self::Day,
            0b00000000_00000000_00000000_00010000 => Self::Julian,
            0b00000000_00000000_00000000_00100000 => Self::Tz,
            0b00000000_00000000_00000000_01000000 => Self::DTz,
            0b00000000_00000000_00000000_10000000 => Self::DynTz,
            0b00000000_00000000_00000001_00000000 => Self::IgnoreDtf,
            0b00000000_00000000_00000010_00000000 => Self::AmPm,
            0b00000000_00000000_00000100_00000000 => Self::Hour,
            0b00000000_00000000_00001000_00000000 => Self::Minute,
            0b00000000_00000000_00010000_00000000 => Self::Second,
            0b00000000_00000000_00100000_00000000 => Self::Millisecond,
            0b00000000_00000000_01000000_00000000 => Self::Microsecond,
            0b00000000_00000000_10000000_00000000 => Self::Doy,
            0b00000000_00000001_00000000_00000000 => Self::Dow,
            0b00000000_00000010_00000000_00000000 => Self::Units,
            0b00000000_00000100_00000000_00000000 => Self::Adbc,
            0b00000000_00001000_00000000_00000000 => Self::Ago,
            0b00000000_00010000_00000000_00000000 => Self::AbsBefore,
            0b00000000_00100000_00000000_00000000 => Self::AbsAfter,
            0b00000000_01000000_00000000_00000000 => Self::IsoDate,
            0b00000000_10000000_00000000_00000000 => Self::IsoTime,
            0b00000001_00000000_00000000_00000000 => Self::Week,
            0b00000010_00000000_00000000_00000000 => Self::Decade,
            0b00000100_00000000_00000000_00000000 => Self::Century,
            0b00001000_00000000_00000000_00000000 => Self::Millennium,
            0b00010000_00000000_00000000_00000000 => Self::DtzMod,
            0b10000000_00000000_00000000_00000000 => Self::UnknownField,
            n => panic!("unknown field {n}"),
        }
    }
}

pub static FIELD_MASK_ALL_SECS: Lazy<FieldMask> =
    Lazy::new(|| RealFieldType::Second | RealFieldType::Millisecond | RealFieldType::Microsecond);
pub static FIELD_MASK_DATE: Lazy<FieldMask> =
    Lazy::new(|| RealFieldType::Year | RealFieldType::Month | RealFieldType::Day);
pub static FIELD_MASK_TIME: Lazy<FieldMask> =
    Lazy::new(|| RealFieldType::Hour | RealFieldType::Minute | *FIELD_MASK_ALL_SECS);

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
