use std::ffi::CString;

use crate::datetime_raw::{fsec_t, pg_tm, DateADT, DecodeDateTime, ParseDateTime};

const MAXDATEFIELDS: usize = 25;
const MAXDATELEN: usize = 128;

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

impl From<i32> for FieldType {
    fn from(code: i32) -> Self {
        match code {
            0 => Self::Number,
            1 => Self::String,
            2 => Self::Date,
            3 => Self::Time,
            4 => Self::Tz,
            5 => Self::Ago,
            6 => Self::Special,
            9 => Self::Early,
            10 => Self::Late,
            11 => Self::Epoch,
            12 => Self::Now,
            13 => Self::Yesterday,
            14 => Self::Today,
            15 => Self::Tomorrow,
            16 => Self::Zulu,
            17 => Self::Delta,
            18 => Self::Second,
            19 => Self::Minute,
            20 => Self::Hour,
            21 => Self::Day,
            22 => Self::Week,
            23 => Self::Month,
            24 => Self::Quarter,
            25 => Self::Year,
            26 => Self::Decade,
            27 => Self::Century,
            28 => Self::Millennium,
            29 => Self::Millisec,
            30 => Self::Microsec,
            31 => Self::Julian,
            32 => Self::Dow,
            33 => Self::Doy,
            34 => Self::TzHour,
            35 => Self::TzMinute,
            36 => Self::IsoYear,
            37 => Self::IsoDow,
            _ => panic!(),
        }
    }
}

pub fn parse(s: &str) -> Result<Vec<(String, FieldType)>, i32> {
    let mut dterr = 0i32;
    let mut fields = Vec::with_capacity(MAXDATEFIELDS);
    let mut ftypes = Vec::with_capacity(MAXDATEFIELDS);

    if dterr != 0 {
        return Err(dterr);
    }

    unsafe {
        let workbuf = CString::from_vec_unchecked(vec![0; MAXDATELEN]);

        dterr = ParseDateTime(s, &mut fields, &mut ftypes, MAXDATEFIELDS as i32);

        let mut ret = Vec::with_capacity(fields.len());
        for (field, typ) in fields.into_iter().zip(ftypes) {
            let typ = FieldType::from(typ);
            ret.push((field, typ));
        }
        Ok(ret)
    }
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
