use ::libc;

use crate::datetime::{
    FieldMask, RealFieldType, TokenFieldType, FIELD_MASK_ALL_SECS, FIELD_MASK_DATE, FIELD_MASK_TIME,
};

const HOURS_PER_DAY: libc::c_int = 24;
const MINS_PER_HOUR: libc::c_int = 60;
const SECS_PER_DAY: libc::c_int = 86400;
const SECS_PER_HOUR: libc::c_int = 3600;
const SECS_PER_MINUTE: libc::c_int = 60;
const USECS_PER_DAY: libc::c_long = 86400000000;
const USECS_PER_HOUR: libc::c_long = 3600000000;
const USECS_PER_MINUTE: libc::c_long = 60000000;
const USECS_PER_SEC: libc::c_long = 1000000;
const POSTGRES_EPOCH_JDATE: libc::c_long = 2451545; /* == date2j(2000, 1, 1) */
const UNIX_EPOCH_JDATE: libc::c_long = 2440588; /* == date2j(1970, 1, 1) */

fn pg_toupper(mut ch: libc::c_uchar) -> libc::c_uchar {
    ch.make_ascii_uppercase();
    ch
}
static DateOrder: libc::c_int = 0;
fn dt2time(
    jd: Timestamp,
    hour: *mut libc::c_int,
    min: *mut libc::c_int,
    sec: *mut libc::c_int,
    fsec: *mut fsec_t,
) {
    unsafe {
        let mut time: TimeOffset;

        time = jd;

        *hour = (time / USECS_PER_HOUR).try_into().unwrap();
        time -= (*hour as i64) * USECS_PER_HOUR;
        *min = (time / USECS_PER_MINUTE).try_into().unwrap();
        time -= (*min as i64) * USECS_PER_MINUTE;
        *sec = (time / USECS_PER_SEC).try_into().unwrap();
        *fsec = (time - (*sec as i64 * USECS_PER_SEC)).try_into().unwrap();
    }
}
fn errstart(_elevel: libc::c_int, _domain: *const libc::c_char) -> bool {
    false
}
fn errstart_cold(_elevel: libc::c_int, _domain: *const libc::c_char) -> bool {
    false
}
fn errfinish(_filename: *const libc::c_char, _lineno: libc::c_int, _funcname: *const libc::c_char) {
}
fn errcode(_sqlerrcode: libc::c_int) -> libc::c_int {
    0
}
fn errmsg0(fmt: *const libc::c_char) -> libc::c_int {
    unsafe {
        let s = std::ffi::CStr::from_ptr(fmt);
        println!("{}", s.to_str().unwrap());
    }
    0
}
fn errmsg(fmt: *const libc::c_char, _arg: *mut libc::c_void) -> libc::c_int {
    unsafe {
        let s = std::ffi::CStr::from_ptr(fmt);
        println!("{}", s.to_str().unwrap());
    }
    0
}
fn errdetail(fmt: *const libc::c_char, _arg: *mut libc::c_void) -> libc::c_int {
    unsafe {
        let s = std::ffi::CStr::from_ptr(fmt);
        println!("{}", s.to_str().unwrap());
    }
    0
}
fn GetCurrentTransactionStartTimestamp() -> TimestampTz {
    11223344
}

fn pg_localtime(_timep: *const pg_time_t, _tz: *const pg_tz) -> *mut pg_tm {
    Box::into_raw(Box::new(pg_tm {
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
        tm_zone: std::ptr::null(),
    }))
}

fn pg_interpret_timezone_abbrev(
    _abbrev: *const libc::c_char,
    _timep: *const pg_time_t,
    _gmtoff: *mut libc::c_long,
    _isdst: &mut bool,
    _tz: *const pg_tz,
) -> bool {
    unimplemented!()
}
fn pg_next_dst_boundary(
    _timep: *const pg_time_t,
    _before_gmtoff: *mut libc::c_long,
    _before_isdst: &mut bool,
    _boundary: *mut pg_time_t,
    _after_gmtoff: *mut libc::c_long,
    _after_isdst: &mut bool,
    _tz: *const pg_tz,
) -> libc::c_int {
    0
}
fn pg_tzset(_tzname: *const libc::c_char) -> *mut pg_tz {
    std::ptr::null_mut()
}
static mut session_timezone: *mut pg_tz = 0 as *mut _;

fn strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: libc::c_ulong) -> libc::c_ulong {
    unsafe {
        let mut d: *mut libc::c_char = dst;
        let mut s: *const libc::c_char = src;
        let mut n: libc::c_ulong = siz;

        /* Copy as many bytes as will fit */
        if n != 0 {
            loop {
                n -= 1;
                if n == 0 {
                    break;
                }
                *d = *s;
                s = s.offset(1);
                d = d.offset(1);
                if *s == 0 {
                    break;
                }
            }
        }

        /* Not enough room in dst, add NUL and traverse rest of src */
        if n == 0 {
            if siz != 0 {
                *d = 0; /* NUL-terminate dst */
            }
            while *s != 0 {
                s = s.offset(1);
            }
        }

        return (s as isize - src as isize - 1) as u64; /* count does not include NUL */
    }
}
fn strtoint(
    str: *const libc::c_char,
    endptr: *mut *mut libc::c_char,
    base: libc::c_int,
) -> libc::c_int {
    unsafe {
        let val = libc::strtol(str, endptr, base);
        return val.try_into().unwrap();
    }
}
fn time_overflows(hour: libc::c_int, min: libc::c_int, sec: libc::c_int, fsec: fsec_t) -> bool {
    /* Range-check the fields individually. */
    if hour < 0
        || hour > HOURS_PER_DAY
        || min < 0
        || min >= MINS_PER_HOUR
        || sec < 0
        || sec > SECS_PER_MINUTE
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
    tzp: *mut libc::c_int,
    tm: *mut pg_tm,
    fsec: *mut fsec_t,
    tzn: *mut *const libc::c_char,
    mut attimezone: *mut pg_tz,
) -> libc::c_int {
    unsafe {
        let mut date: Timestamp = 0;
        let mut time: Timestamp;
        let utime: pg_time_t;

        /* Use session timezone if caller asks for default */
        if attimezone.is_null() {
            attimezone = session_timezone;
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
            &mut (*tm).tm_year,
            &mut (*tm).tm_mon,
            &mut (*tm).tm_mday,
        );
        dt2time(
            time,
            &mut (*tm).tm_hour,
            &mut (*tm).tm_min,
            &mut (*tm).tm_sec,
            fsec,
        );

        /* Done if no TZ conversion wanted */
        if tzp.is_null() {
            (*tm).tm_isdst = None;
            (*tm).tm_gmtoff = 0;
            (*tm).tm_zone = std::ptr::null_mut();
            if tzn != std::ptr::null_mut() {
                *tzn = std::ptr::null_mut();
            }
            return 0;
        }

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
        utime = dt;
        if utime == dt {
            let tx = pg_localtime(&utime, attimezone);

            (*tm).tm_year = (*tx).tm_year + 1900;
            (*tm).tm_mon = (*tx).tm_mon + 1;
            (*tm).tm_mday = (*tx).tm_mday;
            (*tm).tm_hour = (*tx).tm_hour;
            (*tm).tm_min = (*tx).tm_min;
            (*tm).tm_sec = (*tx).tm_sec;
            (*tm).tm_isdst = (*tx).tm_isdst;
            (*tm).tm_gmtoff = (*tx).tm_gmtoff;
            (*tm).tm_zone = (*tx).tm_zone;
            *tzp = (-(*tm).tm_gmtoff).try_into().unwrap();
            if !tzn.is_null() {
                *tzn = (*tm).tm_zone;
            }
        } else {
            /*
             * When out of range of pg_time_t, treat as GMT
             */
            *tzp = 0;
            /* Mark this as *no* time zone available */
            (*tm).tm_isdst = None;
            (*tm).tm_gmtoff = 0;
            (*tm).tm_zone = std::ptr::null_mut();
            if !tzn.is_null() {
                *tzn = std::ptr::null_mut();
            }
        }

        return 0;
    }
}

extern "C" {
    #![allow(improper_ctypes)]
    type AttrMissing;
    type PartitionDirectoryData;
    type RelationData;
    type ParseState;
    type JitInstrumentation;
    type JitContext;
    type dsa_area;
    type QueryEnvironment;
    type CopyMultiInsertBuffer;
    type FdwRoutine;
    type GlobalVisState;
    type SharedJitInstrumentation;
    type ExprEvalStep;
    type Tuplestorestate;
    type pg_tz;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn rint(_: libc::c_double) -> libc::c_double;
}

