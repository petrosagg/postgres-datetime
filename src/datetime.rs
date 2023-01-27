use std::ffi::CString;

use crate::datetime_raw::{fsec_t, pg_tm, DateADT, DecodeDateTime};

#[derive(Debug)]
#[repr(i32)]
pub enum FieldType {
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

pub fn decode(fields: Vec<(String, FieldType)>) -> Result<(pg_tm, fsec_t, i32, i32), i32> {
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