type Oid = libc::c_uint;
type size_t = libc::c_ulong;
type __time_t = libc::c_long;
type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
struct timespec {
    tv_sec: __time_t,
    tv_nsec: __syscall_slong_t,
}
type uintptr_t = libc::c_ulong;
type int16 = libc::c_short;
type int32 = libc::c_int;
type uint8 = libc::c_uchar;
type uint16 = libc::c_ushort;
type uint32 = libc::c_uint;
type bits8 = uint8;
type int64 = libc::c_long;
type uint64 = libc::c_ulong;
type Size = size_t;
type Index = libc::c_uint;
type TransactionId = uint32;
type CommandId = uint32;
#[derive(Copy, Clone)]
#[repr(C)]
struct varlena {
    vl_len_: [libc::c_char; 4],
    vl_dat: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
struct nameData {
    data: [libc::c_char; 64],
}
type NameData = nameData;
type C2RustUnnamed = libc::c_uint;
const _ISalnum: C2RustUnnamed = 8;
const _ISpunct: C2RustUnnamed = 4;
const _IScntrl: C2RustUnnamed = 2;
const _ISblank: C2RustUnnamed = 1;
const _ISgraph: C2RustUnnamed = 32768;
const _ISprint: C2RustUnnamed = 16384;
const _ISspace: C2RustUnnamed = 8192;
const _ISxdigit: C2RustUnnamed = 4096;
const _ISdigit: C2RustUnnamed = 2048;
const _ISalpha: C2RustUnnamed = 1024;
const _ISlower: C2RustUnnamed = 512;
const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
struct MemoryContextData {
    type_0: NodeTag,
    isReset: bool,
    allowInCritSection: bool,
    mem_allocated: Size,
    methods: *const MemoryContextMethods,
    parent: MemoryContext,
    firstchild: MemoryContext,
    prevchild: MemoryContext,
    nextchild: MemoryContext,
    name: *const libc::c_char,
    ident: *const libc::c_char,
    reset_cbs: *mut MemoryContextCallback,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct MemoryContextCallback {
    func: MemoryContextCallbackFunction,
    arg: *mut libc::c_void,
    next: *mut MemoryContextCallback,
}
type MemoryContextCallbackFunction = Option<unsafe fn(*mut libc::c_void) -> ()>;
type MemoryContext = *mut MemoryContextData;
#[derive(Copy, Clone)]
#[repr(C)]
struct MemoryContextMethods {
    alloc: Option<unsafe fn(MemoryContext, Size) -> *mut libc::c_void>,
    free_p: Option<unsafe fn(MemoryContext, *mut libc::c_void) -> ()>,
    realloc: Option<unsafe fn(MemoryContext, *mut libc::c_void, Size) -> *mut libc::c_void>,
    reset: Option<unsafe fn(MemoryContext) -> ()>,
    delete_context: Option<unsafe fn(MemoryContext) -> ()>,
    get_chunk_space: Option<unsafe fn(MemoryContext, *mut libc::c_void) -> Size>,
    is_empty: Option<unsafe fn(MemoryContext) -> bool>,
    stats: Option<
        unsafe fn(
            MemoryContext,
            MemoryStatsPrintFunc,
            *mut libc::c_void,
            *mut MemoryContextCounters,
            bool,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct MemoryContextCounters {
    nblocks: Size,
    freechunks: Size,
    totalspace: Size,
    freespace: Size,
}
type MemoryStatsPrintFunc =
    Option<unsafe fn(MemoryContext, *mut libc::c_void, *const libc::c_char, bool) -> ()>;
type NodeTag = libc::c_uint;
type Datum = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
struct NullableDatum {
    value: Datum,
    isnull: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct BlockIdData {
    bi_hi: uint16,
    bi_lo: uint16,
}
type OffsetNumber = uint16;
#[derive(Copy, Clone)]
#[repr(C, align(2))]
struct ItemPointerData(ItemPointerData_Inner);
#[derive(Copy, Clone)]
#[repr(C, packed)]
struct ItemPointerData_Inner {
    ip_blkid: BlockIdData,
    ip_posid: OffsetNumber,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct HeapTupleHeaderData {
    t_choice: C2RustUnnamed_0,
    t_ctid: ItemPointerData,
    t_infomask2: uint16,
    t_infomask: uint16,
    t_hoff: uint8,
    t_bits: [bits8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
union C2RustUnnamed_0 {
    t_heap: HeapTupleFields,
    t_datum: DatumTupleFields,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct DatumTupleFields {
    datum_len_: int32,
    datum_typmod: int32,
    datum_typeid: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct HeapTupleFields {
    t_xmin: TransactionId,
    t_xmax: TransactionId,
    t_field3: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
union C2RustUnnamed_1 {
    t_cid: CommandId,
    t_xvac: TransactionId,
}
type HeapTupleHeader = *mut HeapTupleHeaderData;
#[derive(Copy, Clone)]
#[repr(C)]
struct MinimalTupleData {
    t_len: uint32,
    mt_padding: [libc::c_char; 6],
    t_infomask2: uint16,
    t_infomask: uint16,
    t_hoff: uint8,
    t_bits: [bits8; 0],
}
type MinimalTuple = *mut MinimalTupleData;
#[derive(Copy, Clone)]
#[repr(C)]
struct HeapTupleData {
    t_len: uint32,
    t_self: ItemPointerData,
    t_tableOid: Oid,
    t_data: HeapTupleHeader,
}
type HeapTuple = *mut HeapTupleData;
type XLogRecPtr = uint64;
type AttrNumber = int16;
#[derive(Copy, Clone)]
#[repr(C)]
struct FormData_pg_attribute {
    attrelid: Oid,
    attname: NameData,
    atttypid: Oid,
    attstattarget: int32,
    attlen: int16,
    attnum: int16,
    attndims: int32,
    attcacheoff: int32,
    atttypmod: int32,
    attbyval: bool,
    attalign: libc::c_char,
    attstorage: libc::c_char,
    attcompression: libc::c_char,
    attnotnull: bool,
    atthasdef: bool,
    atthasmissing: bool,
    attidentity: libc::c_char,
    attgenerated: libc::c_char,
    attisdropped: bool,
    attislocal: bool,
    attinhcount: int32,
    attcollation: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct Node {
    type_0: NodeTag,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct Bitmapset {
    nwords: libc::c_int,
    words: [bitmapword; 0],
}
type bitmapword = uint64;
type Cost = libc::c_double;
type Cardinality = libc::c_double;
type CmdType = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
union ListCell {
    ptr_value: *mut libc::c_void,
    int_value: libc::c_int,
    oid_value: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct List {
    type_0: NodeTag,
    length: libc::c_int,
    max_length: libc::c_int,
    elements: *mut ListCell,
    initial_elements: [ListCell; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
struct AttrDefault {
    adnum: AttrNumber,
    adbin: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct ConstrCheck {
    ccname: *mut libc::c_char,
    ccbin: *mut libc::c_char,
    ccvalid: bool,
    ccnoinherit: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct TupleConstr {
    defval: *mut AttrDefault,
    check: *mut ConstrCheck,
    missing: *mut AttrMissing,
    num_defval: uint16,
    num_check: uint16,
    has_not_null: bool,
    has_generated_stored: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct TupleDescData {
    natts: libc::c_int,
    tdtypeid: Oid,
    tdtypmod: int32,
    tdrefcount: libc::c_int,
    constr: *mut TupleConstr,
    attrs: [FormData_pg_attribute; 0],
}
type TupleDesc = *mut TupleDescData;
type Timestamp = int64;
type TimestampTz = int64;
type TimeOffset = int64;
pub type fsec_t = int32;
pub type DateADT = int32;
#[derive(Copy, Clone)]
#[repr(C)]
struct Interval {
    time: TimeOffset,
    day: int32,
    month: int32,
}
type LockClauseStrength = libc::c_uint;
type LockWaitPolicy = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
struct Expr {
    type_0: NodeTag,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct Const {
    xpr: Expr,
    consttype: Oid,
    consttypmod: int32,
    constcollid: Oid,
    constlen: libc::c_int,
    constvalue: Datum,
    constisnull: bool,
    constbyval: bool,
    location: libc::c_int,
}
type ParamKind = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
struct Param {
    xpr: Expr,
    paramkind: ParamKind,
    paramid: libc::c_int,
    paramtype: Oid,
    paramtypmod: int32,
    paramcollid: Oid,
    location: libc::c_int,
}
type CoercionForm = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
struct FuncExpr {
    xpr: Expr,
    funcid: Oid,
    funcresulttype: Oid,
    funcretset: bool,
    funcvariadic: bool,
    funcformat: CoercionForm,
    funccollid: Oid,
    inputcollid: Oid,
    args: *mut List,
    location: libc::c_int,
}
type PartitionDirectory = *mut PartitionDirectoryData;
type Relation = *mut RelationData;
type RelationPtr = *mut Relation;
#[derive(Copy, Clone)]
#[repr(C)]
struct AttrMap {
    attnums: *mut AttrNumber,
    maplen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct TupleTableSlotOps {
    base_slot_size: size_t,
    init: Option<unsafe fn(*mut TupleTableSlot) -> ()>,
    release: Option<unsafe fn(*mut TupleTableSlot) -> ()>,
    clear: Option<unsafe fn(*mut TupleTableSlot) -> ()>,
    getsomeattrs: Option<unsafe fn(*mut TupleTableSlot, libc::c_int) -> ()>,
    getsysattr: Option<unsafe fn(*mut TupleTableSlot, libc::c_int, *mut bool) -> Datum>,
    materialize: Option<unsafe fn(*mut TupleTableSlot) -> ()>,
    copyslot: Option<unsafe fn(*mut TupleTableSlot, *mut TupleTableSlot) -> ()>,
    get_heap_tuple: Option<unsafe fn(*mut TupleTableSlot) -> HeapTuple>,
    get_minimal_tuple: Option<unsafe fn(*mut TupleTableSlot) -> MinimalTuple>,
    copy_heap_tuple: Option<unsafe fn(*mut TupleTableSlot) -> HeapTuple>,
    copy_minimal_tuple: Option<unsafe fn(*mut TupleTableSlot) -> MinimalTuple>,
}
#[repr(C)]
struct TupleTableSlot<'a> {
    type_0: NodeTag,
    tts_flags: uint16,
    tts_nvalid: AttrNumber,
    tts_ops: *const TupleTableSlotOps,
    tts_tupleDescriptor: TupleDesc,
    tts_values: *mut Datum,
    tts_isnull: &'a mut bool,
    tts_mcxt: MemoryContext,
    tts_tid: ItemPointerData,
    tts_tableOid: Oid,
}
#[repr(C)]
struct TupleConversionMap<'a> {
    indesc: TupleDesc,
    outdesc: TupleDesc,
    attrMap: *mut AttrMap,
    invalues: *mut Datum,
    inisnull: &'a mut bool,
    outvalues: *mut Datum,
    outisnull: &'a mut bool,
}
type instr_time = timespec;
#[derive(Copy, Clone)]
#[repr(C)]
struct BufferUsage {
    shared_blks_hit: int64,
    shared_blks_read: int64,
    shared_blks_dirtied: int64,
    shared_blks_written: int64,
    local_blks_hit: int64,
    local_blks_read: int64,
    local_blks_dirtied: int64,
    local_blks_written: int64,
    temp_blks_read: int64,
    temp_blks_written: int64,
    blk_read_time: instr_time,
    blk_write_time: instr_time,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct WalUsage {
    wal_records: int64,
    wal_fpi: int64,
    wal_bytes: uint64,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct Instrumentation {
    need_timer: bool,
    need_bufusage: bool,
    need_walusage: bool,
    async_mode: bool,
    running: bool,
    starttime: instr_time,
    counter: instr_time,
    firsttuple: libc::c_double,
    tuplecount: libc::c_double,
    bufusage_start: BufferUsage,
    walusage_start: WalUsage,
    startup: libc::c_double,
    total: libc::c_double,
    ntuples: libc::c_double,
    ntuples2: libc::c_double,
    nloops: libc::c_double,
    nfiltered1: libc::c_double,
    nfiltered2: libc::c_double,
    bufusage: BufferUsage,
    walusage: WalUsage,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct WorkerInstrumentation {
    num_workers: libc::c_int,
    instrument: [Instrumentation; 0],
}
type fmNodePtr = *mut Node;
#[derive(Copy, Clone)]
#[repr(C)]
struct FunctionCallInfoBaseData {
    flinfo: *mut FmgrInfo,
    context: fmNodePtr,
    resultinfo: fmNodePtr,
    fncollation: Oid,
    isnull: bool,
    nargs: libc::c_short,
    args: [NullableDatum; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
struct FmgrInfo {
    fn_addr: PGFunction,
    fn_oid: Oid,
    fn_nargs: libc::c_short,
    fn_strict: bool,
    fn_retset: bool,
    fn_stats: libc::c_uchar,
    fn_extra: *mut libc::c_void,
    fn_mcxt: MemoryContext,
    fn_expr: fmNodePtr,
}
type PGFunction = Option<unsafe fn(FunctionCallInfo) -> Datum>;
type FunctionCallInfo = *mut FunctionCallInfoBaseData;
#[derive(Copy, Clone)]
#[repr(C)]
struct pairingheap_node {
    first_child: *mut pairingheap_node,
    next_sibling: *mut pairingheap_node,
    prev_or_parent: *mut pairingheap_node,
}
#[repr(C)]
struct ExprState<'a> {
    type_0: NodeTag,
    flags: uint8,
    resnull: bool,
    resvalue: Datum,
    resultslot: *mut TupleTableSlot<'a>,
    steps: *mut ExprEvalStep,
    evalfunc: ExprStateEvalFunc,
    expr: *mut Expr,
    evalfunc_private: *mut libc::c_void,
    steps_len: libc::c_int,
    steps_alloc: libc::c_int,
    parent: *mut PlanState<'a>,
    ext_params: ParamListInfo,
    innermost_caseval: *mut Datum,
    innermost_casenull: &'a mut bool,
    innermost_domainval: *mut Datum,
    innermost_domainnull: &'a mut bool,
}
type ParamListInfo = *mut ParamListInfoData;
#[derive(Copy, Clone)]
#[repr(C)]
struct ParamListInfoData {
    paramFetch: ParamFetchHook,
    paramFetchArg: *mut libc::c_void,
    paramCompile: ParamCompileHook,
    paramCompileArg: *mut libc::c_void,
    parserSetup: ParserSetupHook,
    parserSetupArg: *mut libc::c_void,
    paramValuesStr: *mut libc::c_char,
    numParams: libc::c_int,
    params: [ParamExternData; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
struct ParamExternData {
    value: Datum,
    isnull: bool,
    pflags: uint16,
    ptype: Oid,
}
type ParserSetupHook = Option<unsafe fn(*mut ParseState, *mut libc::c_void) -> ()>;
type ParamCompileHook =
    Option<unsafe fn(ParamListInfo, *mut Param, *mut ExprState, *mut Datum, &mut bool) -> ()>;
type ParamFetchHook = Option<
    unsafe fn(ParamListInfo, libc::c_int, bool, *mut ParamExternData) -> *mut ParamExternData,
>;
#[repr(C)]
struct PlanState<'a> {
    type_0: NodeTag,
    plan: *mut Plan,
    state: *mut EState<'a>,
    ExecProcNode: ExecProcNodeMtd,
    ExecProcNodeReal: ExecProcNodeMtd,
    instrument: *mut Instrumentation,
    worker_instrument: *mut WorkerInstrumentation,
    worker_jit_instrument: *mut SharedJitInstrumentation,
    qual: *mut ExprState<'a>,
    lefttree: *mut PlanState<'a>,
    righttree: *mut PlanState<'a>,
    initPlan: *mut List,
    subPlan: *mut List,
    chgParam: *mut Bitmapset,
    ps_ResultTupleDesc: TupleDesc,
    ps_ResultTupleSlot: *mut TupleTableSlot<'a>,
    ps_ExprContext: *mut ExprContext<'a>,
    ps_ProjInfo: *mut ProjectionInfo<'a>,
    async_capable: bool,
    scandesc: TupleDesc,
    scanops: *const TupleTableSlotOps,
    outerops: *const TupleTableSlotOps,
    innerops: *const TupleTableSlotOps,
    resultops: *const TupleTableSlotOps,
    scanopsfixed: bool,
    outeropsfixed: bool,
    inneropsfixed: bool,
    resultopsfixed: bool,
    scanopsset: bool,
    outeropsset: bool,
    inneropsset: bool,
    resultopsset: bool,
}
#[repr(C)]
struct ProjectionInfo<'a> {
    type_0: NodeTag,
    pi_state: ExprState<'a>,
    pi_exprContext: *mut ExprContext<'a>,
}
#[repr(C)]
struct ExprContext<'a> {
    type_0: NodeTag,
    ecxt_scantuple: *mut TupleTableSlot<'a>,
    ecxt_innertuple: *mut TupleTableSlot<'a>,
    ecxt_outertuple: *mut TupleTableSlot<'a>,
    ecxt_per_query_memory: MemoryContext,
    ecxt_per_tuple_memory: MemoryContext,
    ecxt_param_exec_vals: *mut ParamExecData,
    ecxt_param_list_info: ParamListInfo,
    ecxt_aggvalues: *mut Datum,
    ecxt_aggnulls: &'a mut bool,
    caseValue_datum: Datum,
    caseValue_isNull: bool,
    domainValue_datum: Datum,
    domainValue_isNull: bool,
    ecxt_estate: *mut EState<'a>,
    ecxt_callbacks: *mut ExprContext_CB,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct ExprContext_CB {
    next: *mut ExprContext_CB,
    function: ExprContextCallbackFunction,
    arg: Datum,
}
type ExprContextCallbackFunction = Option<unsafe fn(Datum) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
struct EState<'a> {
    type_0: NodeTag,
    es_direction: ScanDirection,
    es_snapshot: Snapshot,
    es_crosscheck_snapshot: Snapshot,
    es_range_table: *mut List,
    es_range_table_size: Index,
    es_relations: *mut Relation,
    es_rowmarks: *mut *mut ExecRowMark,
    es_plannedstmt: *mut PlannedStmt,
    es_sourceText: *const libc::c_char,
    es_junkFilter: *mut JunkFilter<'a>,
    es_output_cid: CommandId,
    es_result_relations: *mut *mut ResultRelInfo<'a>,
    es_opened_result_relations: *mut List,
    es_partition_directory: PartitionDirectory,
    es_tuple_routing_result_relations: *mut List,
    es_trig_target_relations: *mut List,
    es_param_list_info: ParamListInfo,
    es_param_exec_vals: *mut ParamExecData,
    es_queryEnv: *mut QueryEnvironment,
    es_query_cxt: MemoryContext,
    es_tupleTable: *mut List,
    es_processed: uint64,
    es_top_eflags: libc::c_int,
    es_instrument: libc::c_int,
    es_finished: bool,
    es_exprcontexts: *mut List,
    es_subplanstates: *mut List,
    es_auxmodifytables: *mut List,
    es_per_tuple_exprcontext: *mut ExprContext<'a>,
    es_epq_active: *mut EPQState<'a>,
    es_use_parallel_mode: bool,
    es_query_dsa: *mut dsa_area,
    es_jit_flags: libc::c_int,
    es_jit: *mut JitContext,
    es_jit_worker_instr: *mut JitInstrumentation,
}
#[repr(C)]
struct EPQState<'a> {
    parentestate: *mut EState<'a>,
    epqParam: libc::c_int,
    tuple_table: *mut List,
    relsubs_slot: *mut *mut TupleTableSlot<'a>,
    plan: *mut Plan,
    arowMarks: *mut List,
    origslot: *mut TupleTableSlot<'a>,
    recheckestate: *mut EState<'a>,
    relsubs_rowmark: *mut *mut ExecAuxRowMark,
    relsubs_done: &'a mut bool,
    recheckplanstate: *mut PlanState<'a>,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct ExecAuxRowMark {
    rowmark: *mut ExecRowMark,
    ctidAttNo: AttrNumber,
    toidAttNo: AttrNumber,
    wholeAttNo: AttrNumber,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct ExecRowMark {
    relation: Relation,
    relid: Oid,
    rti: Index,
    prti: Index,
    rowmarkId: Index,
    markType: RowMarkType,
    strength: LockClauseStrength,
    waitPolicy: LockWaitPolicy,
    ermActive: bool,
    curCtid: ItemPointerData,
    ermExtra: *mut libc::c_void,
}
type RowMarkType = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
struct Plan {
    type_0: NodeTag,
    startup_cost: Cost,
    total_cost: Cost,
    plan_rows: Cardinality,
    plan_width: libc::c_int,
    parallel_aware: bool,
    parallel_safe: bool,
    async_capable: bool,
    plan_node_id: libc::c_int,
    targetlist: *mut List,
    qual: *mut List,
    lefttree: *mut Plan,
    righttree: *mut Plan,
    initPlan: *mut List,
    extParam: *mut Bitmapset,
    allParam: *mut Bitmapset,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct ParamExecData {
    execPlan: *mut libc::c_void,
    value: Datum,
    isnull: bool,
}
#[repr(C)]
struct ResultRelInfo<'a> {
    type_0: NodeTag,
    ri_RangeTableIndex: Index,
    ri_RelationDesc: Relation,
    ri_NumIndices: libc::c_int,
    ri_IndexRelationDescs: RelationPtr,
    ri_IndexRelationInfo: *mut *mut IndexInfo<'a>,
    ri_RowIdAttNo: AttrNumber,
    ri_projectNew: *mut ProjectionInfo<'a>,
    ri_newTupleSlot: *mut TupleTableSlot<'a>,
    ri_oldTupleSlot: *mut TupleTableSlot<'a>,
    ri_projectNewInfoValid: bool,
    ri_TrigDesc: *mut TriggerDesc,
    ri_TrigFunctions: *mut FmgrInfo,
    ri_TrigWhenExprs: *mut *mut ExprState<'a>,
    ri_TrigInstrument: *mut Instrumentation,
    ri_ReturningSlot: *mut TupleTableSlot<'a>,
    ri_TrigOldSlot: *mut TupleTableSlot<'a>,
    ri_TrigNewSlot: *mut TupleTableSlot<'a>,
    ri_FdwRoutine: *mut FdwRoutine,
    ri_FdwState: *mut libc::c_void,
    ri_usesFdwDirectModify: bool,
    ri_NumSlots: libc::c_int,
    ri_NumSlotsInitialized: libc::c_int,
    ri_BatchSize: libc::c_int,
    ri_Slots: *mut *mut TupleTableSlot<'a>,
    ri_PlanSlots: *mut *mut TupleTableSlot<'a>,
    ri_WithCheckOptions: *mut List,
    ri_WithCheckOptionExprs: *mut List,
    ri_ConstraintExprs: *mut *mut ExprState<'a>,
    ri_GeneratedExprs: *mut *mut ExprState<'a>,
    ri_NumGeneratedNeeded: libc::c_int,
    ri_returningList: *mut List,
    ri_projectReturning: *mut ProjectionInfo<'a>,
    ri_onConflictArbiterIndexes: *mut List,
    ri_onConflict: *mut OnConflictSetState<'a>,
    ri_PartitionCheckExpr: *mut ExprState<'a>,
    ri_RootResultRelInfo: *mut ResultRelInfo<'a>,
    ri_RootToPartitionMap: *mut TupleConversionMap<'a>,
    ri_PartitionTupleSlot: *mut TupleTableSlot<'a>,
    ri_ChildToRootMap: *mut TupleConversionMap<'a>,
    ri_ChildToRootMapValid: bool,
    ri_CopyMultiInsertBuffer: *mut CopyMultiInsertBuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct OnConflictSetState<'a> {
    type_0: NodeTag,
    oc_Existing: *mut TupleTableSlot<'a>,
    oc_ProjSlot: *mut TupleTableSlot<'a>,
    oc_ProjInfo: *mut ProjectionInfo<'a>,
    oc_WhereClause: *mut ExprState<'a>,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct TriggerDesc {
    triggers: *mut Trigger,
    numtriggers: libc::c_int,
    trig_insert_before_row: bool,
    trig_insert_after_row: bool,
    trig_insert_instead_row: bool,
    trig_insert_before_statement: bool,
    trig_insert_after_statement: bool,
    trig_update_before_row: bool,
    trig_update_after_row: bool,
    trig_update_instead_row: bool,
    trig_update_before_statement: bool,
    trig_update_after_statement: bool,
    trig_delete_before_row: bool,
    trig_delete_after_row: bool,
    trig_delete_instead_row: bool,
    trig_delete_before_statement: bool,
    trig_delete_after_statement: bool,
    trig_truncate_before_statement: bool,
    trig_truncate_after_statement: bool,
    trig_insert_new_table: bool,
    trig_update_old_table: bool,
    trig_update_new_table: bool,
    trig_delete_old_table: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct Trigger {
    tgoid: Oid,
    tgname: *mut libc::c_char,
    tgfoid: Oid,
    tgtype: int16,
    tgenabled: libc::c_char,
    tgisinternal: bool,
    tgisclone: bool,
    tgconstrrelid: Oid,
    tgconstrindid: Oid,
    tgconstraint: Oid,
    tgdeferrable: bool,
    tginitdeferred: bool,
    tgnargs: int16,
    tgnattr: int16,
    tgattr: *mut int16,
    tgargs: *mut *mut libc::c_char,
    tgqual: *mut libc::c_char,
    tgoldtable: *mut libc::c_char,
    tgnewtable: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct IndexInfo<'a> {
    type_0: NodeTag,
    ii_NumIndexAttrs: libc::c_int,
    ii_NumIndexKeyAttrs: libc::c_int,
    ii_IndexAttrNumbers: [AttrNumber; 32],
    ii_Expressions: *mut List,
    ii_ExpressionsState: *mut List,
    ii_Predicate: *mut List,
    ii_PredicateState: *mut ExprState<'a>,
    ii_ExclusionOps: *mut Oid,
    ii_ExclusionProcs: *mut Oid,
    ii_ExclusionStrats: *mut uint16,
    ii_UniqueOps: *mut Oid,
    ii_UniqueProcs: *mut Oid,
    ii_UniqueStrats: *mut uint16,
    ii_OpclassOptions: *mut Datum,
    ii_Unique: bool,
    ii_NullsNotDistinct: bool,
    ii_ReadyForInserts: bool,
    ii_CheckedUnchanged: bool,
    ii_IndexUnchanged: bool,
    ii_Concurrent: bool,
    ii_BrokenHotChain: bool,
    ii_ParallelWorkers: libc::c_int,
    ii_Am: Oid,
    ii_AmCache: *mut libc::c_void,
    ii_Context: MemoryContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct JunkFilter<'a> {
    type_0: NodeTag,
    jf_targetList: *mut List,
    jf_cleanTupType: TupleDesc,
    jf_cleanMap: *mut AttrNumber,
    jf_resultSlot: *mut TupleTableSlot<'a>,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct PlannedStmt {
    type_0: NodeTag,
    commandType: CmdType,
    queryId: uint64,
    hasReturning: bool,
    hasModifyingCTE: bool,
    canSetTag: bool,
    transientPlan: bool,
    dependsOnRole: bool,
    parallelModeNeeded: bool,
    jitFlags: libc::c_int,
    planTree: *mut Plan,
    rtable: *mut List,
    resultRelations: *mut List,
    appendRelations: *mut List,
    subplans: *mut List,
    rewindPlanIDs: *mut Bitmapset,
    rowMarks: *mut List,
    relationOids: *mut List,
    invalItems: *mut List,
    paramExecTypes: *mut List,
    utilityStmt: *mut Node,
    stmt_location: libc::c_int,
    stmt_len: libc::c_int,
}
type Snapshot = *mut SnapshotData;
#[derive(Copy, Clone)]
#[repr(C)]
struct SnapshotData {
    snapshot_type: SnapshotType,
    xmin: TransactionId,
    xmax: TransactionId,
    xip: *mut TransactionId,
    xcnt: uint32,
    subxip: *mut TransactionId,
    subxcnt: int32,
    suboverflowed: bool,
    takenDuringRecovery: bool,
    copied: bool,
    curcid: CommandId,
    speculativeToken: uint32,
    vistest: *mut GlobalVisState,
    active_count: uint32,
    regd_count: uint32,
    ph_node: pairingheap_node,
    whenTaken: TimestampTz,
    lsn: XLogRecPtr,
    snapXactCompletionCount: uint64,
}
type SnapshotType = libc::c_uint;
type ScanDirection = libc::c_int;
type ExecProcNodeMtd = Option<unsafe fn(*mut PlanState) -> *mut TupleTableSlot>;
type ExprStateEvalFunc = Option<unsafe fn(*mut ExprState, *mut ExprContext, &mut bool) -> Datum>;
type ExprDoneCond = libc::c_uint;
type SetFunctionReturnMode = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
struct ReturnSetInfo<'a> {
    type_0: NodeTag,
    econtext: *mut ExprContext<'a>,
    expectedDesc: TupleDesc,
    allowedModes: libc::c_int,
    returnMode: SetFunctionReturnMode,
    isDone: ExprDoneCond,
    setResult: *mut Tuplestorestate,
    setDesc: TupleDesc,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct AttInMetadata {
    tupdesc: TupleDesc,
    attinfuncs: *mut FmgrInfo,
    attioparams: *mut Oid,
    atttypmods: *mut int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct FuncCallContext {
    call_cntr: uint64,
    max_calls: uint64,
    user_fctx: *mut libc::c_void,
    attinmeta: *mut AttInMetadata,
    multi_call_memory_ctx: MemoryContext,
    tuple_desc: TupleDesc,
}
type pg_time_t = int64;
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct pg_tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: Option<bool>,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct datetkn {
    token: [libc::c_char; 11],
    type_0: RealFieldType,
    value: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct TimeZoneAbbrevTable {
    tblsize: Size,
    numabbrevs: libc::c_int,
    abbrevs: [datetkn; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
struct DynamicZoneAbbrev {
    tz: *mut pg_tz,
    zone: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
struct tzEntry {
    abbrev: *mut libc::c_char,
    zone: *mut libc::c_char,
    offset: libc::c_int,
    is_dst: bool,
    lineno: libc::c_int,
    filename: *const libc::c_char,
}

static mut day_tab: [[libc::c_int; 13]; 2] = [
    [
        31 as libc::c_int,
        28 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        31 as libc::c_int,
        29 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        0 as libc::c_int,
    ],
];

const EPOCH: &'static str = "epoch";
const EARLY: &'static str = "-infinity";
const LATE: &'static str = "infinity";
const NOW: &'static str = "now";
const TODAY: &'static str = "today";
const TOMORROW: &'static str = "tomorrow";
const YESTERDAY: &'static str = "yesterday";
const DA_D: &'static str = "ad";
const DB_C: &'static str = "bc";

// Fundamental time field definitions for parsing.
//
// Meridian:  am, pm, or 24-hour style.
// Millennium: ad, bc
const AM: i32 = 0;
const PM: i32 = 1;

const AD: i32 = 0;
const BC: i32 = 1;

/// holds date/time keywords.
///
/// Note that this table must be strictly alphabetically ordered to allow an
/// O(ln(N)) search algorithm to be used.
///
/// The static table contains no TZ, DTZ, or DYNTZ entries; rather those
/// are loaded from configuration files and stored in zoneabbrevtbl, whose
/// abbrevs[] field has the same format as the static DATE_TOKEN_TABLE.
static DATE_TOKEN_TABLE: &'static [(&'static str, RealFieldType, i32)] = &[
    /* token, type, value */
    (EARLY, RealFieldType::Reserved, TokenFieldType::Early as i32), /* "-infinity" reserved for "early time" */
    (DA_D, RealFieldType::Adbc, AD),                                /* "ad" for years > 0 */
    (
        "allballs",
        RealFieldType::Reserved,
        TokenFieldType::Zulu as i32,
    ), /* 00:00:00 */
    ("am", RealFieldType::AmPm, AM),
    ("apr", RealFieldType::Month, 4),
    ("april", RealFieldType::Month, 4),
    ("at", RealFieldType::IgnoreDtf, 0), /* "at" (throwaway) */
    ("aug", RealFieldType::Month, 8),
    ("august", RealFieldType::Month, 8),
    (DB_C, RealFieldType::Adbc, BC), /* "bc" for years <= 0 */
    ("d", RealFieldType::Units, TokenFieldType::Day as i32), /* "day of month" for ISO input */
    ("dec", RealFieldType::Month, 12),
    ("december", RealFieldType::Month, 12),
    ("dow", RealFieldType::Units, TokenFieldType::Dow as i32), /* day of week */
    ("doy", RealFieldType::Units, TokenFieldType::Doy as i32), /* day of year */
    ("dst", RealFieldType::DtzMod, SECS_PER_HOUR),
    (EPOCH, RealFieldType::Reserved, TokenFieldType::Epoch as i32), /* "epoch" reserved for system epoch time */
    ("feb", RealFieldType::Month, 2),
    ("february", RealFieldType::Month, 2),
    ("fri", RealFieldType::Dow, 5),
    ("friday", RealFieldType::Dow, 5),
    ("h", RealFieldType::Units, TokenFieldType::Hour as i32), /* "hour" */
    (LATE, RealFieldType::Reserved, TokenFieldType::Late as i32), /* "infinity" reserved for "late time" */
    (
        "isodow",
        RealFieldType::Units,
        TokenFieldType::IsoDow as i32,
    ), /* ISO day of week, Sunday == 7 */
    (
        "isoyear",
        RealFieldType::Units,
        TokenFieldType::IsoYear as i32,
    ), /* year in terms of the ISO week date */
    ("j", RealFieldType::Units, TokenFieldType::Julian as i32),
    ("jan", RealFieldType::Month, 1),
    ("january", RealFieldType::Month, 1),
    ("jd", RealFieldType::Units, TokenFieldType::Julian as i32),
    ("jul", RealFieldType::Month, 7),
    (
        "julian",
        RealFieldType::Units,
        TokenFieldType::Julian as i32,
    ),
    ("july", RealFieldType::Month, 7),
    ("jun", RealFieldType::Month, 6),
    ("june", RealFieldType::Month, 6),
    ("m", RealFieldType::Units, TokenFieldType::Month as i32), /* "month" for ISO input */
    ("mar", RealFieldType::Month, 3),
    ("march", RealFieldType::Month, 3),
    ("may", RealFieldType::Month, 5),
    ("mm", RealFieldType::Units, TokenFieldType::Minute as i32), /* "minute" for ISO input */
    ("mon", RealFieldType::Dow, 1),
    ("monday", RealFieldType::Dow, 1),
    ("nov", RealFieldType::Month, 11),
    ("november", RealFieldType::Month, 11),
    (NOW, RealFieldType::Reserved, TokenFieldType::Now as i32), /* current transaction time */
    ("oct", RealFieldType::Month, 10),
    ("october", RealFieldType::Month, 10),
    ("on", RealFieldType::IgnoreDtf, 0), /* "on" (throwaway) */
    ("pm", RealFieldType::AmPm, PM),
    ("s", RealFieldType::Units, TokenFieldType::Second as i32), /* "seconds" for ISO input */
    ("sat", RealFieldType::Dow, 6),
    ("saturday", RealFieldType::Dow, 6),
    ("sep", RealFieldType::Month, 9),
    ("sept", RealFieldType::Month, 9),
    ("september", RealFieldType::Month, 9),
    ("sun", RealFieldType::Dow, 0),
    ("sunday", RealFieldType::Dow, 0),
    ("t", RealFieldType::IsoTime, TokenFieldType::Time as i32), /* Filler for ISO time fields */
    ("thu", RealFieldType::Dow, 4),
    ("thur", RealFieldType::Dow, 4),
    ("thurs", RealFieldType::Dow, 4),
    ("thursday", RealFieldType::Dow, 4),
    (TODAY, RealFieldType::Reserved, TokenFieldType::Today as i32), /* midnight */
    (
        TOMORROW,
        RealFieldType::Reserved,
        TokenFieldType::Tomorrow as i32,
    ), /* tomorrow midnight */
    ("tue", RealFieldType::Dow, 2),
    ("tues", RealFieldType::Dow, 2),
    ("tuesday", RealFieldType::Dow, 2),
    ("wed", RealFieldType::Dow, 3),
    ("wednesday", RealFieldType::Dow, 3),
    ("weds", RealFieldType::Dow, 3),
    ("y", RealFieldType::Units, TokenFieldType::Year as i32), /* "year" for ISO input */
    (
        YESTERDAY,
        RealFieldType::Reserved,
        TokenFieldType::Yesterday as i32,
    ), /* yesterday midnight */
];

static mut datetktbl: [datetkn; 71] = unsafe {
    [
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"-infinity\0\0",
                ),
                type_0: RealFieldType::Reserved,
                value: 9 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"ad\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Adbc,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"allballs\0\0\0",
                ),
                type_0: RealFieldType::Reserved,
                value: 16 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"am\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::AmPm,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"apr\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 4 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"april\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 4 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"at\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::IgnoreDtf,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"aug\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 8 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"august\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 8 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"bc\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Adbc,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"d\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 21 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"dec\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 12 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"december\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 12 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"dow\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 32 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"doy\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 33 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"dst\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::DtzMod,
                value: 3600 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"epoch\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Reserved,
                value: 11 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"feb\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 2 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"february\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 2 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"fri\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 5 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"friday\0\0\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 5 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"h\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 20 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"infinity\0\0\0",
                ),
                type_0: RealFieldType::Reserved,
                value: 10 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"isodow\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 37 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"isoyear\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 36 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"j\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 31 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"jan\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"january\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"jd\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 31 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"jul\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 7 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"julian\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 31 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"july\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 7 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"jun\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 6 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"june\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 6 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"m\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 23 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mar\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 3 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"march\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 3 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"may\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 5 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mm\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 19 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mon\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"monday\0\0\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"nov\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 11 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"november\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 11 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"now\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Reserved,
                value: 12 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"oct\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 10 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"october\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 10 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"on\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::IgnoreDtf,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"pm\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::AmPm,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"s\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 18 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"sat\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 6 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"saturday\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 6 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"sep\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 9 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"sept\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 9 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"september\0\0",
                ),
                type_0: RealFieldType::Month,
                value: 9 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"sun\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"sunday\0\0\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"t\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::IsoTime,
                value: 3 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"thu\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 4 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"thur\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 4 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"thurs\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 4 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"thursday\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 4 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"today\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Reserved,
                value: 14 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"tomorrow\0\0\0",
                ),
                type_0: RealFieldType::Reserved,
                value: 15 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"tue\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 2 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"tues\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 2 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"tuesday\0\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 2 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"wed\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 3 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"wednesday\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 3 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"weds\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Dow,
                value: 3 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"y\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 25 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"yesterday\0\0",
                ),
                type_0: RealFieldType::Reserved,
                value: 13 as libc::c_int,
            };
            init
        },
    ]
};
static mut szdatetktbl: libc::c_int = 0;
static mut szdeltatktbl: libc::c_int = 0;
static mut zoneabbrevtbl: *mut TimeZoneAbbrevTable =
    0 as *const TimeZoneAbbrevTable as *mut TimeZoneAbbrevTable;
static mut datecache: [*const datetkn; 25] = [
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
];
static mut abbrevcache: [*const datetkn; 25] = [
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
    0 as *const datetkn,
];

unsafe fn date2j(mut y: libc::c_int, mut m: libc::c_int, d: libc::c_int) -> libc::c_int {
    if m > 2 as libc::c_int {
        m += 1 as libc::c_int;
        y += 4800 as libc::c_int;
    } else {
        m += 13 as libc::c_int;
        y += 4799 as libc::c_int;
    }
    let century = y / 100 as libc::c_int;
    let mut julian = y * 365 as libc::c_int - 32167 as libc::c_int;
    julian += y / 4 as libc::c_int - century + century / 4 as libc::c_int;
    julian += 7834 as libc::c_int * m / 256 as libc::c_int + d;
    return julian;
}

unsafe fn j2date(
    jd: libc::c_int,
    year: *mut libc::c_int,
    month: *mut libc::c_int,
    day: *mut libc::c_int,
) {
    let mut julian = jd as libc::c_uint;
    julian = julian.wrapping_add(32044 as libc::c_int as libc::c_uint);
    let mut quad = julian.wrapping_div(146097 as libc::c_int as libc::c_uint);
    let extra = julian
        .wrapping_sub(quad.wrapping_mul(146097 as libc::c_int as libc::c_uint))
        .wrapping_mul(4 as libc::c_int as libc::c_uint)
        .wrapping_add(3 as libc::c_int as libc::c_uint);
    julian = julian.wrapping_add(
        (60 as libc::c_int as libc::c_uint)
            .wrapping_add(quad.wrapping_mul(3 as libc::c_int as libc::c_uint))
            .wrapping_add(extra.wrapping_div(146097 as libc::c_int as libc::c_uint)),
    );
    quad = julian.wrapping_div(1461 as libc::c_int as libc::c_uint);
    julian = julian.wrapping_sub(quad.wrapping_mul(1461 as libc::c_int as libc::c_uint));
    let mut y = julian
        .wrapping_mul(4 as libc::c_int as libc::c_uint)
        .wrapping_div(1461 as libc::c_int as libc::c_uint) as libc::c_int;
    julian = (if y != 0 as libc::c_int {
        julian
            .wrapping_add(305 as libc::c_int as libc::c_uint)
            .wrapping_rem(365 as libc::c_int as libc::c_uint)
    } else {
        julian
            .wrapping_add(306 as libc::c_int as libc::c_uint)
            .wrapping_rem(366 as libc::c_int as libc::c_uint)
    })
    .wrapping_add(123 as libc::c_int as libc::c_uint);
    y = (y as libc::c_uint).wrapping_add(quad.wrapping_mul(4 as libc::c_int as libc::c_uint))
        as libc::c_int as libc::c_int;
    *year = y - 4800 as libc::c_int;
    quad = julian
        .wrapping_mul(2141 as libc::c_int as libc::c_uint)
        .wrapping_div(65536 as libc::c_int as libc::c_uint);
    *day = julian.wrapping_sub(
        (7834 as libc::c_int as libc::c_uint)
            .wrapping_mul(quad)
            .wrapping_div(256 as libc::c_int as libc::c_uint),
    ) as libc::c_int;
    *month = quad
        .wrapping_add(10 as libc::c_int as libc::c_uint)
        .wrapping_rem(12 as libc::c_int as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
}

unsafe fn GetCurrentDateTime(tm: *mut pg_tm) {
    let mut fsec: fsec_t = 0;
    GetCurrentTimeUsec(tm, &mut fsec, 0 as *mut libc::c_int);
}

unsafe fn GetCurrentTimeUsec(tm: *mut pg_tm, fsec: *mut fsec_t, tzp: *mut libc::c_int) {
    let cur_ts: TimestampTz = GetCurrentTransactionStartTimestamp();
    static mut cache_ts: TimestampTz = 0 as libc::c_int as TimestampTz;
    static mut cache_timezone: *mut pg_tz = 0 as *const pg_tz as *mut pg_tz;
    static mut cache_tm: pg_tm = pg_tm {
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
        tm_zone: 0 as *const libc::c_char,
    };
    static mut cache_fsec: fsec_t = 0;
    static mut cache_tz: libc::c_int = 0;
    if cur_ts != cache_ts || session_timezone != cache_timezone {
        cache_timezone = 0 as *mut pg_tz;
        if timestamp2tm(
            cur_ts,
            &mut cache_tz,
            &mut cache_tm,
            &mut cache_fsec,
            0 as *mut *const libc::c_char,
            session_timezone,
        ) != 0 as libc::c_int
        {
            let mut __errno_location_0: libc::c_int = 0;
            if if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
                errstart_cold(21 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
            } else {
                errstart(21 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
            } != 0
            {
                errcode(
                    ('2' as i32 - '0' as i32 & 0x3f as libc::c_int)
                        + (('2' as i32 - '0' as i32 & 0x3f as libc::c_int) << 6 as libc::c_int)
                        + (('0' as i32 - '0' as i32 & 0x3f as libc::c_int) << 12 as libc::c_int)
                        + (('0' as i32 - '0' as i32 & 0x3f as libc::c_int) << 18 as libc::c_int)
                        + (('8' as i32 - '0' as i32 & 0x3f as libc::c_int) << 24 as libc::c_int),
                );
                errmsg0(b"timestamp out of range\0" as *const u8 as *const libc::c_char);
                errfinish(
                    b"/home/petrosagg/projects/postgres-datetime/src/datetime.c\0" as *const u8
                        as *const libc::c_char,
                    405 as libc::c_int,
                    (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"GetCurrentTimeUsec\0",
                    ))
                    .as_ptr(),
                );
            }
            if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
                unreachable!();
            }
        }
        cache_ts = cur_ts;
        cache_timezone = session_timezone;
    }
    *tm = cache_tm;
    *fsec = cache_fsec;
    if !tzp.is_null() {
        *tzp = cache_tz;
    }
}
unsafe fn ParseFractionalSecond(mut cp: *mut libc::c_char, fsec: *mut fsec_t) -> libc::c_int {
    *__errno_location() = 0 as libc::c_int;
    let frac = strtod(cp, &mut cp);
    if *cp as libc::c_int != '\0' as i32 || *__errno_location() != 0 as libc::c_int {
        eprintln!("parse fractional second failed");
        return -(1 as libc::c_int);
    }
    *fsec = rint(frac * 1000000 as libc::c_int as libc::c_double) as fsec_t;
    return 0 as libc::c_int;
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
        if chars.peek().unwrap().is_ascii_whitespace() {
            chars.next();
            continue;
        }

        // Record start of current field
        let mut fdata = String::new();
        let mut ftype: TokenFieldType;

        // leading digit? then date or time
        if chars.peek().unwrap().is_ascii_digit() {
            while let Some(c) = chars.next_if(|c| c.is_ascii_digit()) {
                fdata.push(c);
            }

            // time field?
            if *chars.peek().unwrap() == ':' {
                ftype = TokenFieldType::Time;

                while let Some(c) = chars.next_if(|&c| c.is_ascii_digit() || c == ':' || c == '.') {
                    fdata.push(c);
                }
            // date field? allow embedded text month
            } else if matches!(*chars.peek().unwrap(), '-' | '/' | '.') {
                // save delimiting character to use later
                let delim = *chars.peek().unwrap();

                fdata.push(chars.next().unwrap());

                // second field is all digits? then no embedded text month
                if chars.peek().unwrap().is_ascii_digit() {
                    ftype = match delim {
                        '.' => TokenFieldType::Number,
                        _ => TokenFieldType::Date,
                    };
                    while let Some(c) = chars.next_if(|c| c.is_ascii_digit()) {
                        fdata.push(c);
                    }

                    // insist that the delimiters match to get a three-field date.
                    if *chars.peek().unwrap() == delim {
                        ftype = TokenFieldType::Date;

                        fdata.push(chars.next().unwrap());
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
        } else if *chars.peek().unwrap() == '.' {
            fdata.push(chars.next().unwrap());
            while let Some(c) = chars.next_if(|&c| c.is_ascii_digit()) {
                fdata.push(c);
            }
            ftype = TokenFieldType::Number;
        // text? then date string, month, day of week, special, or timezone
        } else if chars.peek().unwrap().is_ascii_alphabetic() {
            ftype = TokenFieldType::String;
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
                if !DATE_TOKEN_TABLE
                    .binary_search_by(|(token, _, _)| token.cmp(&&*fdata))
                    .is_ok()
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
        } else if matches!(*chars.peek().unwrap(), '+' | '-') {
            fdata.push(chars.next().unwrap());
            // soak up leading whitespace
            while chars.next_if(|c| c.is_ascii_whitespace()).is_some() {}
            // numeric timezone?
            // note that "DTK_TZ" could also be a signed float or yyyy-mm
            if chars.peek().unwrap().is_ascii_digit() {
                ftype = TokenFieldType::Tz;
                fdata.push(chars.next().unwrap());
                while let Some(c) =
                    chars.next_if(|&c| c.is_ascii_digit() || matches!(c, ':' | '.' | '-'))
                {
                    fdata.push(c.to_ascii_lowercase());
                }
            // special?
            } else if chars.peek().unwrap().is_ascii_alphabetic() {
                ftype = TokenFieldType::Special;
                while let Some(c) = chars.next_if(|&c| c.is_ascii_alphabetic()) {
                    fdata.push(c.to_ascii_lowercase());
                }
            // otherwise something wrong...
            } else {
                return Err(-1);
            }
        // ignore other punctuation but use as delimiter
        } else if chars.peek().unwrap().is_ascii_punctuation() {
            chars.next();
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
///  		"<weekday> <month>-<day>-<year> <hour>:<minute>:<second>"
///  		"Fri Feb-7-1997 15:23:27"
///  		"Feb-7-1997 15:23:27"
///  		"2-7-1997 15:23:27"
///  		"1997-2-7 15:23:27"
///  		"1997.038 15:23:27"		(day of year 1-366)
///  Also supports input in compact time:
///  		"970207 152327"
///  		"97038 152327"
///  		"20011225T040506.789-07"
///
/// Use the system-provided functions to get the current time zone
/// if not specified in the input string.
///
/// If the date is outside the range of pg_time_t (in practice that could only
/// happen if pg_time_t is just 32 bits), then assume UTC time zone - thomas
/// 1997-05-27

pub unsafe fn DecodeDateTime(
    field: *mut *mut libc::c_char,
    ftype: *mut libc::c_int,
    nf: libc::c_int,
    dtype: *mut libc::c_int,
    mut tm: *mut pg_tm,
    fsec: *mut fsec_t,
    tzp: *mut libc::c_int,
) -> libc::c_int {
    let mut fmask = FieldMask::none();
    let mut tmask = FieldMask::none();
    let mut ptype = TokenFieldType::Number; // "prefix type" for ISO y2001m02d04 format
    let mut val: libc::c_int = 0;
    let mut mer: libc::c_int = 2 as libc::c_int;
    let mut haveTextMonth = false;
    let mut isjulian = false;
    let mut is2digits = false;
    let mut bc = false;
    let mut namedTz: *mut pg_tz = 0 as *mut pg_tz;
    let mut abbrevTz: *mut pg_tz = 0 as *mut pg_tz;
    let mut valtz: *mut pg_tz = 0 as *mut pg_tz;
    let mut abbrev: *mut libc::c_char = 0 as *mut libc::c_char;
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
        tm_zone: 0 as *const libc::c_char,
    };

    // We'll insist on at least all of the date fields, but initialize the
    // remaining fields in case they are not set later...
    *dtype = 2 as libc::c_int;
    (*tm).tm_hour = 0 as libc::c_int;
    (*tm).tm_min = 0 as libc::c_int;
    (*tm).tm_sec = 0 as libc::c_int;
    *fsec = 0 as libc::c_int;

    // don't know daylight savings time status apriori
    (*tm).tm_isdst = None;
    if !tzp.is_null() {
        *tzp = 0 as libc::c_int;
    }
    let mut current_block_236: u64;
    for i in 0..nf {
        match *ftype.offset(i as isize) {
            2 => {
                if ptype == TokenFieldType::Julian {
                    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
                    if tzp.is_null() {
                        eprintln!("tzp is null");
                        return -(1 as libc::c_int);
                    }
                    *__errno_location() = 0 as libc::c_int;
                    let val_0 = strtoint(*field.offset(i as isize), &mut cp, 10 as libc::c_int);
                    if *__errno_location() == 34 as libc::c_int || val_0 < 0 as libc::c_int {
                        return -(2 as libc::c_int);
                    }
                    j2date(
                        val_0,
                        &mut (*tm).tm_year,
                        &mut (*tm).tm_mon,
                        &mut (*tm).tm_mday,
                    );
                    isjulian = true;
                    let dterr = DecodeTimezone(cp, tzp);
                    if dterr != 0 {
                        return dterr;
                    }
                    tmask = *FIELD_MASK_DATE | *FIELD_MASK_TIME | RealFieldType::Tz;
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
                    || fmask.contains(RealFieldType::Month | RealFieldType::Day)
                {
                    // No time zone accepted? Then quit...
                    if tzp.is_null() {
                        eprintln!("tzp is null");
                        return -1;
                    }
                    if *(*__ctype_b_loc())
                        .offset(**field.offset(i as isize) as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                        || ptype != TokenFieldType::Number
                    {
                        if ptype != TokenFieldType::Number {
                            if ptype != TokenFieldType::Time {
                                eprintln!("ptype is not Time: {:?}", ptype);
                                return -(1 as libc::c_int);
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
                        let cp_0 = strchr(*field.offset(i as isize), '-' as i32);
                        if cp_0.is_null() {
                            eprintln!("couldn't find '-' character");
                            return -(1 as libc::c_int);
                        }
                        let dterr = DecodeTimezone(cp_0, tzp);
                        if dterr != 0 {
                            return dterr;
                        }
                        *cp_0 = '\0' as i32 as libc::c_char;
                        let dterr = DecodeNumberField(
                            strlen(*field.offset(i as isize)) as libc::c_int,
                            *field.offset(i as isize),
                            fmask,
                            &mut tmask,
                            tm,
                            fsec,
                            &mut is2digits,
                        );
                        if dterr < 0 as libc::c_int {
                            return dterr;
                        }
                        // modify tmask after returning from DecodeNumberField()
                        tmask.set(RealFieldType::Tz);
                    } else {
                        namedTz = pg_tzset(*field.offset(i as isize));
                        if namedTz.is_null() {
                            let mut __errno_location_0: libc::c_int = 0;
                            if if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
                                errstart_cold(21 as libc::c_int, 0 as *const libc::c_char)
                                    as libc::c_int
                            } else {
                                errstart(21 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
                            } != 0
                            {
                                errcode(
                                    ('2' as i32 - '0' as i32 & 0x3f as libc::c_int)
                                        + (('2' as i32 - '0' as i32 & 0x3f as libc::c_int)
                                            << 6 as libc::c_int)
                                        + (('0' as i32 - '0' as i32 & 0x3f as libc::c_int)
                                            << 12 as libc::c_int)
                                        + (('2' as i32 - '0' as i32 & 0x3f as libc::c_int)
                                            << 18 as libc::c_int)
                                        + (('3' as i32 - '0' as i32 & 0x3f as libc::c_int)
                                            << 24 as libc::c_int),
                                );
                                errmsg(
                                    b"time zone \"%s\" not recognized\0" as *const u8
                                        as *const libc::c_char,
                                    *field.offset(i as isize) as *mut _,
                                );
                                errfinish(
                                    b"/home/petrosagg/projects/postgres-datetime/src/datetime.c\0"
                                        as *const u8
                                        as *const libc::c_char,
                                    952 as libc::c_int,
                                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                                        b"DecodeDateTime\0",
                                    ))
                                    .as_ptr(),
                                );
                            }
                            if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
                                unreachable!();
                            }
                        }
                        // we'll apply the zone setting below
                        tmask = FieldMask::from(RealFieldType::Tz);
                    }
                } else {
                    let dterr = DecodeDate(
                        *field.offset(i as isize),
                        fmask,
                        &mut tmask,
                        &mut is2digits,
                        tm,
                    );
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
                        return -(1 as libc::c_int);
                    }
                    ptype = TokenFieldType::Number
                }
                let dterr = DecodeTime(
                    *field.offset(i as isize),
                    fmask,
                    0x7fff as libc::c_int,
                    &mut tmask,
                    tm,
                    fsec,
                );
                if dterr != 0 {
                    return dterr;
                }
                if time_overflows((*tm).tm_hour, (*tm).tm_min, (*tm).tm_sec, *fsec) {
                    return -(2 as libc::c_int);
                }
                current_block_236 = 13797367574128857302;
            }
            4 => {
                let mut tz: libc::c_int = 0;
                if tzp.is_null() {
                    eprintln!("tzp is null");
                    return -(1 as libc::c_int);
                }
                let dterr = DecodeTimezone(*field.offset(i as isize), &mut tz);
                if dterr != 0 {
                    return dterr;
                }
                *tzp = tz;
                tmask = FieldMask::from(RealFieldType::Tz);
                current_block_236 = 13797367574128857302;
            }
            0 => {
                if ptype != TokenFieldType::Number {
                    let mut cp_1: *mut libc::c_char = 0 as *mut libc::c_char;
                    *__errno_location() = 0 as libc::c_int;
                    let val_1 = strtoint(*field.offset(i as isize), &mut cp_1, 10 as libc::c_int);
                    if *__errno_location() == 34 as libc::c_int {
                        return -(2 as libc::c_int);
                    }
                    if *cp_1 as libc::c_int == '.' as i32 {
                        match ptype {
                            TokenFieldType::Julian
                            | TokenFieldType::Time
                            | TokenFieldType::Second => {}
                            _ => {
                                eprintln!("ptype is not Julian, Time, or Second: {:?}", ptype);
                                return -(1 as libc::c_int);
                            }
                        }
                    } else if *cp_1 as libc::c_int != '\0' as i32 {
                        eprintln!("expected EOF");
                        return -(1 as libc::c_int);
                    }
                    match ptype {
                        TokenFieldType::Year => {
                            (*tm).tm_year = val_1;
                            tmask = FieldMask::from(RealFieldType::Year);
                        }
                        TokenFieldType::Month => {
                            // already have a month and hour? then assume minutes
                            if fmask.contains(RealFieldType::Month | RealFieldType::Hour) {
                                (*tm).tm_min = val_1;
                                tmask = FieldMask::from(RealFieldType::Minute);
                            } else {
                                (*tm).tm_mon = val_1;
                                tmask = FieldMask::from(RealFieldType::Month);
                            }
                        }
                        TokenFieldType::Day => {
                            (*tm).tm_mday = val_1;
                            tmask = FieldMask::from(RealFieldType::Day);
                        }
                        TokenFieldType::Hour => {
                            (*tm).tm_hour = val_1;
                            tmask = FieldMask::from(RealFieldType::Hour);
                        }
                        TokenFieldType::Minute => {
                            (*tm).tm_min = val_1;
                            tmask = FieldMask::from(RealFieldType::Minute);
                        }
                        TokenFieldType::Second => {
                            (*tm).tm_sec = val_1;
                            tmask = FieldMask::from(RealFieldType::Second);
                            if *cp_1 as libc::c_int == '.' as i32 {
                                let dterr = ParseFractionalSecond(cp_1, fsec);
                                if dterr != 0 {
                                    return dterr;
                                }
                                tmask = *FIELD_MASK_ALL_SECS;
                            }
                        }
                        TokenFieldType::Tz => {
                            tmask = FieldMask::from(RealFieldType::Tz);
                            let dterr = DecodeTimezone(*field.offset(i as isize), tzp);
                            if dterr != 0 {
                                return dterr;
                            }
                        }
                        TokenFieldType::Julian => {
                            if val_1 < 0 as libc::c_int {
                                return -(2 as libc::c_int);
                            }
                            tmask = *FIELD_MASK_DATE;
                            j2date(
                                val_1,
                                &mut (*tm).tm_year,
                                &mut (*tm).tm_mon,
                                &mut (*tm).tm_mday,
                            );
                            isjulian = true;
                            if *cp_1 as libc::c_int == '.' as i32 {
                                *__errno_location() = 0 as libc::c_int;
                                let mut time = strtod(cp_1, &mut cp_1);
                                if *cp_1 as libc::c_int != '\0' as i32
                                    || *__errno_location() != 0 as libc::c_int
                                {
                                    eprintln!("unclear what happened");
                                    return -(1 as libc::c_int);
                                }
                                time *= 86400000000 as libc::c_long as libc::c_double;
                                dt2time(
                                    time as Timestamp,
                                    &mut (*tm).tm_hour,
                                    &mut (*tm).tm_min,
                                    &mut (*tm).tm_sec,
                                    fsec,
                                );
                                tmask.set(*FIELD_MASK_TIME);
                            }
                        }
                        TokenFieldType::Time => {
                            let dterr = DecodeNumberField(
                                strlen(*field.offset(i as isize)) as libc::c_int,
                                *field.offset(i as isize),
                                fmask | *FIELD_MASK_DATE,
                                &mut tmask,
                                tm,
                                fsec,
                                &mut is2digits,
                            );
                            if dterr < 0 as libc::c_int {
                                return dterr;
                            }
                            if tmask != *FIELD_MASK_TIME {
                                eprintln!("tmask is not FIELD_MASK_TIME");
                                return -(1 as libc::c_int);
                            }
                        }
                        typ => {
                            eprintln!("unexpected ptype: {:?}", typ);
                            return -(1 as libc::c_int);
                        }
                    }
                    ptype = TokenFieldType::Number;
                    *dtype = 2 as libc::c_int;
                } else {
                    let flen = strlen(*field.offset(i as isize)) as libc::c_int;
                    let cp_2 = strchr(*field.offset(i as isize), '.' as i32);
                    if !cp_2.is_null() && !fmask.intersects(*FIELD_MASK_DATE) {
                        let dterr = DecodeDate(
                            *field.offset(i as isize),
                            fmask,
                            &mut tmask,
                            &mut is2digits,
                            tm,
                        );
                        if dterr != 0 {
                            return dterr;
                        }
                    } else if !cp_2.is_null()
                        && (flen as libc::c_ulong).wrapping_sub(strlen(cp_2))
                            > 2 as libc::c_int as libc::c_ulong
                    {
                        let dterr = DecodeNumberField(
                            flen,
                            *field.offset(i as isize),
                            fmask,
                            &mut tmask,
                            tm,
                            fsec,
                            &mut is2digits,
                        );
                        if dterr < 0 as libc::c_int {
                            return dterr;
                        }
                    } else if flen >= 6
                        && (!fmask.intersects(*FIELD_MASK_DATE)
                            || !fmask.intersects(*FIELD_MASK_TIME))
                    {
                        let dterr = DecodeNumberField(
                            flen,
                            *field.offset(i as isize),
                            fmask,
                            &mut tmask,
                            tm,
                            fsec,
                            &mut is2digits,
                        );
                        if dterr < 0 as libc::c_int {
                            return dterr;
                        }
                    } else {
                        let dterr = DecodeNumber(
                            flen,
                            *field.offset(i as isize),
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
                let mut type_0 =
                    DecodeTimezoneAbbrev(i, *field.offset(i as isize), &mut val, &mut valtz);
                if type_0 == RealFieldType::UnknownField {
                    type_0 = DecodeSpecial(i, *field.offset(i as isize), &mut val);
                }
                if type_0 == RealFieldType::IgnoreDtf {
                    current_block_236 = 12209867499936983673;
                } else {
                    tmask = FieldMask::from(type_0);
                    match type_0 {
                        RealFieldType::Reserved => match val {
                            12 => {
                                tmask = *FIELD_MASK_DATE | *FIELD_MASK_TIME | RealFieldType::Tz;
                                *dtype = 2 as libc::c_int;
                                GetCurrentTimeUsec(tm, fsec, tzp);
                            }
                            13 => {
                                tmask = *FIELD_MASK_DATE;
                                *dtype = 2 as libc::c_int;
                                GetCurrentDateTime(&mut cur_tm);
                                j2date(
                                    date2j(cur_tm.tm_year, cur_tm.tm_mon, cur_tm.tm_mday)
                                        - 1 as libc::c_int,
                                    &mut (*tm).tm_year,
                                    &mut (*tm).tm_mon,
                                    &mut (*tm).tm_mday,
                                );
                            }
                            14 => {
                                tmask = *FIELD_MASK_DATE;
                                *dtype = 2 as libc::c_int;
                                GetCurrentDateTime(&mut cur_tm);
                                (*tm).tm_year = cur_tm.tm_year;
                                (*tm).tm_mon = cur_tm.tm_mon;
                                (*tm).tm_mday = cur_tm.tm_mday;
                            }
                            15 => {
                                tmask = *FIELD_MASK_DATE;
                                *dtype = 2 as libc::c_int;
                                GetCurrentDateTime(&mut cur_tm);
                                j2date(
                                    date2j(cur_tm.tm_year, cur_tm.tm_mon, cur_tm.tm_mday)
                                        + 1 as libc::c_int,
                                    &mut (*tm).tm_year,
                                    &mut (*tm).tm_mon,
                                    &mut (*tm).tm_mday,
                                );
                            }
                            16 => {
                                tmask = *FIELD_MASK_TIME | RealFieldType::Tz;
                                *dtype = 2 as libc::c_int;
                                (*tm).tm_hour = 0 as libc::c_int;
                                (*tm).tm_min = 0 as libc::c_int;
                                (*tm).tm_sec = 0 as libc::c_int;
                                if !tzp.is_null() {
                                    *tzp = 0 as libc::c_int;
                                }
                            }
                            _ => {
                                *dtype = val;
                            }
                        },
                        RealFieldType::Month => {
                            // already have a (numeric) month? then see if we can substitute...
                            if fmask.contains(RealFieldType::Month)
                                && !haveTextMonth
                                && !fmask.contains(RealFieldType::Day)
                                && (*tm).tm_mon >= 1 as libc::c_int
                                && (*tm).tm_mon <= 31 as libc::c_int
                            {
                                (*tm).tm_mday = (*tm).tm_mon;
                                tmask = FieldMask::from(RealFieldType::Day);
                            }
                            haveTextMonth = true;
                            (*tm).tm_mon = val;
                        }
                        RealFieldType::DtzMod => {
                            tmask.set(RealFieldType::DTz);
                            (*tm).tm_isdst = Some(true);
                            if tzp.is_null() {
                                eprintln!("tzp is null");
                                return -(1 as libc::c_int);
                            }
                            *tzp -= val;
                        }
                        RealFieldType::DTz => {
                            tmask.set(RealFieldType::Tz);
                            (*tm).tm_isdst = Some(true);
                            if tzp.is_null() {
                                eprintln!("tzp is null");
                                return -(1 as libc::c_int);
                            }
                            *tzp = -val;
                        }
                        RealFieldType::Tz => {
                            (*tm).tm_isdst = Some(false);
                            if tzp.is_null() {
                                eprintln!("tzp is null");
                                return -(1 as libc::c_int);
                            }
                            *tzp = -val;
                        }
                        RealFieldType::DynTz => {
                            tmask.set(RealFieldType::Tz);
                            if tzp.is_null() {
                                eprintln!("tzp is null");
                                return -(1 as libc::c_int);
                            }
                            abbrevTz = valtz;
                            abbrev = *field.offset(i as isize);
                        }
                        RealFieldType::AmPm => {
                            mer = val;
                        }
                        RealFieldType::Adbc => {
                            bc = val == 1;
                        }
                        RealFieldType::Dow => {
                            (*tm).tm_wday = val;
                        }
                        RealFieldType::Units => {
                            tmask = FieldMask::none();
                            ptype = val.try_into().unwrap();
                        }
                        RealFieldType::IsoTime => {
                            // This is a filler field "t" indicating that the next
                            // field is time. Try to verify that this is sensible.
                            tmask = FieldMask::none();

                            // No preceding date? Then quit...
                            if !fmask.contains(*FIELD_MASK_DATE) {
                                eprintln!("no preceding date");
                                return -(1 as libc::c_int);
                            }

                            // We will need one of the following fields:
                            //	DTK_NUMBER should be hhmmss.fff
                            //	DTK_TIME should be hh:mm:ss.fff
                            //	DTK_DATE should be hhmmss-zz
                            if i >= nf - 1 as libc::c_int
                                || *ftype.offset((i + 1 as libc::c_int) as isize)
                                    != 0 as libc::c_int
                                    && *ftype.offset((i + 1 as libc::c_int) as isize)
                                        != 3 as libc::c_int
                                    && *ftype.offset((i + 1 as libc::c_int) as isize)
                                        != 2 as libc::c_int
                            {
                                eprintln!("next field are not the right type");
                                return -(1 as libc::c_int);
                            }
                            ptype = val.try_into().unwrap();
                        }
                        RealFieldType::UnknownField => {
                            // Before giving up and declaring error, check to see
                            // if it is an all-alpha timezone name.
                            namedTz = pg_tzset(*field.offset(i as isize));
                            if namedTz.is_null() {
                                eprintln!("namedTz is null");
                                return -(1 as libc::c_int);
                            }
                            tmask = FieldMask::from(RealFieldType::Tz);
                        }
                        typ => {
                            eprintln!("unexpected field type: {:?}", typ);
                            return -(1 as libc::c_int);
                        }
                    }
                    current_block_236 = 13797367574128857302;
                }
            }
            _ => return -(1 as libc::c_int),
        }
        match current_block_236 {
            13797367574128857302 => {
                if tmask.intersects(fmask) {
                    return -(1 as libc::c_int);
                }
                fmask |= tmask;
            }
            _ => {}
        }
    }
    // do final checking/adjustment of Y/M/D fields
    let dterr = ValidateDate(fmask, isjulian, is2digits, bc, tm);
    if dterr != 0 {
        return dterr;
    }
    // handle AM/PM
    if mer != 2 as libc::c_int && (*tm).tm_hour > 24 as libc::c_int / 2 as libc::c_int {
        return -(2 as libc::c_int);
    }
    if mer == 0 as libc::c_int && (*tm).tm_hour == 24 as libc::c_int / 2 as libc::c_int {
        (*tm).tm_hour = 0 as libc::c_int;
    } else if mer == 1 as libc::c_int && (*tm).tm_hour != 24 as libc::c_int / 2 as libc::c_int {
        (*tm).tm_hour += 24 as libc::c_int / 2 as libc::c_int;
    }
    // do additional checking for full date specs...
    if *dtype == 2 as libc::c_int {
        if !fmask.contains(*FIELD_MASK_DATE) {
            if fmask.contains(*FIELD_MASK_TIME) {
                // TODO(petrosagg): this is actually success, as noted in the function doc
                return 1 as libc::c_int;
            }
            return -(1 as libc::c_int);
        }
        // If we had a full timezone spec, compute the offset (we could not do
        // it before, because we need the date to resolve DST status).
        if !namedTz.is_null() {
            // daylight savings time modifier disallowed with full TZ
            if fmask.contains(RealFieldType::DtzMod) {
                return -(1 as libc::c_int);
            }
            *tzp = DetermineTimeZoneOffset(tm, namedTz);
        }
        // Likewise, if we had a dynamic timezone abbreviation, resolve it now.
        if !abbrevTz.is_null() {
            if fmask.contains(RealFieldType::DtzMod) {
                return -(1 as libc::c_int);
            }
            *tzp = DetermineTimeZoneAbbrevOffset(tm, abbrev, abbrevTz);
        }
        // timezone not specified? then use session timezone
        if !tzp.is_null() && !fmask.contains(RealFieldType::Tz) {
            // daylight savings time modifier but no standard timezone? then error
            if fmask.contains(RealFieldType::DtzMod) {
                return -(1 as libc::c_int);
            }
            *tzp = DetermineTimeZoneOffset(tm, session_timezone);
        }
    }
    return 0 as libc::c_int;
}

unsafe fn DetermineTimeZoneOffset(tm: *mut pg_tm, tzp: *mut pg_tz) -> libc::c_int {
    let mut t: pg_time_t = 0;
    return DetermineTimeZoneOffsetInternal(tm, tzp, &mut t);
}
unsafe fn DetermineTimeZoneOffsetInternal(
    mut tm: *mut pg_tm,
    tzp: *mut pg_tz,
    tp: *mut pg_time_t,
) -> libc::c_int {
    let mut boundary: pg_time_t = 0;
    let mut before_gmtoff: libc::c_long = 0;
    let mut after_gmtoff: libc::c_long = 0;
    let mut before_isdst = false;
    let mut after_isdst = false;
    if ((*tm).tm_year > -(4713 as libc::c_int)
        || (*tm).tm_year == -(4713 as libc::c_int) && (*tm).tm_mon >= 11 as libc::c_int)
        && ((*tm).tm_year < 5874898 as libc::c_int
            || (*tm).tm_year == 5874898 as libc::c_int && (*tm).tm_mon < 6 as libc::c_int)
    {
        let date = date2j((*tm).tm_year, (*tm).tm_mon, (*tm).tm_mday) - 2440588 as libc::c_int;
        let day = date as pg_time_t * 86400 as libc::c_int as libc::c_long;
        if !(day / 86400 as libc::c_int as libc::c_long != date as libc::c_long) {
            let sec = (*tm).tm_sec
                + ((*tm).tm_min + (*tm).tm_hour * 60 as libc::c_int) * 60 as libc::c_int;
            let mytime = day + sec as libc::c_long;
            if !(mytime < 0 as libc::c_int as libc::c_long
                && day > 0 as libc::c_int as libc::c_long)
            {
                let mut prevtime = mytime - 86400 as libc::c_int as libc::c_long;
                if !(mytime < 0 as libc::c_int as libc::c_long
                    && prevtime > 0 as libc::c_int as libc::c_long)
                {
                    let res = pg_next_dst_boundary(
                        &mut prevtime,
                        &mut before_gmtoff,
                        &mut before_isdst,
                        &mut boundary,
                        &mut after_gmtoff,
                        &mut after_isdst,
                        tzp,
                    );
                    if !(res < 0 as libc::c_int) {
                        if res == 0 as libc::c_int {
                            (*tm).tm_isdst = Some(before_isdst);
                            *tp = mytime - before_gmtoff;
                            return -(before_gmtoff as libc::c_int);
                        }
                        let beforetime = mytime - before_gmtoff;
                        if !(before_gmtoff > 0 as libc::c_int as libc::c_long
                            && mytime < 0 as libc::c_int as libc::c_long
                            && beforetime > 0 as libc::c_int as libc::c_long
                            || before_gmtoff <= 0 as libc::c_int as libc::c_long
                                && mytime > 0 as libc::c_int as libc::c_long
                                && beforetime < 0 as libc::c_int as libc::c_long)
                        {
                            let aftertime = mytime - after_gmtoff;
                            if !(after_gmtoff > 0 as libc::c_int as libc::c_long
                                && mytime < 0 as libc::c_int as libc::c_long
                                && aftertime > 0 as libc::c_int as libc::c_long
                                || after_gmtoff <= 0 as libc::c_int as libc::c_long
                                    && mytime > 0 as libc::c_int as libc::c_long
                                    && aftertime < 0 as libc::c_int as libc::c_long)
                            {
                                if beforetime < boundary && aftertime < boundary {
                                    (*tm).tm_isdst = Some(before_isdst);
                                    *tp = beforetime;
                                    return -(before_gmtoff as libc::c_int);
                                }
                                if beforetime > boundary && aftertime >= boundary {
                                    (*tm).tm_isdst = Some(after_isdst);
                                    *tp = aftertime;
                                    return -(after_gmtoff as libc::c_int);
                                }
                                if beforetime > aftertime {
                                    (*tm).tm_isdst = Some(before_isdst);
                                    *tp = beforetime;
                                    return -(before_gmtoff as libc::c_int);
                                }
                                (*tm).tm_isdst = Some(after_isdst);
                                *tp = aftertime;
                                return -(after_gmtoff as libc::c_int);
                            }
                        }
                    }
                }
            }
        }
    }
    (*tm).tm_isdst = Some(false);
    *tp = 0 as libc::c_int as pg_time_t;
    return 0 as libc::c_int;
}

unsafe fn DetermineTimeZoneAbbrevOffset(
    mut tm: *mut pg_tm,
    abbr: *const libc::c_char,
    tzp: *mut pg_tz,
) -> libc::c_int {
    let mut t: pg_time_t = 0;
    let mut abbr_offset: libc::c_int = 0;
    let mut abbr_isdst = false;
    let zone_offset = DetermineTimeZoneOffsetInternal(tm, tzp, &mut t);
    if DetermineTimeZoneAbbrevOffsetInternal(t, abbr, tzp, &mut abbr_offset, &mut abbr_isdst) {
        (*tm).tm_isdst = Some(abbr_isdst);
        return abbr_offset;
    }
    return zone_offset;
}

unsafe fn DetermineTimeZoneAbbrevOffsetInternal(
    mut t: pg_time_t,
    abbr: *const libc::c_char,
    tzp: *mut pg_tz,
    offset: *mut libc::c_int,
    isdst: &mut bool,
) -> bool {
    let mut upabbr: [libc::c_char; 256] = [0; 256];
    let mut gmtoff: libc::c_long = 0;
    strlcpy(
        upabbr.as_mut_ptr(),
        abbr,
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
    let mut p = upabbr.as_mut_ptr() as *mut libc::c_uchar;
    while *p != 0 {
        *p = pg_toupper(*p);
        p = p.offset(1);
    }
    if pg_interpret_timezone_abbrev(upabbr.as_mut_ptr(), &mut t, &mut gmtoff, isdst, tzp) {
        *offset = -gmtoff as libc::c_int;
        return true;
    }
    return false;
}

unsafe fn DecodeDate(
    mut str: *mut libc::c_char,
    mut fmask: FieldMask,
    tmask: &mut FieldMask,
    is2digits: &mut bool,
    mut tm: *mut pg_tm,
) -> libc::c_int {
    let mut fsec: fsec_t = 0;
    let mut nf: libc::c_int = 0 as libc::c_int;
    let mut haveTextMonth: bool = false;
    let mut val: libc::c_int = 0;
    let mut dmask = FieldMask::none();
    let mut field: [*mut libc::c_char; 25] = [0 as *mut libc::c_char; 25];
    *tmask = FieldMask::none();
    while *str as libc::c_int != '\0' as i32 && nf < 25 as libc::c_int {
        while *str as libc::c_int != '\0' as i32
            && *(*__ctype_b_loc()).offset(*str as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                == 0
        {
            str = str.offset(1);
        }
        if *str as libc::c_int == '\0' as i32 {
            return -(1 as libc::c_int);
        }
        field[nf as usize] = str;
        if *(*__ctype_b_loc()).offset(*str as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            while *(*__ctype_b_loc()).offset(*str as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                str = str.offset(1);
            }
        } else if *(*__ctype_b_loc()).offset(*str as libc::c_uchar as libc::c_int as isize)
            as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            while *(*__ctype_b_loc()).offset(*str as libc::c_uchar as libc::c_int as isize)
                as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                str = str.offset(1);
            }
        }
        if *str as libc::c_int != '\0' as i32 {
            let fresh42 = str;
            str = str.offset(1);
            *fresh42 = '\0' as i32 as libc::c_char;
        }
        nf += 1;
    }
    for i in 0..nf {
        if *(*__ctype_b_loc()).offset(*field[i as usize] as libc::c_uchar as libc::c_int as isize)
            as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            let type_0 = DecodeSpecial(i, field[i as usize], &mut val);
            if type_0 != RealFieldType::IgnoreDtf {
                dmask = FieldMask::from(type_0);
                match type_0 {
                    RealFieldType::Month => {
                        (*tm).tm_mon = val;
                        haveTextMonth = true;
                    }
                    typ => {
                        eprintln!("unexpected field type: {:?}", typ);
                        return -(1 as libc::c_int);
                    }
                }
                if fmask.intersects(dmask) {
                    return -(1 as libc::c_int);
                }
                fmask |= dmask;
                *tmask |= dmask;
                field[i as usize] = 0 as *mut libc::c_char;
            }
        }
    }
    for i in 0..nf {
        if !(field[i as usize]).is_null() {
            let len = strlen(field[i as usize]) as libc::c_int;
            if len <= 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            let dterr = DecodeNumber(
                len,
                field[i as usize],
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
                return -(1 as libc::c_int);
            }
            fmask |= dmask;
            *tmask |= dmask;
        }
    }
    if fmask & !(RealFieldType::Doy | RealFieldType::Tz) != *FIELD_MASK_DATE {
        return -(1 as libc::c_int);
    }

    // validation of the field values must wait until ValidateDate()
    return 0 as libc::c_int;
}

/// Check valid year/month/day values, handle BC and DOY cases Return 0 if okay, a DTERR code if not.

unsafe fn ValidateDate(
    fmask: FieldMask,
    isjulian: bool,
    is2digits: bool,
    bc: bool,
    mut tm: *mut pg_tm,
) -> libc::c_int {
    if fmask.contains(RealFieldType::Year) {
        if !isjulian {
            if bc {
                if (*tm).tm_year <= 0 as libc::c_int {
                    return -(2 as libc::c_int);
                }
                (*tm).tm_year = -((*tm).tm_year - 1 as libc::c_int);
            } else if is2digits {
                if (*tm).tm_year < 0 as libc::c_int {
                    return -(2 as libc::c_int);
                }
                if (*tm).tm_year < 70 as libc::c_int {
                    (*tm).tm_year += 2000 as libc::c_int;
                } else if (*tm).tm_year < 100 as libc::c_int {
                    (*tm).tm_year += 1900 as libc::c_int;
                }
            } else if (*tm).tm_year <= 0 as libc::c_int {
                return -(2 as libc::c_int);
            }
        }
    }
    // now that we have correct year, decode DOY
    if fmask.contains(RealFieldType::Doy) {
        j2date(
            date2j((*tm).tm_year, 1 as libc::c_int, 1 as libc::c_int) + (*tm).tm_yday
                - 1 as libc::c_int,
            &mut (*tm).tm_year,
            &mut (*tm).tm_mon,
            &mut (*tm).tm_mday,
        );
    }
    // check for valid month
    if fmask.contains(RealFieldType::Month) {
        if (*tm).tm_mon < 1 as libc::c_int || (*tm).tm_mon > 12 as libc::c_int {
            return -(3 as libc::c_int);
        }
    }
    // minimal check for valid day
    if fmask.contains(RealFieldType::Day) {
        if (*tm).tm_mday < 1 as libc::c_int || (*tm).tm_mday > 31 as libc::c_int {
            return -(3 as libc::c_int);
        }
    }
    if fmask.contains(*FIELD_MASK_DATE) {
        if (*tm).tm_mday
            > day_tab[((*tm).tm_year % 4 as libc::c_int == 0 as libc::c_int
                && ((*tm).tm_year % 100 as libc::c_int != 0 as libc::c_int
                    || (*tm).tm_year % 400 as libc::c_int == 0 as libc::c_int))
                as libc::c_int as usize][((*tm).tm_mon - 1 as libc::c_int) as usize]
        {
            return -(2 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}

/// Decode time string which includes delimiters.
/// Return 0 if okay, a DTERR code if not.
///
/// Only check the lower limit on hours, since this same code can be
/// used to represent time spans.
unsafe fn DecodeTime(
    str: *mut libc::c_char,
    _fmask: FieldMask,
    range: libc::c_int,
    tmask: &mut FieldMask,
    mut tm: *mut pg_tm,
    fsec: *mut fsec_t,
) -> libc::c_int {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    *tmask = *FIELD_MASK_TIME;
    *__errno_location() = 0 as libc::c_int;
    (*tm).tm_hour = strtoint(str, &mut cp, 10 as libc::c_int);
    if *__errno_location() == 34 as libc::c_int {
        return -(2 as libc::c_int);
    }
    if *cp as libc::c_int != ':' as i32 {
        return -(1 as libc::c_int);
    }
    *__errno_location() = 0 as libc::c_int;
    (*tm).tm_min = strtoint(
        cp.offset(1 as libc::c_int as isize),
        &mut cp,
        10 as libc::c_int,
    );
    if *__errno_location() == 34 as libc::c_int {
        return -(2 as libc::c_int);
    }
    if *cp as libc::c_int == '\0' as i32 {
        (*tm).tm_sec = 0 as libc::c_int;
        *fsec = 0 as libc::c_int;
        if range
            == (1 as libc::c_int) << 11 as libc::c_int | (1 as libc::c_int) << 12 as libc::c_int
        {
            (*tm).tm_sec = (*tm).tm_min;
            (*tm).tm_min = (*tm).tm_hour;
            (*tm).tm_hour = 0 as libc::c_int;
        }
    } else if *cp as libc::c_int == '.' as i32 {
        let dterr = ParseFractionalSecond(cp, fsec);
        if dterr != 0 {
            return dterr;
        }
        (*tm).tm_sec = (*tm).tm_min;
        (*tm).tm_min = (*tm).tm_hour;
        (*tm).tm_hour = 0 as libc::c_int;
    } else if *cp as libc::c_int == ':' as i32 {
        *__errno_location() = 0 as libc::c_int;
        (*tm).tm_sec = strtoint(
            cp.offset(1 as libc::c_int as isize),
            &mut cp,
            10 as libc::c_int,
        );
        if *__errno_location() == 34 as libc::c_int {
            return -(2 as libc::c_int);
        }
        if *cp as libc::c_int == '\0' as i32 {
            *fsec = 0 as libc::c_int;
        } else if *cp as libc::c_int == '.' as i32 {
            let dterr = ParseFractionalSecond(cp, fsec);
            if dterr != 0 {
                return dterr;
            }
        } else {
            return -(1 as libc::c_int);
        }
    } else {
        return -(1 as libc::c_int);
    }
    if (*tm).tm_hour < 0 as libc::c_int
        || (*tm).tm_min < 0 as libc::c_int
        || (*tm).tm_min > 60 as libc::c_int - 1 as libc::c_int
        || (*tm).tm_sec < 0 as libc::c_int
        || (*tm).tm_sec > 60 as libc::c_int
        || (*fsec as libc::c_long) < 0 as libc::c_long
        || *fsec as libc::c_long > 1000000 as libc::c_long
    {
        return -(2 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe fn DecodeNumber(
    flen: libc::c_int,
    str: *mut libc::c_char,
    haveTextMonth: bool,
    fmask: FieldMask,
    tmask: &mut FieldMask,
    mut tm: *mut pg_tm,
    fsec: *mut fsec_t,
    is2digits: &mut bool,
) -> libc::c_int {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    *tmask = FieldMask::none();
    *__errno_location() = 0 as libc::c_int;
    let val = strtoint(str, &mut cp, 10 as libc::c_int);
    if *__errno_location() == 34 as libc::c_int {
        return -(2 as libc::c_int);
    }
    if cp == str {
        return -(1 as libc::c_int);
    }
    if *cp as libc::c_int == '.' as i32 {
        if cp.offset_from(str) as libc::c_long > 2 as libc::c_int as libc::c_long {
            let dterr = DecodeNumberField(
                flen,
                str,
                fmask | *FIELD_MASK_DATE,
                tmask,
                tm,
                fsec,
                is2digits,
            );
            if dterr < 0 as libc::c_int {
                return dterr;
            }
            return 0 as libc::c_int;
        }
        let dterr = ParseFractionalSecond(cp, fsec);
        if dterr != 0 {
            return dterr;
        }
    } else if *cp as libc::c_int != '\0' as i32 {
        return -(1 as libc::c_int);
    }
    /* Special case for day of year */
    if flen == 3 as libc::c_int
        && fmask & *FIELD_MASK_DATE == FieldMask::from(RealFieldType::Year)
        && val >= 1 as libc::c_int
        && val <= 366 as libc::c_int
    {
        *tmask = RealFieldType::Doy | RealFieldType::Month | RealFieldType::Day;
        (*tm).tm_yday = val;
        // tm_mon and tm_mday can't actually be set yet ...
        return 0 as libc::c_int;
    }
    // Switch based on what we have so far
    match *(fmask & *FIELD_MASK_DATE) {
        0 => {
            if flen >= 3 as libc::c_int || DateOrder == 0 as libc::c_int {
                *tmask = FieldMask::from(RealFieldType::Year);
                (*tm).tm_year = val;
            } else if DateOrder == 1 as libc::c_int {
                *tmask = FieldMask::from(RealFieldType::Day);
                (*tm).tm_mday = val;
            } else {
                *tmask = FieldMask::from(RealFieldType::Month);
                (*tm).tm_mon = val;
            }
        }
        4 => {
            // Must be at second field of YY-MM-DD
            *tmask = FieldMask::from(RealFieldType::Month);
            (*tm).tm_mon = val;
        }
        2 => {
            if haveTextMonth {
                if flen >= 3 as libc::c_int || DateOrder == 0 as libc::c_int {
                    *tmask = FieldMask::from(RealFieldType::Year);
                    (*tm).tm_year = val;
                } else {
                    *tmask = FieldMask::from(RealFieldType::Day);
                    (*tm).tm_mday = val;
                }
            } else {
                *tmask = FieldMask::from(RealFieldType::Day);
                (*tm).tm_mday = val;
            }
        }
        6 => {
            if haveTextMonth {
                // Need to accept DD-MON-YYYY even in YMD mode
                if flen >= 3 as libc::c_int && *is2digits as libc::c_int != 0 {
                    // Guess that first numeric field is day was wrong
                    // YEAR is already set
                    *tmask = FieldMask::from(RealFieldType::Day);
                    (*tm).tm_mday = (*tm).tm_year;
                    (*tm).tm_year = val;
                    *is2digits = false;
                } else {
                    *tmask = FieldMask::from(RealFieldType::Day);
                    (*tm).tm_mday = val;
                }
            } else {
                // Must be at third field of YY-MM-DD
                *tmask = FieldMask::from(RealFieldType::Day);
                (*tm).tm_mday = val;
            }
        }
        8 => {
            // Must be at second field of DD-MM-YY
            *tmask = FieldMask::from(RealFieldType::Month);
            (*tm).tm_mon = val;
        }
        10 => {
            *tmask = FieldMask::from(RealFieldType::Year);
            (*tm).tm_year = val;
        }
        14 => {
            let dterr = DecodeNumberField(flen, str, fmask, tmask, tm, fsec, is2digits);
            if dterr < 0 as libc::c_int {
                return dterr;
            }
            return 0 as libc::c_int;
        }
        _ => return -(1 as libc::c_int),
    }
    // When processing a year field, mark it for adjustment if it's only one or two digits.
    if *tmask == FieldMask::from(RealFieldType::Year) {
        *is2digits = flen <= 2;
    }
    return 0 as libc::c_int;
}
unsafe fn DecodeNumberField(
    mut len: libc::c_int,
    str: *mut libc::c_char,
    fmask: FieldMask,
    tmask: &mut FieldMask,
    mut tm: *mut pg_tm,
    fsec: *mut fsec_t,
    is2digits: &mut bool,
) -> libc::c_int {
    let cp = strchr(str, '.' as i32);
    if !cp.is_null() {
        *__errno_location() = 0 as libc::c_int;
        let frac = strtod(cp, 0 as *mut *mut libc::c_char);
        if *__errno_location() != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        *fsec = rint(frac * 1000000 as libc::c_int as libc::c_double) as fsec_t;
        *cp = '\0' as i32 as libc::c_char;
        len = strlen(str) as libc::c_int;
    // No decimal point and no complete date yet?
    } else if !fmask.contains(*FIELD_MASK_DATE) {
        if len >= 6 as libc::c_int {
            *tmask = *FIELD_MASK_DATE;
            (*tm).tm_mday = atoi(str.offset((len - 2 as libc::c_int) as isize));
            *str.offset((len - 2 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
            (*tm).tm_mon = atoi(str.offset((len - 4 as libc::c_int) as isize));
            *str.offset((len - 4 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
            (*tm).tm_year = atoi(str);
            if len - 4 as libc::c_int == 2 as libc::c_int {
                *is2digits = true;
            }
            return 2 as libc::c_int;
        }
    }
    if !fmask.contains(*FIELD_MASK_TIME) {
        // hhmmss
        if len == 6 as libc::c_int {
            *tmask = *FIELD_MASK_TIME;
            (*tm).tm_sec = atoi(str.offset(4 as libc::c_int as isize));
            *str.offset(4 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            (*tm).tm_min = atoi(str.offset(2 as libc::c_int as isize));
            *str.offset(2 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            (*tm).tm_hour = atoi(str);
            return 3 as libc::c_int;
        } else {
            if len == 4 as libc::c_int {
                *tmask = *FIELD_MASK_TIME;
                (*tm).tm_sec = 0 as libc::c_int;
                (*tm).tm_min = atoi(str.offset(2 as libc::c_int as isize));
                *str.offset(2 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                (*tm).tm_hour = atoi(str);
                return 3 as libc::c_int;
            }
        }
    }
    return -(1 as libc::c_int);
}

unsafe fn DecodeTimezone(str: *mut libc::c_char, tzp: *mut libc::c_int) -> libc::c_int {
    let mut tz: libc::c_int;
    let min: libc::c_int;
    let mut sec: libc::c_int = 0 as libc::c_int;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if *str as libc::c_int != '+' as i32 && *str as libc::c_int != '-' as i32 {
        return -(1 as libc::c_int);
    }
    *__errno_location() = 0 as libc::c_int;
    let mut hr = strtoint(
        str.offset(1 as libc::c_int as isize),
        &mut cp,
        10 as libc::c_int,
    );
    if *__errno_location() == 34 as libc::c_int {
        return -(5 as libc::c_int);
    }
    if *cp as libc::c_int == ':' as i32 {
        *__errno_location() = 0 as libc::c_int;
        min = strtoint(
            cp.offset(1 as libc::c_int as isize),
            &mut cp,
            10 as libc::c_int,
        );
        if *__errno_location() == 34 as libc::c_int {
            return -(5 as libc::c_int);
        }
        if *cp as libc::c_int == ':' as i32 {
            *__errno_location() = 0 as libc::c_int;
            sec = strtoint(
                cp.offset(1 as libc::c_int as isize),
                &mut cp,
                10 as libc::c_int,
            );
            if *__errno_location() == 34 as libc::c_int {
                return -(5 as libc::c_int);
            }
        }
    } else if *cp as libc::c_int == '\0' as i32 && strlen(str) > 3 as libc::c_int as libc::c_ulong {
        min = hr % 100 as libc::c_int;
        hr = hr / 100 as libc::c_int;
    } else {
        min = 0 as libc::c_int;
    }
    if hr < 0 as libc::c_int || hr > 15 as libc::c_int {
        return -(5 as libc::c_int);
    }
    if min < 0 as libc::c_int || min >= 60 as libc::c_int {
        return -(5 as libc::c_int);
    }
    if sec < 0 as libc::c_int || sec >= 60 as libc::c_int {
        return -(5 as libc::c_int);
    }
    tz = (hr * 60 as libc::c_int + min) * 60 as libc::c_int + sec;
    if *str as libc::c_int == '-' as i32 {
        tz = -tz;
    }
    *tzp = -tz;
    if *cp as libc::c_int != '\0' as i32 {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}

unsafe fn DecodeTimezoneAbbrev(
    field: libc::c_int,
    lowtoken: *mut libc::c_char,
    offset: *mut libc::c_int,
    tz: *mut *mut pg_tz,
) -> RealFieldType {
    let mut tp = abbrevcache[field as usize];
    if tp.is_null()
        || strncmp(
            lowtoken,
            ((*tp).token).as_ptr(),
            10 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        if !zoneabbrevtbl.is_null() {
            tp = datebsearch(
                lowtoken,
                ((*zoneabbrevtbl).abbrevs).as_mut_ptr(),
                (*zoneabbrevtbl).numabbrevs,
            );
        } else {
            tp = 0 as *const datetkn;
        }
    }
    if tp.is_null() {
        *offset = 0 as libc::c_int;
        *tz = 0 as *mut pg_tz;
        RealFieldType::UnknownField
    } else {
        abbrevcache[field as usize] = tp;
        match (*tp).type_0 {
            RealFieldType::DynTz => {
                *offset = 0 as libc::c_int;
                *tz = FetchDynamicTimeZone(zoneabbrevtbl, tp);
            }
            _ => {
                *offset = (*tp).value;
                *tz = 0 as *mut pg_tz;
            }
        }
        (*tp).type_0
    }
}

unsafe fn DecodeSpecial(
    field: libc::c_int,
    lowtoken: *mut libc::c_char,
    val: *mut libc::c_int,
) -> RealFieldType {
    let mut tp = datecache[field as usize];
    if tp.is_null()
        || strncmp(
            lowtoken,
            ((*tp).token).as_ptr(),
            10 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        tp = datebsearch(lowtoken, datetktbl.as_ptr(), szdatetktbl);
    }
    if tp.is_null() {
        *val = 0 as libc::c_int;
        RealFieldType::UnknownField
    } else {
        datecache[field as usize] = tp;
        *val = (*tp).value;
        (*tp).type_0
    }
}

unsafe fn datebsearch(
    key: *const libc::c_char,
    mut base: *const datetkn,
    nel: libc::c_int,
) -> *const datetkn {
    if nel > 0 as libc::c_int {
        let mut last: *const datetkn = base
            .offset(nel as isize)
            .offset(-(1 as libc::c_int as isize));
        let mut position: *const datetkn;
        let mut result;
        while last >= base {
            position =
                base.offset((last.offset_from(base) as libc::c_long >> 1 as libc::c_int) as isize);
            result = *key.offset(0 as libc::c_int as isize) as libc::c_int
                - (*position).token[0 as libc::c_int as usize] as libc::c_int;
            if result == 0 as libc::c_int {
                result = strncmp(
                    key,
                    ((*position).token).as_ptr(),
                    10 as libc::c_int as libc::c_ulong,
                );
                if result == 0 as libc::c_int {
                    return position;
                }
            }
            if result < 0 as libc::c_int {
                last = position.offset(-(1 as libc::c_int as isize));
            } else {
                base = position.offset(1 as libc::c_int as isize);
            }
        }
    }
    return 0 as *const datetkn;
}

unsafe fn FetchDynamicTimeZone(tbl: *mut TimeZoneAbbrevTable, tp: *const datetkn) -> *mut pg_tz {
    let dtza = (tbl as *mut libc::c_char).offset((*tp).value as isize) as *mut DynamicZoneAbbrev;
    if ((*dtza).tz).is_null() {
        (*dtza).tz = pg_tzset(((*dtza).zone).as_mut_ptr());
        if ((*dtza).tz).is_null() {
            let mut __errno_location_0: libc::c_int = 0;
            if if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
                errstart_cold(21 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
            } else {
                errstart(21 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
            } != 0
            {
                errcode(
                    ('F' as i32 - '0' as i32 & 0x3f as libc::c_int)
                        + (('0' as i32 - '0' as i32 & 0x3f as libc::c_int) << 6 as libc::c_int)
                        + (('0' as i32 - '0' as i32 & 0x3f as libc::c_int) << 12 as libc::c_int)
                        + (('0' as i32 - '0' as i32 & 0x3f as libc::c_int) << 18 as libc::c_int)
                        + (('0' as i32 - '0' as i32 & 0x3f as libc::c_int) << 24 as libc::c_int),
                );
                errmsg(
                    b"time zone \"%s\" not recognized\0" as *const u8 as *const libc::c_char,
                    ((*dtza).zone).as_mut_ptr() as *mut _,
                );
                errdetail(
                    b"This time zone name appears in the configuration file for time zone abbreviation \"%s\".\0"
                        as *const u8 as *const libc::c_char,
                    ((*tp).token).as_ptr() as *mut _,
                );
                errfinish(
                    b"/home/petrosagg/projects/postgres-datetime/src/datetime.c\0" as *const u8
                        as *const libc::c_char,
                    4647 as libc::c_int,
                    (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                        b"FetchDynamicTimeZone\0",
                    ))
                    .as_ptr(),
                );
            }
            if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
                unreachable!();
            }
        }
    }
    return (*dtza).tz;
}

unsafe fn run_static_initializers() {
    szdatetktbl = (::core::mem::size_of::<[datetkn; 71]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<datetkn>() as libc::c_ulong)
        as libc::c_int;
    szdeltatktbl = (::core::mem::size_of::<[datetkn; 61]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<datetkn>() as libc::c_ulong)
        as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [run_static_initializers];
