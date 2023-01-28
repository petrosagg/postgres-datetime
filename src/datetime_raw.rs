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
fn errmsg2(
    fmt: *const libc::c_char,
    _arg1: *mut libc::c_void,
    _arg2: *mut libc::c_void,
) -> libc::c_int {
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
    type pg_tzenum;
    fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn errhint(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn pg_sprintf(str: *mut libc::c_char, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn palloc(size: Size) -> *mut libc::c_void;
    static mut CurrentMemoryContext: MemoryContext;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn rint(_: libc::c_double) -> libc::c_double;
    fn CreateTemplateTupleDesc(natts: libc::c_int) -> TupleDesc;
    fn TupleDescInitEntry(
        desc: TupleDesc,
        attributeNumber: AttrNumber,
        attributeName: *const libc::c_char,
        oidtypeid: Oid,
        typmod: int32,
        attdim: libc::c_int,
    );
    fn heap_form_tuple(
        tupleDescriptor: TupleDesc,
        values: *mut Datum,
        isnull: &mut [bool],
    ) -> HeapTuple;
    fn tuplestore_begin_heap(
        randomAccess: bool,
        interXact: bool,
        maxKBytes: libc::c_int,
    ) -> *mut Tuplestorestate;
    fn tuplestore_putvalues(
        state: *mut Tuplestorestate,
        tdesc: TupleDesc,
        values: *mut Datum,
        isnull: &mut [bool],
    );
    fn end_MultiFuncCall(fcinfo: FunctionCallInfo, funcctx: *mut FuncCallContext);
    fn per_MultiFuncCall(fcinfo: FunctionCallInfo) -> *mut FuncCallContext;
    fn init_MultiFuncCall(fcinfo: FunctionCallInfo) -> *mut FuncCallContext;
    fn HeapTupleHeaderGetDatum(tuple: HeapTupleHeader) -> Datum;
    fn BlessTupleDesc(tupdesc: TupleDesc) -> TupleDesc;
    fn get_call_result_type(
        fcinfo: FunctionCallInfo,
        resultTypeId: *mut Oid,
        resultTupleDesc: *mut TupleDesc,
    ) -> TypeFuncClass;
    static mut IntervalStyle: libc::c_int;
    static mut work_mem: libc::c_int;
    fn pg_get_timezone_offset(tz: *const pg_tz, gmtoff: *mut libc::c_long) -> bool;
    fn pg_get_timezone_name(tz: *mut pg_tz) -> *const libc::c_char;
    fn pg_tzenumerate_start() -> *mut pg_tzenum;
    fn pg_tzenumerate_next(dir: *mut pg_tzenum) -> *mut pg_tz;
    fn pg_tzenumerate_end(dir: *mut pg_tzenum);
    fn exprTypmod(expr: *const Node) -> int32;
    fn relabel_to_typmod(expr: *mut Node, typmod: int32) -> *mut Node;
    fn pg_ultostr_zeropad(
        str: *mut libc::c_char,
        value: uint32,
        minwidth: int32,
    ) -> *mut libc::c_char;
    fn pg_ultostr(str: *mut libc::c_char, value: uint32) -> *mut libc::c_char;
    fn cstring_to_text(s: *const libc::c_char) -> *mut text;
    fn timestamptz_to_time_t(t: TimestampTz) -> pg_time_t;
    fn tm2interval(tm: *mut pg_tm, fsec: fsec_t, span: *mut Interval) -> libc::c_int;
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
type text = varlena;
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
const T_SupportRequestIndexCondition: NodeTag = 430;
const T_SupportRequestRows: NodeTag = 429;
const T_SupportRequestCost: NodeTag = 428;
const T_SupportRequestSelectivity: NodeTag = 427;
const T_SupportRequestSimplify: NodeTag = 426;
const T_CallContext: NodeTag = 425;
const T_ForeignKeyCacheInfo: NodeTag = 424;
const T_TsmRoutine: NodeTag = 423;
const T_TableAmRoutine: NodeTag = 422;
const T_IndexAmRoutine: NodeTag = 421;
const T_FdwRoutine: NodeTag = 420;
const T_InlineCodeBlock: NodeTag = 419;
const T_TIDBitmap: NodeTag = 418;
const T_WindowObjectData: NodeTag = 417;
const T_ReturnSetInfo: NodeTag = 416;
const T_EventTriggerData: NodeTag = 415;
const T_TriggerData: NodeTag = 414;
const T_TimeLineHistoryCmd: NodeTag = 413;
const T_StartReplicationCmd: NodeTag = 412;
const T_ReadReplicationSlotCmd: NodeTag = 411;
const T_DropReplicationSlotCmd: NodeTag = 410;
const T_CreateReplicationSlotCmd: NodeTag = 409;
const T_BaseBackupCmd: NodeTag = 408;
const T_IdentifySystemCmd: NodeTag = 407;
const T_PublicationTable: NodeTag = 406;
const T_PublicationObjSpec: NodeTag = 405;
const T_VacuumRelation: NodeTag = 404;
const T_PartitionCmd: NodeTag = 403;
const T_PartitionRangeDatum: NodeTag = 402;
const T_PartitionBoundSpec: NodeTag = 401;
const T_PartitionSpec: NodeTag = 400;
const T_PartitionElem: NodeTag = 399;
const T_TriggerTransition: NodeTag = 398;
const T_RoleSpec: NodeTag = 397;
const T_CommonTableExpr: NodeTag = 396;
const T_CTECycleClause: NodeTag = 395;
const T_CTESearchClause: NodeTag = 394;
const T_OnConflictClause: NodeTag = 393;
const T_InferClause: NodeTag = 392;
const T_WithClause: NodeTag = 391;
const T_XmlSerialize: NodeTag = 390;
const T_RowMarkClause: NodeTag = 389;
const T_LockingClause: NodeTag = 388;
const T_FunctionParameter: NodeTag = 387;
const T_TableLikeClause: NodeTag = 386;
const T_CreateOpClassItem: NodeTag = 385;
const T_AccessPriv: NodeTag = 384;
const T_ObjectWithArgs: NodeTag = 383;
const T_WindowClause: NodeTag = 382;
const T_GroupingSet: NodeTag = 381;
const T_SortGroupClause: NodeTag = 380;
const T_WithCheckOption: NodeTag = 379;
const T_TableSampleClause: NodeTag = 378;
const T_RangeTblFunction: NodeTag = 377;
const T_RangeTblEntry: NodeTag = 376;
const T_DefElem: NodeTag = 375;
const T_Constraint: NodeTag = 374;
const T_StatsElem: NodeTag = 373;
const T_IndexElem: NodeTag = 372;
const T_ColumnDef: NodeTag = 371;
const T_TypeName: NodeTag = 370;
const T_RangeTableFuncCol: NodeTag = 369;
const T_RangeTableFunc: NodeTag = 368;
const T_RangeTableSample: NodeTag = 367;
const T_RangeFunction: NodeTag = 366;
const T_RangeSubselect: NodeTag = 365;
const T_WindowDef: NodeTag = 364;
const T_SortBy: NodeTag = 363;
const T_CollateClause: NodeTag = 362;
const T_TypeCast: NodeTag = 361;
const T_MultiAssignRef: NodeTag = 360;
const T_ResTarget: NodeTag = 359;
const T_A_ArrayExpr: NodeTag = 358;
const T_A_Indirection: NodeTag = 357;
const T_A_Indices: NodeTag = 356;
const T_A_Star: NodeTag = 355;
const T_FuncCall: NodeTag = 354;
const T_A_Const: NodeTag = 353;
const T_ParamRef: NodeTag = 352;
const T_ColumnRef: NodeTag = 351;
const T_A_Expr: NodeTag = 350;
const T_AlterStatsStmt: NodeTag = 349;
const T_CallStmt: NodeTag = 348;
const T_AlterCollationStmt: NodeTag = 347;
const T_CreateStatsStmt: NodeTag = 346;
const T_DropSubscriptionStmt: NodeTag = 345;
const T_AlterSubscriptionStmt: NodeTag = 344;
const T_CreateSubscriptionStmt: NodeTag = 343;
const T_AlterPublicationStmt: NodeTag = 342;
const T_CreatePublicationStmt: NodeTag = 341;
const T_CreateAmStmt: NodeTag = 340;
const T_CreateTransformStmt: NodeTag = 339;
const T_AlterPolicyStmt: NodeTag = 338;
const T_CreatePolicyStmt: NodeTag = 337;
const T_AlterSystemStmt: NodeTag = 336;
const T_ReplicaIdentityStmt: NodeTag = 335;
const T_RefreshMatViewStmt: NodeTag = 334;
const T_AlterEventTrigStmt: NodeTag = 333;
const T_CreateEventTrigStmt: NodeTag = 332;
const T_AlterExtensionContentsStmt: NodeTag = 331;
const T_AlterExtensionStmt: NodeTag = 330;
const T_CreateExtensionStmt: NodeTag = 329;
const T_ImportForeignSchemaStmt: NodeTag = 328;
const T_CreateForeignTableStmt: NodeTag = 327;
const T_SecLabelStmt: NodeTag = 326;
const T_AlterTableMoveAllStmt: NodeTag = 325;
const T_AlterTableSpaceOptionsStmt: NodeTag = 324;
const T_DropUserMappingStmt: NodeTag = 323;
const T_AlterUserMappingStmt: NodeTag = 322;
const T_CreateUserMappingStmt: NodeTag = 321;
const T_AlterForeignServerStmt: NodeTag = 320;
const T_CreateForeignServerStmt: NodeTag = 319;
const T_AlterFdwStmt: NodeTag = 318;
const T_CreateFdwStmt: NodeTag = 317;
const T_AlterTSConfigurationStmt: NodeTag = 316;
const T_AlterTSDictionaryStmt: NodeTag = 315;
const T_AlterEnumStmt: NodeTag = 314;
const T_CreateRangeStmt: NodeTag = 313;
const T_CreateEnumStmt: NodeTag = 312;
const T_CompositeTypeStmt: NodeTag = 311;
const T_ReassignOwnedStmt: NodeTag = 310;
const T_DropOwnedStmt: NodeTag = 309;
const T_AlterTypeStmt: NodeTag = 308;
const T_AlterOperatorStmt: NodeTag = 307;
const T_AlterOwnerStmt: NodeTag = 306;
const T_AlterObjectSchemaStmt: NodeTag = 305;
const T_AlterObjectDependsStmt: NodeTag = 304;
const T_DropTableSpaceStmt: NodeTag = 303;
const T_CreateTableSpaceStmt: NodeTag = 302;
const T_DeclareCursorStmt: NodeTag = 301;
const T_DeallocateStmt: NodeTag = 300;
const T_ExecuteStmt: NodeTag = 299;
const T_PrepareStmt: NodeTag = 298;
const T_AlterOpFamilyStmt: NodeTag = 297;
const T_CreateOpFamilyStmt: NodeTag = 296;
const T_CreateOpClassStmt: NodeTag = 295;
const T_CreateCastStmt: NodeTag = 294;
const T_CreateConversionStmt: NodeTag = 293;
const T_AlterRoleSetStmt: NodeTag = 292;
const T_AlterDatabaseSetStmt: NodeTag = 291;
const T_AlterDatabaseStmt: NodeTag = 290;
const T_CreateSchemaStmt: NodeTag = 289;
const T_CheckPointStmt: NodeTag = 288;
const T_ReindexStmt: NodeTag = 287;
const T_ConstraintsSetStmt: NodeTag = 286;
const T_LockStmt: NodeTag = 285;
const T_DropRoleStmt: NodeTag = 284;
const T_AlterRoleStmt: NodeTag = 283;
const T_CreateRoleStmt: NodeTag = 282;
const T_CreatePLangStmt: NodeTag = 281;
const T_CreateTrigStmt: NodeTag = 280;
const T_DiscardStmt: NodeTag = 279;
const T_VariableShowStmt: NodeTag = 278;
const T_VariableSetStmt: NodeTag = 277;
const T_AlterSeqStmt: NodeTag = 276;
const T_CreateSeqStmt: NodeTag = 275;
const T_CreateTableAsStmt: NodeTag = 274;
const T_ExplainStmt: NodeTag = 273;
const T_VacuumStmt: NodeTag = 272;
const T_DropdbStmt: NodeTag = 271;
const T_CreatedbStmt: NodeTag = 270;
const T_CreateDomainStmt: NodeTag = 269;
const T_LoadStmt: NodeTag = 268;
const T_ViewStmt: NodeTag = 267;
const T_TransactionStmt: NodeTag = 266;
const T_UnlistenStmt: NodeTag = 265;
const T_ListenStmt: NodeTag = 264;
const T_NotifyStmt: NodeTag = 263;
const T_RuleStmt: NodeTag = 262;
const T_RenameStmt: NodeTag = 261;
const T_DoStmt: NodeTag = 260;
const T_AlterFunctionStmt: NodeTag = 259;
const T_CreateFunctionStmt: NodeTag = 258;
const T_IndexStmt: NodeTag = 257;
const T_FetchStmt: NodeTag = 256;
const T_CommentStmt: NodeTag = 255;
const T_TruncateStmt: NodeTag = 254;
const T_DropStmt: NodeTag = 253;
const T_DefineStmt: NodeTag = 252;
const T_CreateStmt: NodeTag = 251;
const T_CopyStmt: NodeTag = 250;
const T_ClusterStmt: NodeTag = 249;
const T_ClosePortalStmt: NodeTag = 248;
const T_AlterDefaultPrivilegesStmt: NodeTag = 247;
const T_GrantRoleStmt: NodeTag = 246;
const T_GrantStmt: NodeTag = 245;
const T_SetOperationStmt: NodeTag = 244;
const T_AlterDomainStmt: NodeTag = 243;
const T_AlterTableCmd: NodeTag = 242;
const T_AlterTableStmt: NodeTag = 241;
const T_PLAssignStmt: NodeTag = 240;
const T_ReturnStmt: NodeTag = 239;
const T_SelectStmt: NodeTag = 238;
const T_UpdateStmt: NodeTag = 237;
const T_DeleteStmt: NodeTag = 236;
const T_InsertStmt: NodeTag = 235;
const T_PlannedStmt: NodeTag = 234;
const T_Query: NodeTag = 233;
const T_RawStmt: NodeTag = 232;
const T_ExtensibleNode: NodeTag = 231;
const T_OidList: NodeTag = 230;
const T_IntList: NodeTag = 229;
const T_List: NodeTag = 228;
const T_BitString: NodeTag = 227;
const T_String: NodeTag = 226;
const T_Boolean: NodeTag = 225;
const T_Float: NodeTag = 224;
const T_Integer: NodeTag = 223;
const T_GenerationContext: NodeTag = 222;
const T_SlabContext: NodeTag = 221;
const T_AllocSetContext: NodeTag = 220;
const T_StatisticExtInfo: NodeTag = 219;
const T_GroupingSetData: NodeTag = 218;
const T_RollupData: NodeTag = 217;
const T_PlannerParamItem: NodeTag = 216;
const T_MinMaxAggInfo: NodeTag = 215;
const T_PlaceHolderInfo: NodeTag = 214;
const T_RowIdentityVarInfo: NodeTag = 213;
const T_AppendRelInfo: NodeTag = 212;
const T_SpecialJoinInfo: NodeTag = 211;
const T_PlaceHolderVar: NodeTag = 210;
const T_IndexClause: NodeTag = 209;
const T_RestrictInfo: NodeTag = 208;
const T_PathTarget: NodeTag = 207;
const T_PathKey: NodeTag = 206;
const T_EquivalenceMember: NodeTag = 205;
const T_EquivalenceClass: NodeTag = 204;
const T_LimitPath: NodeTag = 203;
const T_ModifyTablePath: NodeTag = 202;
const T_LockRowsPath: NodeTag = 201;
const T_RecursiveUnionPath: NodeTag = 200;
const T_SetOpPath: NodeTag = 199;
const T_WindowAggPath: NodeTag = 198;
const T_MinMaxAggPath: NodeTag = 197;
const T_GroupingSetsPath: NodeTag = 196;
const T_AggPath: NodeTag = 195;
const T_UpperUniquePath: NodeTag = 194;
const T_GroupPath: NodeTag = 193;
const T_IncrementalSortPath: NodeTag = 192;
const T_SortPath: NodeTag = 191;
const T_ProjectSetPath: NodeTag = 190;
const T_ProjectionPath: NodeTag = 189;
const T_GatherMergePath: NodeTag = 188;
const T_GatherPath: NodeTag = 187;
const T_UniquePath: NodeTag = 186;
const T_MemoizePath: NodeTag = 185;
const T_MaterialPath: NodeTag = 184;
const T_GroupResultPath: NodeTag = 183;
const T_MergeAppendPath: NodeTag = 182;
const T_AppendPath: NodeTag = 181;
const T_HashPath: NodeTag = 180;
const T_MergePath: NodeTag = 179;
const T_NestPath: NodeTag = 178;
const T_CustomPath: NodeTag = 177;
const T_ForeignPath: NodeTag = 176;
const T_SubqueryScanPath: NodeTag = 175;
const T_TidRangePath: NodeTag = 174;
const T_TidPath: NodeTag = 173;
const T_BitmapOrPath: NodeTag = 172;
const T_BitmapAndPath: NodeTag = 171;
const T_BitmapHeapPath: NodeTag = 170;
const T_IndexPath: NodeTag = 169;
const T_Path: NodeTag = 168;
const T_ParamPathInfo: NodeTag = 167;
const T_ForeignKeyOptInfo: NodeTag = 166;
const T_IndexOptInfo: NodeTag = 165;
const T_RelOptInfo: NodeTag = 164;
const T_PlannerGlobal: NodeTag = 163;
const T_PlannerInfo: NodeTag = 162;
const T_DomainConstraintState: NodeTag = 161;
const T_SubPlanState: NodeTag = 160;
const T_SetExprState: NodeTag = 159;
const T_WindowFuncExprState: NodeTag = 158;
const T_ExprState: NodeTag = 157;
const T_IntoClause: NodeTag = 156;
const T_OnConflictExpr: NodeTag = 155;
const T_FromExpr: NodeTag = 154;
const T_JoinExpr: NodeTag = 153;
const T_RangeTblRef: NodeTag = 152;
const T_TargetEntry: NodeTag = 151;
const T_InferenceElem: NodeTag = 150;
const T_NextValueExpr: NodeTag = 149;
const T_CurrentOfExpr: NodeTag = 148;
const T_SetToDefault: NodeTag = 147;
const T_CoerceToDomainValue: NodeTag = 146;
const T_CoerceToDomain: NodeTag = 145;
const T_BooleanTest: NodeTag = 144;
const T_NullTest: NodeTag = 143;
const T_XmlExpr: NodeTag = 142;
const T_SQLValueFunction: NodeTag = 141;
const T_MinMaxExpr: NodeTag = 140;
const T_CoalesceExpr: NodeTag = 139;
const T_RowCompareExpr: NodeTag = 138;
const T_RowExpr: NodeTag = 137;
const T_ArrayExpr: NodeTag = 136;
const T_CaseTestExpr: NodeTag = 135;
const T_CaseWhen: NodeTag = 134;
const T_CaseExpr: NodeTag = 133;
const T_CollateExpr: NodeTag = 132;
const T_ConvertRowtypeExpr: NodeTag = 131;
const T_ArrayCoerceExpr: NodeTag = 130;
const T_CoerceViaIO: NodeTag = 129;
const T_RelabelType: NodeTag = 128;
const T_FieldStore: NodeTag = 127;
const T_FieldSelect: NodeTag = 126;
const T_AlternativeSubPlan: NodeTag = 125;
const T_SubPlan: NodeTag = 124;
const T_SubLink: NodeTag = 123;
const T_BoolExpr: NodeTag = 122;
const T_ScalarArrayOpExpr: NodeTag = 121;
const T_NullIfExpr: NodeTag = 120;
const T_DistinctExpr: NodeTag = 119;
const T_OpExpr: NodeTag = 118;
const T_NamedArgExpr: NodeTag = 117;
const T_FuncExpr: NodeTag = 116;
const T_SubscriptingRef: NodeTag = 115;
const T_WindowFunc: NodeTag = 114;
const T_GroupingFunc: NodeTag = 113;
const T_Aggref: NodeTag = 112;
const T_Param: NodeTag = 111;
const T_Const: NodeTag = 110;
const T_Var: NodeTag = 109;
const T_TableFunc: NodeTag = 108;
const T_RangeVar: NodeTag = 107;
const T_Alias: NodeTag = 106;
const T_LimitState: NodeTag = 105;
const T_LockRowsState: NodeTag = 104;
const T_SetOpState: NodeTag = 103;
const T_HashState: NodeTag = 102;
const T_GatherMergeState: NodeTag = 101;
const T_GatherState: NodeTag = 100;
const T_UniqueState: NodeTag = 99;
const T_WindowAggState: NodeTag = 98;
const T_AggState: NodeTag = 97;
const T_GroupState: NodeTag = 96;
const T_IncrementalSortState: NodeTag = 95;
const T_SortState: NodeTag = 94;
const T_MemoizeState: NodeTag = 93;
const T_MaterialState: NodeTag = 92;
const T_HashJoinState: NodeTag = 91;
const T_MergeJoinState: NodeTag = 90;
const T_NestLoopState: NodeTag = 89;
const T_JoinState: NodeTag = 88;
const T_CustomScanState: NodeTag = 87;
const T_ForeignScanState: NodeTag = 86;
const T_WorkTableScanState: NodeTag = 85;
const T_NamedTuplestoreScanState: NodeTag = 84;
const T_CteScanState: NodeTag = 83;
const T_ValuesScanState: NodeTag = 82;
const T_TableFuncScanState: NodeTag = 81;
const T_FunctionScanState: NodeTag = 80;
const T_SubqueryScanState: NodeTag = 79;
const T_TidRangeScanState: NodeTag = 78;
const T_TidScanState: NodeTag = 77;
const T_BitmapHeapScanState: NodeTag = 76;
const T_BitmapIndexScanState: NodeTag = 75;
const T_IndexOnlyScanState: NodeTag = 74;
const T_IndexScanState: NodeTag = 73;
const T_SampleScanState: NodeTag = 72;
const T_SeqScanState: NodeTag = 71;
const T_ScanState: NodeTag = 70;
const T_BitmapOrState: NodeTag = 69;
const T_BitmapAndState: NodeTag = 68;
const T_RecursiveUnionState: NodeTag = 67;
const T_MergeAppendState: NodeTag = 66;
const T_AppendState: NodeTag = 65;
const T_ModifyTableState: NodeTag = 64;
const T_ProjectSetState: NodeTag = 63;
const T_ResultState: NodeTag = 62;
const T_PlanState: NodeTag = 61;
const T_PlanInvalItem: NodeTag = 60;
const T_PartitionPruneStepCombine: NodeTag = 59;
const T_PartitionPruneStepOp: NodeTag = 58;
const T_PartitionedRelPruneInfo: NodeTag = 57;
const T_PartitionPruneInfo: NodeTag = 56;
const T_PlanRowMark: NodeTag = 55;
const T_NestLoopParam: NodeTag = 54;
const T_Limit: NodeTag = 53;
const T_LockRows: NodeTag = 52;
const T_SetOp: NodeTag = 51;
const T_Hash: NodeTag = 50;
const T_GatherMerge: NodeTag = 49;
const T_Gather: NodeTag = 48;
const T_Unique: NodeTag = 47;
const T_WindowAgg: NodeTag = 46;
const T_Agg: NodeTag = 45;
const T_Group: NodeTag = 44;
const T_IncrementalSort: NodeTag = 43;
const T_Sort: NodeTag = 42;
const T_Memoize: NodeTag = 41;
const T_Material: NodeTag = 40;
const T_HashJoin: NodeTag = 39;
const T_MergeJoin: NodeTag = 38;
const T_NestLoop: NodeTag = 37;
const T_Join: NodeTag = 36;
const T_CustomScan: NodeTag = 35;
const T_ForeignScan: NodeTag = 34;
const T_WorkTableScan: NodeTag = 33;
const T_NamedTuplestoreScan: NodeTag = 32;
const T_CteScan: NodeTag = 31;
const T_TableFuncScan: NodeTag = 30;
const T_ValuesScan: NodeTag = 29;
const T_FunctionScan: NodeTag = 28;
const T_SubqueryScan: NodeTag = 27;
const T_TidRangeScan: NodeTag = 26;
const T_TidScan: NodeTag = 25;
const T_BitmapHeapScan: NodeTag = 24;
const T_BitmapIndexScan: NodeTag = 23;
const T_IndexOnlyScan: NodeTag = 22;
const T_IndexScan: NodeTag = 21;
const T_SampleScan: NodeTag = 20;
const T_SeqScan: NodeTag = 19;
const T_Scan: NodeTag = 18;
const T_BitmapOr: NodeTag = 17;
const T_BitmapAnd: NodeTag = 16;
const T_RecursiveUnion: NodeTag = 15;
const T_MergeAppend: NodeTag = 14;
const T_Append: NodeTag = 13;
const T_ModifyTable: NodeTag = 12;
const T_ProjectSet: NodeTag = 11;
const T_Result: NodeTag = 10;
const T_Plan: NodeTag = 9;
const T_TupleTableSlot: NodeTag = 8;
const T_EState: NodeTag = 7;
const T_ResultRelInfo: NodeTag = 6;
const T_OnConflictSetState: NodeTag = 5;
const T_JunkFilter: NodeTag = 4;
const T_ProjectionInfo: NodeTag = 3;
const T_ExprContext: NodeTag = 2;
const T_IndexInfo: NodeTag = 1;
const T_Invalid: NodeTag = 0;
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
struct ItemPointerData(pub ItemPointerData_Inner);
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
const CMD_NOTHING: CmdType = 6;
const CMD_UTILITY: CmdType = 5;
const CMD_DELETE: CmdType = 4;
const CMD_INSERT: CmdType = 3;
const CMD_UPDATE: CmdType = 2;
const CMD_SELECT: CmdType = 1;
const CMD_UNKNOWN: CmdType = 0;
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
const LCS_FORUPDATE: LockClauseStrength = 4;
const LCS_FORNOKEYUPDATE: LockClauseStrength = 3;
const LCS_FORSHARE: LockClauseStrength = 2;
const LCS_FORKEYSHARE: LockClauseStrength = 1;
const LCS_NONE: LockClauseStrength = 0;
type LockWaitPolicy = libc::c_uint;
const LockWaitError: LockWaitPolicy = 2;
const LockWaitSkip: LockWaitPolicy = 1;
const LockWaitBlock: LockWaitPolicy = 0;
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
const PARAM_MULTIEXPR: ParamKind = 3;
const PARAM_SUBLINK: ParamKind = 2;
const PARAM_EXEC: ParamKind = 1;
const PARAM_EXTERN: ParamKind = 0;
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
const COERCE_SQL_SYNTAX: CoercionForm = 3;
const COERCE_IMPLICIT_CAST: CoercionForm = 2;
const COERCE_EXPLICIT_CAST: CoercionForm = 1;
const COERCE_EXPLICIT_CALL: CoercionForm = 0;
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
const ROW_MARK_COPY: RowMarkType = 5;
const ROW_MARK_REFERENCE: RowMarkType = 4;
const ROW_MARK_KEYSHARE: RowMarkType = 3;
const ROW_MARK_SHARE: RowMarkType = 2;
const ROW_MARK_NOKEYEXCLUSIVE: RowMarkType = 1;
const ROW_MARK_EXCLUSIVE: RowMarkType = 0;
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
const SNAPSHOT_NON_VACUUMABLE: SnapshotType = 6;
const SNAPSHOT_HISTORIC_MVCC: SnapshotType = 5;
const SNAPSHOT_DIRTY: SnapshotType = 4;
const SNAPSHOT_TOAST: SnapshotType = 3;
const SNAPSHOT_ANY: SnapshotType = 2;
const SNAPSHOT_SELF: SnapshotType = 1;
const SNAPSHOT_MVCC: SnapshotType = 0;
type ScanDirection = libc::c_int;
const ForwardScanDirection: ScanDirection = 1;
const NoMovementScanDirection: ScanDirection = 0;
const BackwardScanDirection: ScanDirection = -1;
type ExecProcNodeMtd = Option<unsafe fn(*mut PlanState) -> *mut TupleTableSlot>;
type ExprStateEvalFunc = Option<unsafe fn(*mut ExprState, *mut ExprContext, &mut bool) -> Datum>;
type ExprDoneCond = libc::c_uint;
const ExprEndResult: ExprDoneCond = 2;
const ExprMultipleResult: ExprDoneCond = 1;
const ExprSingleResult: ExprDoneCond = 0;
type SetFunctionReturnMode = libc::c_uint;
const SFRM_Materialize_Preferred: SetFunctionReturnMode = 8;
const SFRM_Materialize_Random: SetFunctionReturnMode = 4;
const SFRM_Materialize: SetFunctionReturnMode = 2;
const SFRM_ValuePerCall: SetFunctionReturnMode = 1;
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
type TypeFuncClass = libc::c_uint;
const TYPEFUNC_OTHER: TypeFuncClass = 4;
const TYPEFUNC_RECORD: TypeFuncClass = 3;
const TYPEFUNC_COMPOSITE_DOMAIN: TypeFuncClass = 2;
const TYPEFUNC_COMPOSITE: TypeFuncClass = 1;
const TYPEFUNC_SCALAR: TypeFuncClass = 0;
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
#[inline]
unsafe fn MemoryContextSwitchTo(context: MemoryContext) -> MemoryContext {
    let old: MemoryContext = CurrentMemoryContext;
    CurrentMemoryContext = context;
    return old;
}
#[inline]
unsafe fn list_nth_cell(list: *const List, n: libc::c_int) -> *mut ListCell {
    return &mut *((*list).elements).offset(n as isize) as *mut ListCell;
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

static mut months: [*const libc::c_char; 13] = [
    b"Jan\0" as *const u8 as *const libc::c_char,
    b"Feb\0" as *const u8 as *const libc::c_char,
    b"Mar\0" as *const u8 as *const libc::c_char,
    b"Apr\0" as *const u8 as *const libc::c_char,
    b"May\0" as *const u8 as *const libc::c_char,
    b"Jun\0" as *const u8 as *const libc::c_char,
    b"Jul\0" as *const u8 as *const libc::c_char,
    b"Aug\0" as *const u8 as *const libc::c_char,
    b"Sep\0" as *const u8 as *const libc::c_char,
    b"Oct\0" as *const u8 as *const libc::c_char,
    b"Nov\0" as *const u8 as *const libc::c_char,
    b"Dec\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];

static mut days: [*const libc::c_char; 8] = [
    b"Sunday\0" as *const u8 as *const libc::c_char,
    b"Monday\0" as *const u8 as *const libc::c_char,
    b"Tuesday\0" as *const u8 as *const libc::c_char,
    b"Wednesday\0" as *const u8 as *const libc::c_char,
    b"Thursday\0" as *const u8 as *const libc::c_char,
    b"Friday\0" as *const u8 as *const libc::c_char,
    b"Saturday\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
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
static mut deltatktbl: [datetkn; 61] = unsafe {
    [
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"@\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::IgnoreDtf,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"ago\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Ago,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"c\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 27 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"cent\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 27 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"centuries\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 27 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"century\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 27 as libc::c_int,
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
                    b"day\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 21 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"days\0\0\0\0\0\0\0",
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
                type_0: RealFieldType::Units,
                value: 26 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"decade\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 26 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"decades\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 26 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"decs\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 26 as libc::c_int,
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
                    b"hour\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 20 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"hours\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 20 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"hr\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 20 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"hrs\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 20 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"m\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 19 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"microsecon\0",
                ),
                type_0: RealFieldType::Units,
                value: 30 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mil\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 28 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"millennia\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 28 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"millennium\0",
                ),
                type_0: RealFieldType::Units,
                value: 28 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"millisecon\0",
                ),
                type_0: RealFieldType::Units,
                value: 29 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mils\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 28 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"min\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 19 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mins\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 19 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"minute\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 19 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"minutes\0\0\0\0",
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
                type_0: RealFieldType::Units,
                value: 23 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mons\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 23 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"month\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 23 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"months\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 23 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"ms\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 29 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"msec\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 29 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"msecond\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 29 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mseconds\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 29 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"msecs\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 29 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"qtr\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 24 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"quarter\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 24 as libc::c_int,
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
                    b"sec\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 18 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"second\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 18 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"seconds\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 18 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"secs\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 18 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"timezone\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 4 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"timezone_h\0",
                ),
                type_0: RealFieldType::Units,
                value: 34 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"timezone_m\0",
                ),
                type_0: RealFieldType::Units,
                value: 35 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"us\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 30 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"usec\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 30 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"usecond\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 30 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"useconds\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 30 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"usecs\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 30 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"w\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 22 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"week\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 22 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"weeks\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 22 as libc::c_int,
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
                    b"year\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 25 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"years\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 25 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"yr\0\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 25 as libc::c_int,
            };
            init
        },
        {
            let init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"yrs\0\0\0\0\0\0\0\0",
                ),
                type_0: RealFieldType::Units,
                value: 25 as libc::c_int,
            };
            init
        },
    ]
};
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
static mut deltacache: [*const datetkn; 25] = [
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

unsafe fn j2day(mut date: libc::c_int) -> libc::c_int {
    date += 1 as libc::c_int;
    date %= 7 as libc::c_int;
    if date < 0 as libc::c_int {
        date += 7 as libc::c_int;
    }
    return date;
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
unsafe fn AppendSeconds(
    mut cp: *mut libc::c_char,
    sec: libc::c_int,
    fsec: fsec_t,
    mut precision: libc::c_int,
    fillzeros: bool,
) -> *mut libc::c_char {
    if fillzeros {
        cp = pg_ultostr_zeropad(
            cp,
            (if sec >= 0 as libc::c_int { sec } else { -sec }) as uint32,
            2 as libc::c_int,
        );
    } else {
        cp = pg_ultostr(
            cp,
            (if sec >= 0 as libc::c_int { sec } else { -sec }) as uint32,
        );
    }
    if fsec != 0 as libc::c_int {
        let mut value: int32 = if fsec >= 0 as libc::c_int {
            fsec
        } else {
            -fsec
        };
        let mut end: *mut libc::c_char =
            &mut *cp.offset((precision + 1 as libc::c_int) as isize) as *mut libc::c_char;
        let mut gotnonzero = false;
        let fresh0 = cp;
        cp = cp.offset(1);
        *fresh0 = '.' as i32 as libc::c_char;
        loop {
            let fresh1 = precision;
            precision = precision - 1;
            if !(fresh1 != 0) {
                break;
            }
            let oldval: int32 = value;
            value /= 10 as libc::c_int;
            let remainder = oldval - value * 10 as libc::c_int;
            if remainder != 0 {
                gotnonzero = true;
            }
            if gotnonzero {
                *cp.offset(precision as isize) = ('0' as i32 + remainder) as libc::c_char;
            } else {
                end = &mut *cp.offset(precision as isize) as *mut libc::c_char;
            }
        }
        if value != 0 {
            return pg_ultostr(
                cp,
                (if fsec >= 0 as libc::c_int {
                    fsec
                } else {
                    -fsec
                }) as uint32,
            );
        }
        return end;
    } else {
        return cp;
    };
}
unsafe fn AppendTimestampSeconds(
    cp: *mut libc::c_char,
    tm: *mut pg_tm,
    fsec: fsec_t,
) -> *mut libc::c_char {
    return AppendSeconds(cp, (*tm).tm_sec, fsec, 6 as libc::c_int, true);
}
unsafe fn AdjustFractSeconds(
    mut frac: libc::c_double,
    mut tm: *mut pg_tm,
    fsec: *mut fsec_t,
    scale: libc::c_int,
) {
    if frac == 0 as libc::c_int as libc::c_double {
        return;
    }
    frac *= scale as libc::c_double;
    let sec = frac as libc::c_int;
    (*tm).tm_sec += sec;
    frac -= sec as libc::c_double;
    *fsec =
        (*fsec as libc::c_double + rint(frac * 1000000 as libc::c_int as libc::c_double)) as fsec_t;
}
unsafe fn AdjustFractDays(
    mut frac: libc::c_double,
    mut tm: *mut pg_tm,
    fsec: *mut fsec_t,
    scale: libc::c_int,
) {
    if frac == 0 as libc::c_int as libc::c_double {
        return;
    }
    frac *= scale as libc::c_double;
    let extra_days = frac as libc::c_int;
    (*tm).tm_mday += extra_days;
    frac -= extra_days as libc::c_double;
    AdjustFractSeconds(frac, tm, fsec, 86400 as libc::c_int);
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

unsafe fn DetermineTimeZoneAbbrevOffsetTS(
    ts: TimestampTz,
    abbr: *const libc::c_char,
    tzp: *mut pg_tz,
    isdst: &mut bool,
) -> libc::c_int {
    let t: pg_time_t = timestamptz_to_time_t(ts);
    let mut abbr_offset: libc::c_int = 0;
    let mut tz: libc::c_int = 0;
    let mut tm: pg_tm = pg_tm {
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
    let mut fsec: fsec_t = 0;
    if DetermineTimeZoneAbbrevOffsetInternal(t, abbr, tzp, &mut abbr_offset, isdst) {
        return abbr_offset;
    }
    if timestamp2tm(
        ts,
        &mut tz,
        &mut tm,
        &mut fsec,
        0 as *mut *const libc::c_char,
        tzp,
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
                1700 as libc::c_int,
                (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    b"DetermineTimeZoneAbbrevOffsetTS\0",
                ))
                .as_ptr(),
            );
        }
        if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
            unreachable!();
        }
    }
    let zone_offset = DetermineTimeZoneOffset(&mut tm, tzp);
    *isdst = tm.tm_isdst.unwrap();
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

// Interpret parsed string as time fields only.
// Returns 0 if successful, DTERR code if bogus input detected.
//
// Note that support for time zone is here for
// SQL TIME WITH TIME ZONE, but it reveals
// bogosity with SQL date/time standards, since
// we must infer a time zone from current time.
// - thomas 2000-03-10
// Allow specifying date to get a better time zone,
// if time zones are allowed. - thomas 2001-12-26

unsafe fn DecodeTimeOnly(
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
    let mut ptype: libc::c_int = 0 as libc::c_int;
    let mut val: libc::c_int = 0;
    let mut isjulian: bool = false;
    let mut is2digits: bool = false;
    let mut bc: bool = false;
    let mut mer: libc::c_int = 2 as libc::c_int;
    let mut namedTz: *mut pg_tz = 0 as *mut pg_tz;
    let mut abbrevTz: *mut pg_tz = 0 as *mut pg_tz;
    let mut abbrev: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut valtz: *mut pg_tz = 0 as *mut pg_tz;
    *dtype = 3 as libc::c_int;
    (*tm).tm_hour = 0 as libc::c_int;
    (*tm).tm_min = 0 as libc::c_int;
    (*tm).tm_sec = 0 as libc::c_int;
    *fsec = 0 as libc::c_int;
    (*tm).tm_isdst = None;
    if !tzp.is_null() {
        *tzp = 0 as libc::c_int;
    }
    let mut current_block_201: u64;
    for i in 0..nf {
        match *ftype.offset(i as isize) {
            2 => {
                if tzp.is_null() {
                    return -(1 as libc::c_int);
                }
                if i == 0 as libc::c_int
                    && nf >= 2 as libc::c_int
                    && (*ftype.offset((nf - 1 as libc::c_int) as isize) == 2 as libc::c_int
                        || *ftype.offset(1 as libc::c_int as isize) == 3 as libc::c_int)
                {
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
                } else if *(*__ctype_b_loc())
                    .offset(**field.offset(i as isize) as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                {
                    // Starts with a digit but we already have a time
                    // field? Then we are in trouble with time already...
                    if fmask.contains(*FIELD_MASK_TIME) {
                        return -(1 as libc::c_int);
                    }
                    let cp = strchr(*field.offset(i as isize), '-' as i32);
                    if cp.is_null() {
                        return -(1 as libc::c_int);
                    }
                    let dterr = DecodeTimezone(cp, tzp);
                    if dterr != 0 {
                        return dterr;
                    }
                    *cp = '\0' as i32 as libc::c_char;
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
                    *ftype.offset(i as isize) = dterr;
                    tmask.set(RealFieldType::Tz);
                } else {
                    namedTz = pg_tzset(*field.offset(i as isize));
                    if namedTz.is_null() {
                        // We should return an error code instead of
                        // ereport'ing directly, but then there is no way
                        // to report the bad time zone name.
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
                                1859 as libc::c_int,
                                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                                    b"DecodeTimeOnly\0",
                                ))
                                .as_ptr(),
                            );
                        }
                        if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
                            unreachable!();
                        }
                    }
                    *ftype.offset(i as isize) = 4 as libc::c_int;
                    tmask = FieldMask::from(RealFieldType::Tz);
                }
                current_block_201 = 18009804086567542307;
            }
            3 => {
                let dterr = DecodeTime(
                    *field.offset(i as isize),
                    fmask | *FIELD_MASK_DATE,
                    0x7fff as libc::c_int,
                    &mut tmask,
                    tm,
                    fsec,
                );
                if dterr != 0 {
                    return dterr;
                }
                current_block_201 = 18009804086567542307;
            }
            4 => {
                let mut tz: libc::c_int = 0;
                if tzp.is_null() {
                    return -(1 as libc::c_int);
                }
                let dterr = DecodeTimezone(*field.offset(i as isize), &mut tz);
                if dterr != 0 {
                    return dterr;
                }
                *tzp = tz;
                tmask = FieldMask::from(RealFieldType::Tz);
                current_block_201 = 18009804086567542307;
            }
            0 => {
                if ptype != 0 as libc::c_int {
                    let mut cp_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    match ptype {
                        31 | 25 | 23 | 21 => {
                            if tzp.is_null() {
                                return -(1 as libc::c_int);
                            }
                        }
                        _ => {}
                    }
                    *__errno_location() = 0 as libc::c_int;
                    let val_0 = strtoint(*field.offset(i as isize), &mut cp_0, 10 as libc::c_int);
                    if *__errno_location() == 34 as libc::c_int {
                        return -(2 as libc::c_int);
                    }
                    if *cp_0 as libc::c_int == '.' as i32 {
                        match ptype {
                            31 | 3 | 18 => {}
                            _ => return -(1 as libc::c_int),
                        }
                    } else if *cp_0 as libc::c_int != '\0' as i32 {
                        return -(1 as libc::c_int);
                    }
                    match ptype {
                        25 => {
                            (*tm).tm_year = val_0;
                            tmask = FieldMask::from(RealFieldType::Year);
                        }
                        23 => {
                            // already have a month and hour? then assume minutes
                            if fmask.contains(RealFieldType::Month | RealFieldType::Hour) {
                                (*tm).tm_min = val_0;
                                tmask = FieldMask::from(RealFieldType::Minute);
                            } else {
                                (*tm).tm_mon = val_0;
                                tmask = FieldMask::from(RealFieldType::Month);
                            }
                        }
                        21 => {
                            (*tm).tm_mday = val_0;
                            tmask = FieldMask::from(RealFieldType::Day);
                        }
                        20 => {
                            (*tm).tm_hour = val_0;
                            tmask = FieldMask::from(RealFieldType::Hour);
                        }
                        19 => {
                            (*tm).tm_min = val_0;
                            tmask = FieldMask::from(RealFieldType::Minute);
                        }
                        18 => {
                            (*tm).tm_sec = val_0;
                            tmask = FieldMask::from(RealFieldType::Second);
                            if *cp_0 as libc::c_int == '.' as i32 {
                                let dterr = ParseFractionalSecond(cp_0, fsec);
                                if dterr != 0 {
                                    return dterr;
                                }
                                tmask = *FIELD_MASK_ALL_SECS;
                            }
                        }
                        4 => {
                            tmask = FieldMask::from(RealFieldType::Tz);
                            let dterr = DecodeTimezone(*field.offset(i as isize), tzp);
                            if dterr != 0 {
                                return dterr;
                            }
                        }
                        31 => {
                            if val_0 < 0 as libc::c_int {
                                return -(2 as libc::c_int);
                            }
                            tmask = *FIELD_MASK_DATE;
                            j2date(
                                val_0,
                                &mut (*tm).tm_year,
                                &mut (*tm).tm_mon,
                                &mut (*tm).tm_mday,
                            );
                            isjulian = true;
                            if *cp_0 as libc::c_int == '.' as i32 {
                                *__errno_location() = 0 as libc::c_int;
                                let mut time = strtod(cp_0, &mut cp_0);
                                if *cp_0 as libc::c_int != '\0' as i32
                                    || *__errno_location() != 0 as libc::c_int
                                {
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
                        3 => {
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
                            *ftype.offset(i as isize) = dterr;
                            if tmask != *FIELD_MASK_TIME {
                                return -(1 as libc::c_int);
                            }
                        }
                        _ => return -(1 as libc::c_int),
                    }
                    ptype = 0 as libc::c_int;
                    *dtype = 2 as libc::c_int;
                } else {
                    let flen = strlen(*field.offset(i as isize)) as libc::c_int;
                    let cp_1 = strchr(*field.offset(i as isize), '.' as i32);
                    if !cp_1.is_null() {
                        if i == 0 as libc::c_int
                            && nf >= 2 as libc::c_int
                            && *ftype.offset((nf - 1 as libc::c_int) as isize) == 2 as libc::c_int
                        {
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
                        } else if (flen as libc::c_ulong).wrapping_sub(strlen(cp_1))
                            > 2 as libc::c_int as libc::c_ulong
                        {
                            let dterr = DecodeNumberField(
                                flen,
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
                            *ftype.offset(i as isize) = dterr;
                        } else {
                            return -(1 as libc::c_int);
                        }
                    } else if flen > 4 as libc::c_int {
                        let dterr = DecodeNumberField(
                            flen,
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
                        *ftype.offset(i as isize) = dterr;
                    } else {
                        let dterr = DecodeNumber(
                            flen,
                            *field.offset(i as isize),
                            false,
                            fmask | *FIELD_MASK_DATE,
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
                current_block_201 = 18009804086567542307;
            }
            1 | 6 => {
                let mut type_0 =
                    DecodeTimezoneAbbrev(i, *field.offset(i as isize), &mut val, &mut valtz);
                if type_0 == RealFieldType::UnknownField {
                    type_0 = DecodeSpecial(i, *field.offset(i as isize), &mut val);
                }
                if type_0 == RealFieldType::IgnoreDtf {
                    current_block_201 = 13536709405535804910;
                } else {
                    tmask = FieldMask::from(type_0);
                    match type_0 {
                        RealFieldType::Reserved => match val {
                            12 => {
                                tmask = *FIELD_MASK_TIME;
                                *dtype = 3 as libc::c_int;
                                GetCurrentTimeUsec(tm, fsec, 0 as *mut libc::c_int);
                            }
                            16 => {
                                tmask = *FIELD_MASK_TIME | RealFieldType::Tz;
                                *dtype = 3 as libc::c_int;
                                (*tm).tm_hour = 0 as libc::c_int;
                                (*tm).tm_min = 0 as libc::c_int;
                                (*tm).tm_sec = 0 as libc::c_int;
                                (*tm).tm_isdst = Some(false);
                            }
                            _ => return -(1 as libc::c_int),
                        },
                        RealFieldType::DtzMod => {
                            // daylight savings time modifier (solves "MET DST" syntax)
                            tmask.set(RealFieldType::DTz);
                            (*tm).tm_isdst = Some(true);
                            if tzp.is_null() {
                                return -(1 as libc::c_int);
                            }
                            *tzp -= val;
                        }
                        RealFieldType::DTz => {
                            // set mask for TZ here _or_ check for DTZ later when getting default timezone
                            tmask.set(RealFieldType::Tz);
                            (*tm).tm_isdst = Some(true);
                            if tzp.is_null() {
                                return -(1 as libc::c_int);
                            }
                            *tzp = -val;
                            *ftype.offset(i as isize) = 4 as libc::c_int;
                        }
                        RealFieldType::Tz => {
                            (*tm).tm_isdst = Some(false);
                            if tzp.is_null() {
                                return -(1 as libc::c_int);
                            }
                            *tzp = -val;
                            *ftype.offset(i as isize) = 4 as libc::c_int;
                        }
                        RealFieldType::DynTz => {
                            tmask.set(RealFieldType::Tz);
                            if tzp.is_null() {
                                return -(1 as libc::c_int);
                            }
                            // we'll determine the actual offset later
                            abbrevTz = valtz;
                            abbrev = *field.offset(i as isize);
                            *ftype.offset(i as isize) = 4 as libc::c_int;
                        }
                        RealFieldType::AmPm => {
                            mer = val;
                        }
                        RealFieldType::Adbc => {
                            bc = val == 1;
                        }
                        RealFieldType::Units => {
                            tmask = FieldMask::none();
                            ptype = val;
                        }
                        RealFieldType::IsoTime => {
                            tmask = FieldMask::none();
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
                                return -(1 as libc::c_int);
                            }
                            ptype = val;
                        }
                        RealFieldType::UnknownField => {
                            namedTz = pg_tzset(*field.offset(i as isize));
                            if namedTz.is_null() {
                                return -(1 as libc::c_int);
                            }
                            tmask = FieldMask::from(RealFieldType::Tz);
                        }
                        typ => {
                            eprintln!("unexpected field type {:?}", typ);
                            return -(1 as libc::c_int);
                        }
                    }
                    current_block_201 = 18009804086567542307;
                }
            }
            _ => return -(1 as libc::c_int),
        }
        match current_block_201 {
            18009804086567542307 => {
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

    // check for time overflow
    if time_overflows((*tm).tm_hour, (*tm).tm_min, (*tm).tm_sec, *fsec) {
        return -(2 as libc::c_int);
    }
    if !fmask.contains(*FIELD_MASK_TIME) {
        return -(1 as libc::c_int);
    }
    // If we had a full timezone spec, compute the offset (we could not do it
    // before, because we may need the date to resolve DST status).
    if !namedTz.is_null() {
        let mut gmtoff: libc::c_long = 0;
        // daylight savings time modifier disallowed with full TZ
        if fmask.contains(RealFieldType::DtzMod) {
            return -(1 as libc::c_int);
        }
        // if non-DST zone, we do not need to know the date
        if pg_get_timezone_offset(namedTz, &mut gmtoff) {
            *tzp = -(gmtoff as libc::c_int);
        } else {
            // a date has to be specified
            if !fmask.contains(*FIELD_MASK_DATE) {
                return -(1 as libc::c_int);
            }
            *tzp = DetermineTimeZoneOffset(tm, namedTz);
        }
    }
    // Likewise, if we had a dynamic timezone abbreviation, resolve it now.
    if !abbrevTz.is_null() {
        let mut tt: pg_tm = pg_tm {
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
        let mut tmp: *mut pg_tm = &mut tt;

        // daylight savings time modifier but no standard timezone? then error
        if fmask.contains(RealFieldType::DtzMod) {
            return -(1 as libc::c_int);
        }
        if !fmask.intersects(*FIELD_MASK_DATE) {
            GetCurrentDateTime(tmp);
        } else {
            /* a date has to be specified */
            if !fmask.contains(*FIELD_MASK_DATE) {
                return -(1 as libc::c_int);
            }
            (*tmp).tm_year = (*tm).tm_year;
            (*tmp).tm_mon = (*tm).tm_mon;
            (*tmp).tm_mday = (*tm).tm_mday;
        }
        (*tmp).tm_hour = (*tm).tm_hour;
        (*tmp).tm_min = (*tm).tm_min;
        (*tmp).tm_sec = (*tm).tm_sec;
        *tzp = DetermineTimeZoneAbbrevOffset(tmp, abbrev, abbrevTz);
        (*tm).tm_isdst = (*tmp).tm_isdst;
    }
    // timezone not specified? then use session timezone
    if !tzp.is_null() && !fmask.contains(RealFieldType::Tz) {
        let mut tt_0: pg_tm = pg_tm {
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
        let mut tmp_0: *mut pg_tm = &mut tt_0;
        if fmask.contains(RealFieldType::DtzMod) {
            return -(1 as libc::c_int);
        }
        if !fmask.intersects(*FIELD_MASK_DATE) {
            GetCurrentDateTime(tmp_0);
        } else {
            if !fmask.contains(*FIELD_MASK_DATE) {
                return -(1 as libc::c_int);
            }
            (*tmp_0).tm_year = (*tm).tm_year;
            (*tmp_0).tm_mon = (*tm).tm_mon;
            (*tmp_0).tm_mday = (*tm).tm_mday;
        }
        (*tmp_0).tm_hour = (*tm).tm_hour;
        (*tmp_0).tm_min = (*tm).tm_min;
        (*tmp_0).tm_sec = (*tm).tm_sec;
        *tzp = DetermineTimeZoneOffset(tmp_0, session_timezone);
        (*tm).tm_isdst = (*tmp_0).tm_isdst;
    }
    return 0 as libc::c_int;
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
#[inline]
unsafe fn ClearPgTm(mut tm: *mut pg_tm, fsec: *mut fsec_t) {
    (*tm).tm_year = 0 as libc::c_int;
    (*tm).tm_mon = 0 as libc::c_int;
    (*tm).tm_mday = 0 as libc::c_int;
    (*tm).tm_hour = 0 as libc::c_int;
    (*tm).tm_min = 0 as libc::c_int;
    (*tm).tm_sec = 0 as libc::c_int;
    *fsec = 0 as libc::c_int;
}

unsafe fn DecodeInterval(
    field: *mut *mut libc::c_char,
    ftype: *mut libc::c_int,
    nf: libc::c_int,
    range: libc::c_int,
    dtype: *mut libc::c_int,
    mut tm: *mut pg_tm,
    fsec: *mut fsec_t,
) -> libc::c_int {
    let mut is_before = false;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fmask = FieldMask::none();
    let mut tmask = FieldMask::none();
    let mut type_0;
    let mut i;
    let mut dterr;
    let mut val: libc::c_int = 0;
    let mut fval;
    *dtype = 17 as libc::c_int;
    type_0 = 8 as libc::c_int;
    ClearPgTm(tm, fsec);
    let mut current_block_109: u64;
    i = nf - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        match *ftype.offset(i as isize) {
            3 => {
                dterr = DecodeTime(
                    *field.offset(i as isize),
                    fmask,
                    range,
                    &mut tmask,
                    tm,
                    fsec,
                );
                if dterr != 0 {
                    return dterr;
                }
                type_0 = 21 as libc::c_int;
                current_block_109 = 2793352396589381719;
            }
            4 => {
                if !(strchr(
                    (*field.offset(i as isize)).offset(1 as libc::c_int as isize),
                    ':' as i32,
                ))
                .is_null()
                    && DecodeTime(
                        (*field.offset(i as isize)).offset(1 as libc::c_int as isize),
                        fmask,
                        range,
                        &mut tmask,
                        tm,
                        fsec,
                    ) == 0 as libc::c_int
                {
                    if **field.offset(i as isize) as libc::c_int == '-' as i32 {
                        (*tm).tm_hour = -(*tm).tm_hour;
                        (*tm).tm_min = -(*tm).tm_min;
                        (*tm).tm_sec = -(*tm).tm_sec;
                        *fsec = -*fsec;
                    }
                    type_0 = 21 as libc::c_int;
                    current_block_109 = 2793352396589381719;
                } else {
                    current_block_109 = 4548417354006578472;
                }
            }
            2 | 0 => {
                current_block_109 = 4548417354006578472;
            }
            1 | 6 => {
                type_0 = DecodeUnits(i, *field.offset(i as isize), &mut val);
                if type_0 == 8 as libc::c_int {
                    current_block_109 = 7095457783677275021;
                } else {
                    tmask = FieldMask::none();
                    match type_0 {
                        17 => {
                            type_0 = val;
                        }
                        19 => {
                            is_before = true;
                            type_0 = val;
                        }
                        0 => {
                            tmask = *FIELD_MASK_DATE | *FIELD_MASK_TIME;
                            *dtype = val;
                        }
                        _ => return -(1 as libc::c_int),
                    }
                    current_block_109 = 2793352396589381719;
                }
            }
            _ => return -(1 as libc::c_int),
        }
        match current_block_109 {
            4548417354006578472 => {
                if type_0 == 8 as libc::c_int {
                    match range {
                        4 => {
                            type_0 = 25 as libc::c_int;
                        }
                        2 | 6 => {
                            type_0 = 23 as libc::c_int;
                        }
                        8 => {
                            type_0 = 21 as libc::c_int;
                        }
                        1024 | 1032 => {
                            type_0 = 20 as libc::c_int;
                        }
                        2048 | 3072 | 3080 => {
                            type_0 = 19 as libc::c_int;
                        }
                        4096 | 6144 | 7168 | 7176 => {
                            type_0 = 18 as libc::c_int;
                        }
                        _ => {
                            type_0 = 18 as libc::c_int;
                        }
                    }
                }
                *__errno_location() = 0 as libc::c_int;
                val = strtoint(*field.offset(i as isize), &mut cp, 10 as libc::c_int);
                if *__errno_location() == 34 as libc::c_int {
                    return -(2 as libc::c_int);
                }
                if *cp as libc::c_int == '-' as i32 {
                    let mut val2 = strtoint(
                        cp.offset(1 as libc::c_int as isize),
                        &mut cp,
                        10 as libc::c_int,
                    );
                    if *__errno_location() == 34 as libc::c_int
                        || val2 < 0 as libc::c_int
                        || val2 >= 12 as libc::c_int
                    {
                        return -(2 as libc::c_int);
                    }
                    if *cp as libc::c_int != '\0' as i32 {
                        return -(1 as libc::c_int);
                    }
                    type_0 = 23 as libc::c_int;
                    if **field.offset(i as isize) as libc::c_int == '-' as i32 {
                        val2 = -val2;
                    }
                    if val as libc::c_double * 12 as libc::c_int as libc::c_double
                        + val2 as libc::c_double
                        > 2147483647 as libc::c_int as libc::c_double
                        || (val as libc::c_double * 12 as libc::c_int as libc::c_double
                            + val2 as libc::c_double)
                            < (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
                    {
                        return -(2 as libc::c_int);
                    }
                    val = val * 12 as libc::c_int + val2;
                    fval = 0 as libc::c_int as libc::c_double;
                } else if *cp as libc::c_int == '.' as i32 {
                    *__errno_location() = 0 as libc::c_int;
                    fval = strtod(cp, &mut cp);
                    if *cp as libc::c_int != '\0' as i32 || *__errno_location() != 0 as libc::c_int
                    {
                        return -(1 as libc::c_int);
                    }
                    if **field.offset(i as isize) as libc::c_int == '-' as i32 {
                        fval = -fval;
                    }
                } else if *cp as libc::c_int == '\0' as i32 {
                    fval = 0 as libc::c_int as libc::c_double;
                } else {
                    return -(1 as libc::c_int);
                }
                tmask = match type_0 {
                    30 => {
                        *fsec = (*fsec as libc::c_double + rint(val as libc::c_double + fval))
                            as fsec_t;
                        FieldMask::from(RealFieldType::Microsecond)
                    }
                    29 => {
                        (*tm).tm_sec += val / 1000 as libc::c_int;
                        val -= val / 1000 as libc::c_int * 1000 as libc::c_int;
                        *fsec = (*fsec as libc::c_double
                            + rint(
                                (val as libc::c_double + fval)
                                    * 1000 as libc::c_int as libc::c_double,
                            )) as fsec_t;
                        FieldMask::from(RealFieldType::Millisecond)
                    }
                    18 => {
                        (*tm).tm_sec += val;
                        *fsec = (*fsec as libc::c_double
                            + rint(fval * 1000000 as libc::c_int as libc::c_double))
                            as fsec_t;
                        if fval == 0 as libc::c_int as libc::c_double {
                            FieldMask::from(RealFieldType::Second)
                        } else {
                            *FIELD_MASK_ALL_SECS
                        }
                    }
                    19 => {
                        (*tm).tm_min += val;
                        AdjustFractSeconds(fval, tm, fsec, 60 as libc::c_int);
                        FieldMask::from(RealFieldType::Minute)
                    }
                    20 => {
                        (*tm).tm_hour += val;
                        AdjustFractSeconds(fval, tm, fsec, 3600 as libc::c_int);
                        type_0 = 21 as libc::c_int;
                        FieldMask::from(RealFieldType::Hour)
                    }
                    21 => {
                        (*tm).tm_mday += val;
                        AdjustFractSeconds(fval, tm, fsec, 86400 as libc::c_int);
                        FieldMask::from(RealFieldType::Day)
                    }
                    22 => {
                        (*tm).tm_mday += val * 7 as libc::c_int;
                        AdjustFractDays(fval, tm, fsec, 7 as libc::c_int);
                        FieldMask::from(RealFieldType::Week)
                    }
                    23 => {
                        (*tm).tm_mon += val;
                        AdjustFractDays(fval, tm, fsec, 30 as libc::c_int);
                        FieldMask::from(RealFieldType::Month)
                    }
                    25 => {
                        (*tm).tm_year += val;
                        (*tm).tm_mon = ((*tm).tm_mon as libc::c_double
                            + rint(fval * 12 as libc::c_int as libc::c_double))
                            as libc::c_int;
                        FieldMask::from(RealFieldType::Year)
                    }
                    26 => {
                        (*tm).tm_year += val * 10 as libc::c_int;
                        (*tm).tm_mon = ((*tm).tm_mon as libc::c_double
                            + rint(
                                fval * 12 as libc::c_int as libc::c_double
                                    * 10 as libc::c_int as libc::c_double,
                            )) as libc::c_int;
                        FieldMask::from(RealFieldType::Decade)
                    }
                    27 => {
                        (*tm).tm_year += val * 100 as libc::c_int;
                        (*tm).tm_mon = ((*tm).tm_mon as libc::c_double
                            + rint(
                                fval * 12 as libc::c_int as libc::c_double
                                    * 100 as libc::c_int as libc::c_double,
                            )) as libc::c_int;
                        FieldMask::from(RealFieldType::Century)
                    }
                    28 => {
                        (*tm).tm_year += val * 1000 as libc::c_int;
                        (*tm).tm_mon = ((*tm).tm_mon as libc::c_double
                            + rint(
                                fval * 12 as libc::c_int as libc::c_double
                                    * 1000 as libc::c_int as libc::c_double,
                            )) as libc::c_int;
                        FieldMask::from(RealFieldType::Millennium)
                    }
                    _ => return -(1 as libc::c_int),
                };
                current_block_109 = 2793352396589381719;
            }
            _ => {}
        }
        match current_block_109 {
            2793352396589381719 => {
                if tmask.intersects(fmask) {
                    return -(1 as libc::c_int);
                }
                fmask |= tmask;
            }
            _ => {}
        }
        i -= 1;
    }
    if fmask.is_none() {
        return -(1 as libc::c_int);
    }
    if *fsec != 0 as libc::c_int {
        let sec = (*fsec as libc::c_long / 1000000 as libc::c_long) as libc::c_int;
        *fsec = (*fsec as libc::c_long - sec as libc::c_long * 1000000 as libc::c_long) as fsec_t;
        (*tm).tm_sec += sec;
    }
    if IntervalStyle == 2 as libc::c_int
        && **field.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
    {
        let mut more_signs = false;
        i = 1 as libc::c_int;
        while i < nf {
            if **field.offset(i as isize) as libc::c_int == '-' as i32
                || **field.offset(i as isize) as libc::c_int == '+' as i32
            {
                more_signs = true;
                break;
            } else {
                i += 1;
            }
        }
        if !more_signs {
            if *fsec > 0 as libc::c_int {
                *fsec = -*fsec;
            }
            if (*tm).tm_sec > 0 as libc::c_int {
                (*tm).tm_sec = -(*tm).tm_sec;
            }
            if (*tm).tm_min > 0 as libc::c_int {
                (*tm).tm_min = -(*tm).tm_min;
            }
            if (*tm).tm_hour > 0 as libc::c_int {
                (*tm).tm_hour = -(*tm).tm_hour;
            }
            if (*tm).tm_mday > 0 as libc::c_int {
                (*tm).tm_mday = -(*tm).tm_mday;
            }
            if (*tm).tm_mon > 0 as libc::c_int {
                (*tm).tm_mon = -(*tm).tm_mon;
            }
            if (*tm).tm_year > 0 as libc::c_int {
                (*tm).tm_year = -(*tm).tm_year;
            }
        }
    }
    if is_before {
        *fsec = -*fsec;
        (*tm).tm_sec = -(*tm).tm_sec;
        (*tm).tm_min = -(*tm).tm_min;
        (*tm).tm_hour = -(*tm).tm_hour;
        (*tm).tm_mday = -(*tm).tm_mday;
        (*tm).tm_mon = -(*tm).tm_mon;
        (*tm).tm_year = -(*tm).tm_year;
    }
    return 0 as libc::c_int;
}
unsafe fn ParseISO8601Number(
    str: *mut libc::c_char,
    endptr: *mut *mut libc::c_char,
    ipart: *mut libc::c_int,
    fpart: *mut libc::c_double,
) -> libc::c_int {
    if !(*(*__ctype_b_loc()).offset(*str as libc::c_uchar as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        != 0
        || *str as libc::c_int == '-' as i32
        || *str as libc::c_int == '.' as i32)
    {
        return -(1 as libc::c_int);
    }
    *__errno_location() = 0 as libc::c_int;
    let val = strtod(str, endptr);
    if *endptr == str || *__errno_location() != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if val < (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
        || val > 2147483647 as libc::c_int as libc::c_double
    {
        return -(2 as libc::c_int);
    }
    if val >= 0 as libc::c_int as libc::c_double {
        *ipart = floor(val) as libc::c_int;
    } else {
        *ipart = -floor(-val) as libc::c_int;
    }
    *fpart = val - *ipart as libc::c_double;
    return 0 as libc::c_int;
}
unsafe fn ISO8601IntegerWidth(mut fieldstart: *mut libc::c_char) -> libc::c_int {
    if *fieldstart as libc::c_int == '-' as i32 {
        fieldstart = fieldstart.offset(1);
    }
    return strspn(
        fieldstart,
        b"0123456789\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
}

unsafe fn DecodeISO8601Interval(
    mut str: *mut libc::c_char,
    dtype: *mut libc::c_int,
    mut tm: *mut pg_tm,
    fsec: *mut fsec_t,
) -> libc::c_int {
    let mut datepart = true;
    let mut havefield = false;
    *dtype = 17 as libc::c_int;
    ClearPgTm(tm, fsec);
    if strlen(str) < 2 as libc::c_int as libc::c_ulong
        || *str.offset(0 as libc::c_int as isize) as libc::c_int != 'P' as i32
    {
        return -(1 as libc::c_int);
    }
    str = str.offset(1);
    let mut current_block_100: u64;
    while *str != 0 {
        let mut val: libc::c_int = 0;
        let mut fval: libc::c_double = 0.;
        if *str as libc::c_int == 'T' as i32 {
            datepart = false;
            havefield = false;
            str = str.offset(1);
        } else {
            let fieldstart = str;
            let dterr = ParseISO8601Number(str, &mut str, &mut val, &mut fval);
            if dterr != 0 {
                return dterr;
            }
            let fresh43 = str;
            str = str.offset(1);
            let unit = *fresh43;
            if datepart {
                match unit as libc::c_int {
                    89 => {
                        (*tm).tm_year += val;
                        (*tm).tm_mon = ((*tm).tm_mon as libc::c_double
                            + rint(fval * 12 as libc::c_int as libc::c_double))
                            as libc::c_int;
                        current_block_100 = 5722677567366458307;
                    }
                    77 => {
                        (*tm).tm_mon += val;
                        AdjustFractDays(fval, tm, fsec, 30 as libc::c_int);
                        current_block_100 = 5722677567366458307;
                    }
                    87 => {
                        (*tm).tm_mday += val * 7 as libc::c_int;
                        AdjustFractDays(fval, tm, fsec, 7 as libc::c_int);
                        current_block_100 = 5722677567366458307;
                    }
                    68 => {
                        (*tm).tm_mday += val;
                        AdjustFractSeconds(fval, tm, fsec, 86400 as libc::c_int);
                        current_block_100 = 5722677567366458307;
                    }
                    84 | 0 => {
                        if ISO8601IntegerWidth(fieldstart) == 8 as libc::c_int && !havefield {
                            (*tm).tm_year += val / 10000 as libc::c_int;
                            (*tm).tm_mon += val / 100 as libc::c_int % 100 as libc::c_int;
                            (*tm).tm_mday += val % 100 as libc::c_int;
                            AdjustFractSeconds(fval, tm, fsec, 86400 as libc::c_int);
                            if unit as libc::c_int == '\0' as i32 {
                                return 0 as libc::c_int;
                            }
                            datepart = false;
                            havefield = false;
                            continue;
                        } else {
                            current_block_100 = 4994089973047350430;
                        }
                    }
                    45 => {
                        current_block_100 = 4994089973047350430;
                    }
                    _ => return -(1 as libc::c_int),
                }
                match current_block_100 {
                    5722677567366458307 => {}
                    _ => {
                        if havefield {
                            return -(1 as libc::c_int);
                        }
                        (*tm).tm_year += val;
                        (*tm).tm_mon = ((*tm).tm_mon as libc::c_double
                            + rint(fval * 12 as libc::c_int as libc::c_double))
                            as libc::c_int;
                        if unit as libc::c_int == '\0' as i32 {
                            return 0 as libc::c_int;
                        }
                        if unit as libc::c_int == 'T' as i32 {
                            datepart = false;
                            havefield = false;
                            continue;
                        } else {
                            let dterr = ParseISO8601Number(str, &mut str, &mut val, &mut fval);
                            if dterr != 0 {
                                return dterr;
                            }
                            (*tm).tm_mon += val;
                            AdjustFractDays(fval, tm, fsec, 30 as libc::c_int);
                            if *str as libc::c_int == '\0' as i32 {
                                return 0 as libc::c_int;
                            }
                            if *str as libc::c_int == 'T' as i32 {
                                datepart = false;
                                havefield = false;
                                continue;
                            } else {
                                if *str as libc::c_int != '-' as i32 {
                                    return -(1 as libc::c_int);
                                }
                                str = str.offset(1);
                                let dterr = ParseISO8601Number(str, &mut str, &mut val, &mut fval);
                                if dterr != 0 {
                                    return dterr;
                                }
                                (*tm).tm_mday += val;
                                AdjustFractSeconds(fval, tm, fsec, 86400 as libc::c_int);
                                if *str as libc::c_int == '\0' as i32 {
                                    return 0 as libc::c_int;
                                }
                                if *str as libc::c_int == 'T' as i32 {
                                    datepart = false;
                                    havefield = false;
                                    continue;
                                } else {
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
            } else {
                let current_block_97: u64;
                match unit as libc::c_int {
                    72 => {
                        (*tm).tm_hour += val;
                        AdjustFractSeconds(fval, tm, fsec, 3600 as libc::c_int);
                        current_block_97 = 9879896046554623444;
                    }
                    77 => {
                        (*tm).tm_min += val;
                        AdjustFractSeconds(fval, tm, fsec, 60 as libc::c_int);
                        current_block_97 = 9879896046554623444;
                    }
                    83 => {
                        (*tm).tm_sec += val;
                        AdjustFractSeconds(fval, tm, fsec, 1 as libc::c_int);
                        current_block_97 = 9879896046554623444;
                    }
                    0 => {
                        if ISO8601IntegerWidth(fieldstart) == 6 as libc::c_int && !havefield {
                            (*tm).tm_hour += val / 10000 as libc::c_int;
                            (*tm).tm_min += val / 100 as libc::c_int % 100 as libc::c_int;
                            (*tm).tm_sec += val % 100 as libc::c_int;
                            AdjustFractSeconds(fval, tm, fsec, 1 as libc::c_int);
                            return 0 as libc::c_int;
                        }
                        current_block_97 = 1576211671199890301;
                    }
                    58 => {
                        current_block_97 = 1576211671199890301;
                    }
                    _ => return -(1 as libc::c_int),
                }
                match current_block_97 {
                    9879896046554623444 => {}
                    _ => {
                        if havefield {
                            return -(1 as libc::c_int);
                        }
                        (*tm).tm_hour += val;
                        AdjustFractSeconds(fval, tm, fsec, 3600 as libc::c_int);
                        if unit as libc::c_int == '\0' as i32 {
                            return 0 as libc::c_int;
                        }
                        let dterr = ParseISO8601Number(str, &mut str, &mut val, &mut fval);
                        if dterr != 0 {
                            return dterr;
                        }
                        (*tm).tm_min += val;
                        AdjustFractSeconds(fval, tm, fsec, 60 as libc::c_int);
                        if *str as libc::c_int == '\0' as i32 {
                            return 0 as libc::c_int;
                        }
                        if *str as libc::c_int != ':' as i32 {
                            return -(1 as libc::c_int);
                        }
                        str = str.offset(1);
                        let dterr = ParseISO8601Number(str, &mut str, &mut val, &mut fval);
                        if dterr != 0 {
                            return dterr;
                        }
                        (*tm).tm_sec += val;
                        AdjustFractSeconds(fval, tm, fsec, 1 as libc::c_int);
                        if *str as libc::c_int == '\0' as i32 {
                            return 0 as libc::c_int;
                        }
                        return -(1 as libc::c_int);
                    }
                }
            }
            havefield = true;
        }
    }
    return 0 as libc::c_int;
}

unsafe fn DecodeUnits(
    field: libc::c_int,
    lowtoken: *mut libc::c_char,
    val: *mut libc::c_int,
) -> libc::c_int {
    let mut tp = deltacache[field as usize];
    if tp.is_null()
        || strncmp(
            lowtoken,
            ((*tp).token).as_ptr(),
            10 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        tp = datebsearch(lowtoken, deltatktbl.as_ptr(), szdeltatktbl);
    }
    if tp.is_null() {
        *val = 0 as libc::c_int;
        31 as libc::c_int
    } else {
        deltacache[field as usize] = tp;
        *val = (*tp).value;
        (*tp).type_0 as libc::c_int
    }
}

unsafe fn DateTimeParseError(
    dterr: libc::c_int,
    str: *const libc::c_char,
    datatype: *const libc::c_char,
) -> () {
    match dterr {
        -2 => {
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
                errmsg(
                    b"date/time field value out of range: \"%s\"\0" as *const u8
                        as *const libc::c_char,
                    str as *mut _,
                );
                errfinish(
                    b"/home/petrosagg/projects/postgres-datetime/src/datetime.c\0" as *const u8
                        as *const libc::c_char,
                    3772 as libc::c_int,
                    (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"DateTimeParseError\0",
                    ))
                    .as_ptr(),
                );
            }
            if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
                unreachable!();
            }
        }
        -3 => {
            let mut __errno_location_1: libc::c_int = 0;
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
                errmsg(
                    b"date/time field value out of range: \"%s\"\0" as *const u8
                        as *const libc::c_char,
                    str as *mut _,
                );
                errhint(
                    b"Perhaps you need a different \"datestyle\" setting.\0" as *const u8
                        as *const libc::c_char,
                );
                errfinish(
                    b"/home/petrosagg/projects/postgres-datetime/src/datetime.c\0" as *const u8
                        as *const libc::c_char,
                    3780 as libc::c_int,
                    (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"DateTimeParseError\0",
                    ))
                    .as_ptr(),
                );
            }
            if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
                unreachable!();
            }
        }
        -4 => {
            let mut __errno_location_2: libc::c_int = 0;
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
                        + (('1' as i32 - '0' as i32 & 0x3f as libc::c_int) << 18 as libc::c_int)
                        + (('5' as i32 - '0' as i32 & 0x3f as libc::c_int) << 24 as libc::c_int),
                );
                errmsg(
                    b"interval field value out of range: \"%s\"\0" as *const u8
                        as *const libc::c_char,
                    str as *mut _,
                );
                errfinish(
                    b"/home/petrosagg/projects/postgres-datetime/src/datetime.c\0" as *const u8
                        as *const libc::c_char,
                    3786 as libc::c_int,
                    (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"DateTimeParseError\0",
                    ))
                    .as_ptr(),
                );
            }
            if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
                unreachable!();
            }
        }
        -5 => {
            let mut __errno_location_3: libc::c_int = 0;
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
                        + (('9' as i32 - '0' as i32 & 0x3f as libc::c_int) << 24 as libc::c_int),
                );
                errmsg(
                    b"time zone displacement out of range: \"%s\"\0" as *const u8
                        as *const libc::c_char,
                    str as *mut _,
                );
                errfinish(
                    b"/home/petrosagg/projects/postgres-datetime/src/datetime.c\0" as *const u8
                        as *const libc::c_char,
                    3792 as libc::c_int,
                    (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"DateTimeParseError\0",
                    ))
                    .as_ptr(),
                );
            }
            if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
                unreachable!();
            }
        }
        -1 | _ => {
            let mut __errno_location_4: libc::c_int = 0;
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
                        + (('7' as i32 - '0' as i32 & 0x3f as libc::c_int) << 24 as libc::c_int),
                );
                errmsg2(
                    b"invalid input syntax for type %s: \"%s\"\0" as *const u8
                        as *const libc::c_char,
                    datatype as *mut _,
                    str as *mut _,
                );
                errfinish(
                    b"/home/petrosagg/projects/postgres-datetime/src/datetime.c\0" as *const u8
                        as *const libc::c_char,
                    3799 as libc::c_int,
                    (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"DateTimeParseError\0",
                    ))
                    .as_ptr(),
                );
            }
            if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
                unreachable!();
            }
        }
    };
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

unsafe fn EncodeTimezone(
    mut str: *mut libc::c_char,
    tz: libc::c_int,
    style: libc::c_int,
) -> *mut libc::c_char {
    let mut sec = abs(tz);
    let mut min = sec / 60 as libc::c_int;
    sec -= min * 60 as libc::c_int;
    let hour = min / 60 as libc::c_int;
    min -= hour * 60 as libc::c_int;
    let fresh44 = str;
    str = str.offset(1);
    *fresh44 = (if tz <= 0 as libc::c_int {
        '+' as i32
    } else {
        '-' as i32
    }) as libc::c_char;
    if sec != 0 as libc::c_int {
        str = pg_ultostr_zeropad(str, hour as uint32, 2 as libc::c_int);
        let fresh45 = str;
        str = str.offset(1);
        *fresh45 = ':' as i32 as libc::c_char;
        str = pg_ultostr_zeropad(str, min as uint32, 2 as libc::c_int);
        let fresh46 = str;
        str = str.offset(1);
        *fresh46 = ':' as i32 as libc::c_char;
        str = pg_ultostr_zeropad(str, sec as uint32, 2 as libc::c_int);
    } else if min != 0 as libc::c_int || style == 4 as libc::c_int {
        str = pg_ultostr_zeropad(str, hour as uint32, 2 as libc::c_int);
        let fresh47 = str;
        str = str.offset(1);
        *fresh47 = ':' as i32 as libc::c_char;
        str = pg_ultostr_zeropad(str, min as uint32, 2 as libc::c_int);
    } else {
        str = pg_ultostr_zeropad(str, hour as uint32, 2 as libc::c_int);
    }
    return str;
}

unsafe fn EncodeDateOnly(tm: *mut pg_tm, style: libc::c_int, mut str: *mut libc::c_char) {
    match style {
        1 | 4 => {
            str = pg_ultostr_zeropad(
                str,
                (if (*tm).tm_year > 0 as libc::c_int {
                    (*tm).tm_year
                } else {
                    -((*tm).tm_year - 1 as libc::c_int)
                }) as uint32,
                4 as libc::c_int,
            );
            let fresh48 = str;
            str = str.offset(1);
            *fresh48 = '-' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(str, (*tm).tm_mon as uint32, 2 as libc::c_int);
            let fresh49 = str;
            str = str.offset(1);
            *fresh49 = '-' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(str, (*tm).tm_mday as uint32, 2 as libc::c_int);
        }
        2 => {
            if DateOrder == 1 as libc::c_int {
                str = pg_ultostr_zeropad(str, (*tm).tm_mday as uint32, 2 as libc::c_int);
                let fresh50 = str;
                str = str.offset(1);
                *fresh50 = '/' as i32 as libc::c_char;
                str = pg_ultostr_zeropad(str, (*tm).tm_mon as uint32, 2 as libc::c_int);
            } else {
                str = pg_ultostr_zeropad(str, (*tm).tm_mon as uint32, 2 as libc::c_int);
                let fresh51 = str;
                str = str.offset(1);
                *fresh51 = '/' as i32 as libc::c_char;
                str = pg_ultostr_zeropad(str, (*tm).tm_mday as uint32, 2 as libc::c_int);
            }
            let fresh52 = str;
            str = str.offset(1);
            *fresh52 = '/' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(
                str,
                (if (*tm).tm_year > 0 as libc::c_int {
                    (*tm).tm_year
                } else {
                    -((*tm).tm_year - 1 as libc::c_int)
                }) as uint32,
                4 as libc::c_int,
            );
        }
        3 => {
            str = pg_ultostr_zeropad(str, (*tm).tm_mday as uint32, 2 as libc::c_int);
            let fresh53 = str;
            str = str.offset(1);
            *fresh53 = '.' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(str, (*tm).tm_mon as uint32, 2 as libc::c_int);
            let fresh54 = str;
            str = str.offset(1);
            *fresh54 = '.' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(
                str,
                (if (*tm).tm_year > 0 as libc::c_int {
                    (*tm).tm_year
                } else {
                    -((*tm).tm_year - 1 as libc::c_int)
                }) as uint32,
                4 as libc::c_int,
            );
        }
        0 | _ => {
            if DateOrder == 1 as libc::c_int {
                str = pg_ultostr_zeropad(str, (*tm).tm_mday as uint32, 2 as libc::c_int);
                let fresh55 = str;
                str = str.offset(1);
                *fresh55 = '-' as i32 as libc::c_char;
                str = pg_ultostr_zeropad(str, (*tm).tm_mon as uint32, 2 as libc::c_int);
            } else {
                str = pg_ultostr_zeropad(str, (*tm).tm_mon as uint32, 2 as libc::c_int);
                let fresh56 = str;
                str = str.offset(1);
                *fresh56 = '-' as i32 as libc::c_char;
                str = pg_ultostr_zeropad(str, (*tm).tm_mday as uint32, 2 as libc::c_int);
            }
            let fresh57 = str;
            str = str.offset(1);
            *fresh57 = '-' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(
                str,
                (if (*tm).tm_year > 0 as libc::c_int {
                    (*tm).tm_year
                } else {
                    -((*tm).tm_year - 1 as libc::c_int)
                }) as uint32,
                4 as libc::c_int,
            );
        }
    }
    if (*tm).tm_year <= 0 as libc::c_int {
        memcpy(
            str as *mut libc::c_void,
            b" BC\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        );
        str = str.offset(3 as libc::c_int as isize);
    }
    *str = '\0' as i32 as libc::c_char;
}

unsafe fn EncodeTimeOnly(
    tm: *mut pg_tm,
    fsec: fsec_t,
    print_tz: bool,
    tz: libc::c_int,
    style: libc::c_int,
    mut str: *mut libc::c_char,
) {
    str = pg_ultostr_zeropad(str, (*tm).tm_hour as uint32, 2 as libc::c_int);
    let fresh58 = str;
    str = str.offset(1);
    *fresh58 = ':' as i32 as libc::c_char;
    str = pg_ultostr_zeropad(str, (*tm).tm_min as uint32, 2 as libc::c_int);
    let fresh59 = str;
    str = str.offset(1);
    *fresh59 = ':' as i32 as libc::c_char;
    str = AppendSeconds(str, (*tm).tm_sec, fsec, 6 as libc::c_int, true);
    if print_tz {
        str = EncodeTimezone(str, tz, style);
    }
    *str = '\0' as i32 as libc::c_char;
}

unsafe fn EncodeDateTime(
    mut tm: *mut pg_tm,
    fsec: fsec_t,
    mut print_tz: bool,
    tz: libc::c_int,
    tzn: *const libc::c_char,
    style: libc::c_int,
    mut str: *mut libc::c_char,
) {
    if (*tm).tm_isdst.is_none() {
        print_tz = false;
    }
    match style {
        1 | 4 => {
            str = pg_ultostr_zeropad(
                str,
                (if (*tm).tm_year > 0 as libc::c_int {
                    (*tm).tm_year
                } else {
                    -((*tm).tm_year - 1 as libc::c_int)
                }) as uint32,
                4 as libc::c_int,
            );
            let fresh60 = str;
            str = str.offset(1);
            *fresh60 = '-' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(str, (*tm).tm_mon as uint32, 2 as libc::c_int);
            let fresh61 = str;
            str = str.offset(1);
            *fresh61 = '-' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(str, (*tm).tm_mday as uint32, 2 as libc::c_int);
            let fresh62 = str;
            str = str.offset(1);
            *fresh62 = (if style == 1 as libc::c_int {
                ' ' as i32
            } else {
                'T' as i32
            }) as libc::c_char;
            str = pg_ultostr_zeropad(str, (*tm).tm_hour as uint32, 2 as libc::c_int);
            let fresh63 = str;
            str = str.offset(1);
            *fresh63 = ':' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(str, (*tm).tm_min as uint32, 2 as libc::c_int);
            let fresh64 = str;
            str = str.offset(1);
            *fresh64 = ':' as i32 as libc::c_char;
            str = AppendTimestampSeconds(str, tm, fsec);
            if print_tz {
                str = EncodeTimezone(str, tz, style);
            }
        }
        2 => {
            if DateOrder == 1 as libc::c_int {
                str = pg_ultostr_zeropad(str, (*tm).tm_mday as uint32, 2 as libc::c_int);
                let fresh65 = str;
                str = str.offset(1);
                *fresh65 = '/' as i32 as libc::c_char;
                str = pg_ultostr_zeropad(str, (*tm).tm_mon as uint32, 2 as libc::c_int);
            } else {
                str = pg_ultostr_zeropad(str, (*tm).tm_mon as uint32, 2 as libc::c_int);
                let fresh66 = str;
                str = str.offset(1);
                *fresh66 = '/' as i32 as libc::c_char;
                str = pg_ultostr_zeropad(str, (*tm).tm_mday as uint32, 2 as libc::c_int);
            }
            let fresh67 = str;
            str = str.offset(1);
            *fresh67 = '/' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(
                str,
                (if (*tm).tm_year > 0 as libc::c_int {
                    (*tm).tm_year
                } else {
                    -((*tm).tm_year - 1 as libc::c_int)
                }) as uint32,
                4 as libc::c_int,
            );
            let fresh68 = str;
            str = str.offset(1);
            *fresh68 = ' ' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(str, (*tm).tm_hour as uint32, 2 as libc::c_int);
            let fresh69 = str;
            str = str.offset(1);
            *fresh69 = ':' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(str, (*tm).tm_min as uint32, 2 as libc::c_int);
            let fresh70 = str;
            str = str.offset(1);
            *fresh70 = ':' as i32 as libc::c_char;
            str = AppendTimestampSeconds(str, tm, fsec);
            if print_tz {
                if !tzn.is_null() {
                    pg_sprintf(
                        str,
                        b" %.*s\0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int,
                        tzn,
                    );
                    str = str.offset(strlen(str) as isize);
                } else {
                    str = EncodeTimezone(str, tz, style);
                }
            }
        }
        3 => {
            str = pg_ultostr_zeropad(str, (*tm).tm_mday as uint32, 2 as libc::c_int);
            let fresh71 = str;
            str = str.offset(1);
            *fresh71 = '.' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(str, (*tm).tm_mon as uint32, 2 as libc::c_int);
            let fresh72 = str;
            str = str.offset(1);
            *fresh72 = '.' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(
                str,
                (if (*tm).tm_year > 0 as libc::c_int {
                    (*tm).tm_year
                } else {
                    -((*tm).tm_year - 1 as libc::c_int)
                }) as uint32,
                4 as libc::c_int,
            );
            let fresh73 = str;
            str = str.offset(1);
            *fresh73 = ' ' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(str, (*tm).tm_hour as uint32, 2 as libc::c_int);
            let fresh74 = str;
            str = str.offset(1);
            *fresh74 = ':' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(str, (*tm).tm_min as uint32, 2 as libc::c_int);
            let fresh75 = str;
            str = str.offset(1);
            *fresh75 = ':' as i32 as libc::c_char;
            str = AppendTimestampSeconds(str, tm, fsec);
            if print_tz {
                if !tzn.is_null() {
                    pg_sprintf(
                        str,
                        b" %.*s\0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int,
                        tzn,
                    );
                    str = str.offset(strlen(str) as isize);
                } else {
                    str = EncodeTimezone(str, tz, style);
                }
            }
        }
        0 | _ => {
            let day = date2j((*tm).tm_year, (*tm).tm_mon, (*tm).tm_mday);
            (*tm).tm_wday = j2day(day);
            memcpy(
                str as *mut libc::c_void,
                days[(*tm).tm_wday as usize] as *const libc::c_void,
                3 as libc::c_int as libc::c_ulong,
            );
            str = str.offset(3 as libc::c_int as isize);
            let fresh76 = str;
            str = str.offset(1);
            *fresh76 = ' ' as i32 as libc::c_char;
            if DateOrder == 1 as libc::c_int {
                str = pg_ultostr_zeropad(str, (*tm).tm_mday as uint32, 2 as libc::c_int);
                let fresh77 = str;
                str = str.offset(1);
                *fresh77 = ' ' as i32 as libc::c_char;
                memcpy(
                    str as *mut libc::c_void,
                    months[((*tm).tm_mon - 1 as libc::c_int) as usize] as *const libc::c_void,
                    3 as libc::c_int as libc::c_ulong,
                );
                str = str.offset(3 as libc::c_int as isize);
            } else {
                memcpy(
                    str as *mut libc::c_void,
                    months[((*tm).tm_mon - 1 as libc::c_int) as usize] as *const libc::c_void,
                    3 as libc::c_int as libc::c_ulong,
                );
                str = str.offset(3 as libc::c_int as isize);
                let fresh78 = str;
                str = str.offset(1);
                *fresh78 = ' ' as i32 as libc::c_char;
                str = pg_ultostr_zeropad(str, (*tm).tm_mday as uint32, 2 as libc::c_int);
            }
            let fresh79 = str;
            str = str.offset(1);
            *fresh79 = ' ' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(str, (*tm).tm_hour as uint32, 2 as libc::c_int);
            let fresh80 = str;
            str = str.offset(1);
            *fresh80 = ':' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(str, (*tm).tm_min as uint32, 2 as libc::c_int);
            let fresh81 = str;
            str = str.offset(1);
            *fresh81 = ':' as i32 as libc::c_char;
            str = AppendTimestampSeconds(str, tm, fsec);
            let fresh82 = str;
            str = str.offset(1);
            *fresh82 = ' ' as i32 as libc::c_char;
            str = pg_ultostr_zeropad(
                str,
                (if (*tm).tm_year > 0 as libc::c_int {
                    (*tm).tm_year
                } else {
                    -((*tm).tm_year - 1 as libc::c_int)
                }) as uint32,
                4 as libc::c_int,
            );
            if print_tz {
                if !tzn.is_null() {
                    pg_sprintf(
                        str,
                        b" %.*s\0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int,
                        tzn,
                    );
                    str = str.offset(strlen(str) as isize);
                } else {
                    let fresh83 = str;
                    str = str.offset(1);
                    *fresh83 = ' ' as i32 as libc::c_char;
                    str = EncodeTimezone(str, tz, style);
                }
            }
        }
    }
    if (*tm).tm_year <= 0 as libc::c_int {
        memcpy(
            str as *mut libc::c_void,
            b" BC\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        );
        str = str.offset(3 as libc::c_int as isize);
    }
    *str = '\0' as i32 as libc::c_char;
}
unsafe fn AddISO8601IntPart(
    cp: *mut libc::c_char,
    value: libc::c_int,
    units: libc::c_char,
) -> *mut libc::c_char {
    if value == 0 as libc::c_int {
        return cp;
    }
    pg_sprintf(
        cp,
        b"%d%c\0" as *const u8 as *const libc::c_char,
        value,
        units as libc::c_int,
    );
    return cp.offset(strlen(cp) as isize);
}
unsafe fn AddPostgresIntPart(
    cp: *mut libc::c_char,
    value: libc::c_int,
    units: *const libc::c_char,
    is_zero: &mut bool,
    is_before: &mut bool,
) -> *mut libc::c_char {
    if value == 0 as libc::c_int {
        return cp;
    }
    pg_sprintf(
        cp,
        b"%s%s%d %s%s\0" as *const u8 as *const libc::c_char,
        if !*is_zero {
            b" \0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if *is_before as libc::c_int != 0 && value > 0 as libc::c_int {
            b"+\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        value,
        units,
        if value != 1 as libc::c_int {
            b"s\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    *is_before = value < 0;
    *is_zero = false;
    return cp.offset(strlen(cp) as isize);
}
unsafe fn AddVerboseIntPart(
    cp: *mut libc::c_char,
    mut value: libc::c_int,
    units: *const libc::c_char,
    is_zero: &mut bool,
    is_before: &mut bool,
) -> *mut libc::c_char {
    if value == 0 as libc::c_int {
        return cp;
    }
    if *is_zero {
        *is_before = value < 0;
        value = abs(value);
    } else if *is_before {
        value = -value;
    }
    pg_sprintf(
        cp,
        b" %d %s%s\0" as *const u8 as *const libc::c_char,
        value,
        units,
        if value == 1 as libc::c_int {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"s\0" as *const u8 as *const libc::c_char
        },
    );
    *is_zero = false;
    return cp.offset(strlen(cp) as isize);
}

unsafe fn EncodeInterval(
    tm: *mut pg_tm,
    mut fsec: fsec_t,
    style: libc::c_int,
    str: *mut libc::c_char,
) {
    let mut cp: *mut libc::c_char = str;
    let mut year: libc::c_int = (*tm).tm_year;
    let mut mon: libc::c_int = (*tm).tm_mon;
    let mut mday: libc::c_int = (*tm).tm_mday;
    let mut hour: libc::c_int = (*tm).tm_hour;
    let mut min: libc::c_int = (*tm).tm_min;
    let mut sec: libc::c_int = (*tm).tm_sec;
    let mut is_before: bool = false;
    let mut is_zero: bool = true;
    match style {
        2 => {
            let has_negative =
                year < 0 || mon < 0 || mday < 0 || hour < 0 || min < 0 || sec < 0 || fsec < 0;
            let has_positive =
                year > 0 || mon > 0 || mday > 0 || hour > 0 || min > 0 || sec > 0 || fsec > 0;
            let has_year_month = year != 0 || mon != 0;
            let has_day_time = mday != 0 || hour != 0 || min != 0 || sec != 0 || fsec != 0;
            let has_day = mday != 0;
            let sql_standard_value =
                !(has_negative && has_positive) && !(has_year_month && has_day_time);
            if has_negative as libc::c_int != 0 && sql_standard_value as libc::c_int != 0 {
                let fresh84 = cp;
                cp = cp.offset(1);
                *fresh84 = '-' as i32 as libc::c_char;
                year = -year;
                mon = -mon;
                mday = -mday;
                hour = -hour;
                min = -min;
                sec = -sec;
                fsec = -fsec;
            }
            if !has_negative && !has_positive {
                pg_sprintf(cp, b"0\0" as *const u8 as *const libc::c_char);
            } else if !sql_standard_value {
                let year_sign: libc::c_char = (if year < 0 as libc::c_int || mon < 0 as libc::c_int
                {
                    '-' as i32
                } else {
                    '+' as i32
                }) as libc::c_char;
                let day_sign: libc::c_char = (if mday < 0 as libc::c_int {
                    '-' as i32
                } else {
                    '+' as i32
                }) as libc::c_char;
                let sec_sign: libc::c_char = (if hour < 0 as libc::c_int
                    || min < 0 as libc::c_int
                    || sec < 0 as libc::c_int
                    || fsec < 0 as libc::c_int
                {
                    '-' as i32
                } else {
                    '+' as i32
                }) as libc::c_char;
                pg_sprintf(
                    cp,
                    b"%c%d-%d %c%d %c%d:%02d:\0" as *const u8 as *const libc::c_char,
                    year_sign as libc::c_int,
                    abs(year),
                    abs(mon),
                    day_sign as libc::c_int,
                    abs(mday),
                    sec_sign as libc::c_int,
                    abs(hour),
                    abs(min),
                );
                cp = cp.offset(strlen(cp) as isize);
                cp = AppendSeconds(cp, sec, fsec, 6 as libc::c_int, true);
                *cp = '\0' as i32 as libc::c_char;
            } else if has_year_month {
                pg_sprintf(
                    cp,
                    b"%d-%d\0" as *const u8 as *const libc::c_char,
                    year,
                    mon,
                );
            } else if has_day {
                pg_sprintf(
                    cp,
                    b"%d %d:%02d:\0" as *const u8 as *const libc::c_char,
                    mday,
                    hour,
                    min,
                );
                cp = cp.offset(strlen(cp) as isize);
                cp = AppendSeconds(cp, sec, fsec, 6 as libc::c_int, true);
                *cp = '\0' as i32 as libc::c_char;
            } else {
                pg_sprintf(
                    cp,
                    b"%d:%02d:\0" as *const u8 as *const libc::c_char,
                    hour,
                    min,
                );
                cp = cp.offset(strlen(cp) as isize);
                cp = AppendSeconds(cp, sec, fsec, 6 as libc::c_int, true);
                *cp = '\0' as i32 as libc::c_char;
            }
        }
        3 => {
            if year == 0 as libc::c_int
                && mon == 0 as libc::c_int
                && mday == 0 as libc::c_int
                && hour == 0 as libc::c_int
                && min == 0 as libc::c_int
                && sec == 0 as libc::c_int
                && fsec == 0 as libc::c_int
            {
                pg_sprintf(cp, b"PT0S\0" as *const u8 as *const libc::c_char);
            } else {
                let fresh85 = cp;
                cp = cp.offset(1);
                *fresh85 = 'P' as i32 as libc::c_char;
                cp = AddISO8601IntPart(cp, year, 'Y' as i32 as libc::c_char);
                cp = AddISO8601IntPart(cp, mon, 'M' as i32 as libc::c_char);
                cp = AddISO8601IntPart(cp, mday, 'D' as i32 as libc::c_char);
                if hour != 0 as libc::c_int
                    || min != 0 as libc::c_int
                    || sec != 0 as libc::c_int
                    || fsec != 0 as libc::c_int
                {
                    let fresh86 = cp;
                    cp = cp.offset(1);
                    *fresh86 = 'T' as i32 as libc::c_char;
                }
                cp = AddISO8601IntPart(cp, hour, 'H' as i32 as libc::c_char);
                cp = AddISO8601IntPart(cp, min, 'M' as i32 as libc::c_char);
                if sec != 0 as libc::c_int || fsec != 0 as libc::c_int {
                    if sec < 0 as libc::c_int || fsec < 0 as libc::c_int {
                        let fresh87 = cp;
                        cp = cp.offset(1);
                        *fresh87 = '-' as i32 as libc::c_char;
                    }
                    cp = AppendSeconds(cp, sec, fsec, 6 as libc::c_int, false);
                    let fresh88 = cp;
                    cp = cp.offset(1);
                    *fresh88 = 'S' as i32 as libc::c_char;
                    let fresh89 = cp;
                    #[allow(unused_assignments)]
                    {
                        cp = cp.offset(1);
                    }
                    *fresh89 = '\0' as i32 as libc::c_char;
                }
            }
        }
        0 => {
            cp = AddPostgresIntPart(
                cp,
                year,
                b"year\0" as *const u8 as *const libc::c_char,
                &mut is_zero,
                &mut is_before,
            );
            cp = AddPostgresIntPart(
                cp,
                mon,
                b"mon\0" as *const u8 as *const libc::c_char,
                &mut is_zero,
                &mut is_before,
            );
            cp = AddPostgresIntPart(
                cp,
                mday,
                b"day\0" as *const u8 as *const libc::c_char,
                &mut is_zero,
                &mut is_before,
            );
            if is_zero as libc::c_int != 0
                || hour != 0 as libc::c_int
                || min != 0 as libc::c_int
                || sec != 0 as libc::c_int
                || fsec != 0 as libc::c_int
            {
                let minus: bool = hour < 0 || min < 0 || sec < 0 || fsec < 0;
                pg_sprintf(
                    cp,
                    b"%s%s%02d:%02d:\0" as *const u8 as *const libc::c_char,
                    if is_zero as libc::c_int != 0 {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b" \0" as *const u8 as *const libc::c_char
                    },
                    if minus as libc::c_int != 0 {
                        b"-\0" as *const u8 as *const libc::c_char
                    } else if is_before as libc::c_int != 0 {
                        b"+\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    abs(hour),
                    abs(min),
                );
                cp = cp.offset(strlen(cp) as isize);
                cp = AppendSeconds(cp, sec, fsec, 6 as libc::c_int, true);
                *cp = '\0' as i32 as libc::c_char;
            }
        }
        1 | _ => {
            strcpy(cp, b"@\0" as *const u8 as *const libc::c_char);
            cp = cp.offset(1);
            cp = AddVerboseIntPart(
                cp,
                year,
                b"year\0" as *const u8 as *const libc::c_char,
                &mut is_zero,
                &mut is_before,
            );
            cp = AddVerboseIntPart(
                cp,
                mon,
                b"mon\0" as *const u8 as *const libc::c_char,
                &mut is_zero,
                &mut is_before,
            );
            cp = AddVerboseIntPart(
                cp,
                mday,
                b"day\0" as *const u8 as *const libc::c_char,
                &mut is_zero,
                &mut is_before,
            );
            cp = AddVerboseIntPart(
                cp,
                hour,
                b"hour\0" as *const u8 as *const libc::c_char,
                &mut is_zero,
                &mut is_before,
            );
            cp = AddVerboseIntPart(
                cp,
                min,
                b"min\0" as *const u8 as *const libc::c_char,
                &mut is_zero,
                &mut is_before,
            );
            if sec != 0 as libc::c_int || fsec != 0 as libc::c_int {
                let fresh90 = cp;
                cp = cp.offset(1);
                *fresh90 = ' ' as i32 as libc::c_char;
                if sec < 0 as libc::c_int || sec == 0 as libc::c_int && fsec < 0 as libc::c_int {
                    if is_zero {
                        is_before = true;
                    } else if !is_before {
                        let fresh91 = cp;
                        cp = cp.offset(1);
                        *fresh91 = '-' as i32 as libc::c_char;
                    }
                } else if is_before {
                    let fresh92 = cp;
                    cp = cp.offset(1);
                    *fresh92 = '-' as i32 as libc::c_char;
                }
                cp = AppendSeconds(cp, sec, fsec, 6 as libc::c_int, false);
                pg_sprintf(
                    cp,
                    b" sec%s\0" as *const u8 as *const libc::c_char,
                    if abs(sec) != 1 as libc::c_int || fsec != 0 as libc::c_int {
                        b"s\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                );
                is_zero = false;
            }
            if is_zero {
                strcat(cp, b" 0\0" as *const u8 as *const libc::c_char);
            }
            if is_before {
                strcat(cp, b" ago\0" as *const u8 as *const libc::c_char);
            }
        }
    };
}
unsafe fn CheckDateTokenTable(
    tablename: *const libc::c_char,
    base: *const datetkn,
    nel: libc::c_int,
) -> bool {
    let mut ok = true;
    let mut i: libc::c_int = 0;
    while i < nel {
        if strlen(((*base.offset(i as isize)).token).as_ptr()) > 10 as libc::c_int as libc::c_ulong
        {
            let mut __errno_location_0: libc::c_int = 0;
            if if 0 != 0 && 15 as libc::c_int >= 21 as libc::c_int {
                errstart_cold(15 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
            } else {
                errstart(15 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
            } != 0
            {
                errmsg_internal(
                    b"token too long in %s table: \"%.*s\"\0" as *const u8 as *const libc::c_char,
                    tablename,
                    10 as libc::c_int + 1 as libc::c_int,
                    ((*base.offset(i as isize)).token).as_ptr(),
                );
                errfinish(
                    b"/home/petrosagg/projects/postgres-datetime/src/datetime.c\0" as *const u8
                        as *const libc::c_char,
                    4446 as libc::c_int,
                    (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                        b"CheckDateTokenTable\0",
                    ))
                    .as_ptr(),
                );
            }
            if 0 != 0 && 15 as libc::c_int >= 21 as libc::c_int {
                unreachable!();
            }
            ok = false;
            break;
        } else {
            if i > 0 as libc::c_int
                && strcmp(
                    ((*base.offset((i - 1 as libc::c_int) as isize)).token).as_ptr(),
                    ((*base.offset(i as isize)).token).as_ptr(),
                ) >= 0 as libc::c_int
            {
                let mut __errno_location_1: libc::c_int = 0;
                if if 0 != 0 && 15 as libc::c_int >= 21 as libc::c_int {
                    errstart_cold(15 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
                } else {
                    errstart(15 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
                } != 0
                {
                    errmsg_internal(
                        b"ordering error in %s table: \"%s\" >= \"%s\"\0" as *const u8
                            as *const libc::c_char,
                        tablename,
                        ((*base.offset((i - 1 as libc::c_int) as isize)).token).as_ptr(),
                        ((*base.offset(i as isize)).token).as_ptr(),
                    );
                    errfinish(
                        b"/home/petrosagg/projects/postgres-datetime/src/datetime.c\0" as *const u8
                            as *const libc::c_char,
                        4457 as libc::c_int,
                        (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                            b"CheckDateTokenTable\0",
                        ))
                        .as_ptr(),
                    );
                }
                if 0 != 0 && 15 as libc::c_int >= 21 as libc::c_int {
                    unreachable!();
                }
                ok = false;
            }
            i += 1;
        }
    }
    return ok;
}

unsafe fn CheckDateTokenTables() -> bool {
    let mut ok = true;
    ok = ok
        && CheckDateTokenTable(
            b"datetktbl\0" as *const u8 as *const libc::c_char,
            datetktbl.as_ptr(),
            szdatetktbl,
        );
    ok = ok
        && CheckDateTokenTable(
            b"deltatktbl\0" as *const u8 as *const libc::c_char,
            deltatktbl.as_ptr(),
            szdeltatktbl,
        );
    return ok;
}

unsafe fn TemporalSimplify(max_precis: int32, node: *mut Node) -> *mut Node {
    let expr: *mut FuncExpr = node as *mut FuncExpr;
    let mut ret: *mut Node = 0 as *mut Node;
    let typmod = (*list_nth_cell((*expr).args, 1 as libc::c_int)).ptr_value as *mut Node;
    if (*(typmod as *const Node)).type_0 as libc::c_uint == T_Const as libc::c_int as libc::c_uint
        && !(*(typmod as *mut Const)).constisnull
    {
        let source: *mut Node =
            (*list_nth_cell((*expr).args, 0 as libc::c_int)).ptr_value as *mut Node;
        let old_precis: int32 = exprTypmod(source);
        let new_precis: int32 = (*(typmod as *mut Const)).constvalue as int32;
        if new_precis < 0 as libc::c_int
            || new_precis == max_precis
            || old_precis >= 0 as libc::c_int && new_precis >= old_precis
        {
            ret = relabel_to_typmod(source, new_precis);
        }
    }
    return ret;
}

unsafe fn ConvertTimeZoneAbbrevs(
    abbrevs: *mut tzEntry,
    n: libc::c_int,
) -> *mut TimeZoneAbbrevTable {
    let mut tbl_size = (12 as libc::c_ulong).wrapping_add(
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<datetkn>() as libc::c_ulong),
    );
    tbl_size = tbl_size.wrapping_add((8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
        & !((8 as libc::c_int - 1 as libc::c_int) as uintptr_t);
    for i in 0..n {
        let abbr: *mut tzEntry = abbrevs.offset(i as isize);
        if !((*abbr).zone).is_null() {
            let dsize = (8 as libc::c_ulong)
                .wrapping_add(strlen((*abbr).zone))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            tbl_size = (tbl_size as libc::c_ulong).wrapping_add(
                dsize.wrapping_add((8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                    & !((8 as libc::c_int - 1 as libc::c_int) as uintptr_t),
            ) as Size as Size;
        }
    }
    let mut tbl = malloc(tbl_size) as *mut TimeZoneAbbrevTable;
    if tbl.is_null() {
        return 0 as *mut TimeZoneAbbrevTable;
    }
    (*tbl).tblsize = tbl_size;
    (*tbl).numabbrevs = n;
    tbl_size = (12 as libc::c_ulong).wrapping_add(
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<datetkn>() as libc::c_ulong),
    );
    tbl_size = tbl_size.wrapping_add((8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
        & !((8 as libc::c_int - 1 as libc::c_int) as uintptr_t);
    for i in 0..n {
        let abbr_0: *mut tzEntry = abbrevs.offset(i as isize);
        let mut dtoken: *mut datetkn = ((*tbl).abbrevs).as_mut_ptr().offset(i as isize);
        strlcpy(
            ((*dtoken).token).as_mut_ptr(),
            (*abbr_0).abbrev,
            (10 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        );
        if !((*abbr_0).zone).is_null() {
            let dtza =
                (tbl as *mut libc::c_char).offset(tbl_size as isize) as *mut DynamicZoneAbbrev;
            (*dtza).tz = 0 as *mut pg_tz;
            strcpy(((*dtza).zone).as_mut_ptr(), (*abbr_0).zone);
            (*dtoken).type_0 = RealFieldType::DynTz;
            (*dtoken).value = tbl_size as int32;
            let dsize_0 = (8 as libc::c_ulong)
                .wrapping_add(strlen((*abbr_0).zone))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            tbl_size = (tbl_size as libc::c_ulong).wrapping_add(
                dsize_0.wrapping_add((8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                    & !((8 as libc::c_int - 1 as libc::c_int) as uintptr_t),
            ) as Size as Size;
        } else {
            (*dtoken).type_0 = if (*abbr_0).is_dst as libc::c_int != 0 {
                RealFieldType::DTz
            } else {
                RealFieldType::Tz
            };
            (*dtoken).value = (*abbr_0).offset;
        }
    }
    return tbl;
}

unsafe fn InstallTimeZoneAbbrevs(tbl: *mut TimeZoneAbbrevTable) {
    zoneabbrevtbl = tbl;
    memset(
        abbrevcache.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[*const datetkn; 25]>() as libc::c_ulong,
    );
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

unsafe fn pg_timezone_abbrevs(mut fcinfo: FunctionCallInfo) -> Datum {
    let mut funcctx: *mut FuncCallContext;
    let mut pindex: *mut libc::c_int;
    let result: Datum;
    let tuple: HeapTuple;
    let mut values: [Datum; 3] = [0; 3];
    let mut nulls: [bool; 3] = [false; 3];
    let tp: *const datetkn;
    let mut buffer: [libc::c_char; 11] = [0; 11];
    let gmtoffset: libc::c_int;
    let is_dst;
    let mut p: *mut libc::c_uchar;
    let mut tm: pg_tm = pg_tm {
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
    if ((*(*fcinfo).flinfo).fn_extra).is_null() {
        funcctx = init_MultiFuncCall(fcinfo);
        let oldcontext = MemoryContextSwitchTo((*funcctx).multi_call_memory_ctx);
        pindex = palloc(::core::mem::size_of::<libc::c_int>() as libc::c_ulong) as *mut libc::c_int;
        *pindex = 0 as libc::c_int;
        (*funcctx).user_fctx = pindex as *mut libc::c_void;
        let tupdesc = CreateTemplateTupleDesc(3 as libc::c_int);
        TupleDescInitEntry(
            tupdesc,
            1 as libc::c_int as AttrNumber,
            b"abbrev\0" as *const u8 as *const libc::c_char,
            25 as libc::c_int as Oid,
            -(1 as libc::c_int),
            0 as libc::c_int,
        );
        TupleDescInitEntry(
            tupdesc,
            2 as libc::c_int as AttrNumber,
            b"utc_offset\0" as *const u8 as *const libc::c_char,
            1186 as libc::c_int as Oid,
            -(1 as libc::c_int),
            0 as libc::c_int,
        );
        TupleDescInitEntry(
            tupdesc,
            3 as libc::c_int as AttrNumber,
            b"is_dst\0" as *const u8 as *const libc::c_char,
            16 as libc::c_int as Oid,
            -(1 as libc::c_int),
            0 as libc::c_int,
        );
        (*funcctx).tuple_desc = BlessTupleDesc(tupdesc);
        MemoryContextSwitchTo(oldcontext);
    }
    funcctx = per_MultiFuncCall(fcinfo);
    pindex = (*funcctx).user_fctx as *mut libc::c_int;
    if zoneabbrevtbl.is_null() || *pindex >= (*zoneabbrevtbl).numabbrevs {
        end_MultiFuncCall(fcinfo, funcctx);
        let rsi = (*fcinfo).resultinfo as *mut ReturnSetInfo;
        (*rsi).isDone = ExprEndResult;
        (*fcinfo).isnull = true;
        return 0 as libc::c_int as Datum;
    }
    tp = ((*zoneabbrevtbl).abbrevs)
        .as_mut_ptr()
        .offset(*pindex as isize);
    match (*tp).type_0 as libc::c_int {
        5 => {
            gmtoffset = (*tp).value;
            is_dst = false;
        }
        6 => {
            gmtoffset = (*tp).value;
            is_dst = true;
        }
        7 => {
            let mut isdst = false;
            let tzp = FetchDynamicTimeZone(zoneabbrevtbl, tp);
            let now = GetCurrentTransactionStartTimestamp();
            gmtoffset =
                -DetermineTimeZoneAbbrevOffsetTS(now, ((*tp).token).as_ptr(), tzp, &mut isdst);
            is_dst = isdst;
        }
        _ => {
            let mut __errno_location_0: libc::c_int = 0;
            if if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
                errstart_cold(21 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
            } else {
                errstart(21 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
            } != 0
            {
                errmsg_internal(
                    b"unrecognized timezone type %d\0" as *const u8 as *const libc::c_char,
                    (*tp).type_0 as libc::c_int,
                );
                errfinish(
                    b"/home/petrosagg/projects/postgres-datetime/src/datetime.c\0" as *const u8
                        as *const libc::c_char,
                    4746 as libc::c_int,
                    (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                        b"pg_timezone_abbrevs\0",
                    ))
                    .as_ptr(),
                );
            }
            if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
                unreachable!();
            }
            gmtoffset = 0 as libc::c_int;
            is_dst = false;
        }
    }
    let mut _vstart: *mut libc::c_void = nulls.as_mut_ptr() as *mut libc::c_void;
    let mut _val: libc::c_int = 0 as libc::c_int;
    let mut _len: Size = ::core::mem::size_of::<[bool; 3]>() as libc::c_ulong;
    if _vstart as uintptr_t
        & (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
        && _len
            & (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        && _val == 0 as libc::c_int
        && _len <= 1024 as libc::c_int as libc::c_ulong
        && 1024 as libc::c_int != 0 as libc::c_int
    {
        let mut _start: *mut libc::c_long = _vstart as *mut libc::c_long;
        let mut _stop: *mut libc::c_long =
            (_start as *mut libc::c_char).offset(_len as isize) as *mut libc::c_long;
        while _start < _stop {
            let fresh93 = _start;
            _start = _start.offset(1);
            *fresh93 = 0 as libc::c_int as libc::c_long;
        }
    } else {
        memset(_vstart, _val, _len);
    }
    strlcpy(
        buffer.as_mut_ptr(),
        ((*tp).token).as_ptr(),
        ::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong,
    );
    p = buffer.as_mut_ptr() as *mut libc::c_uchar;
    while *p != 0 {
        *p = pg_toupper(*p);
        p = p.offset(1);
    }
    values[0 as libc::c_int as usize] = cstring_to_text(buffer.as_mut_ptr()) as Datum;
    let mut _vstart_0: *mut libc::c_void = &mut tm as *mut pg_tm as *mut libc::c_void;
    let mut _val_0: libc::c_int = 0 as libc::c_int;
    let mut _len_0: Size = ::core::mem::size_of::<pg_tm>() as libc::c_ulong;
    if _vstart_0 as uintptr_t
        & (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
        && _len_0
            & (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        && _val_0 == 0 as libc::c_int
        && _len_0 <= 1024 as libc::c_int as libc::c_ulong
        && 1024 as libc::c_int != 0 as libc::c_int
    {
        let mut _start_0: *mut libc::c_long = _vstart_0 as *mut libc::c_long;
        let mut _stop_0: *mut libc::c_long =
            (_start_0 as *mut libc::c_char).offset(_len_0 as isize) as *mut libc::c_long;
        while _start_0 < _stop_0 {
            let fresh94 = _start_0;
            _start_0 = _start_0.offset(1);
            *fresh94 = 0 as libc::c_int as libc::c_long;
        }
    } else {
        memset(_vstart_0, _val_0, _len_0);
    }
    tm.tm_sec = gmtoffset;
    let resInterval = palloc(::core::mem::size_of::<Interval>() as libc::c_ulong) as *mut Interval;
    tm2interval(&mut tm, 0 as libc::c_int, resInterval);
    values[1 as libc::c_int as usize] = resInterval as Datum;
    values[2 as libc::c_int as usize] = (if is_dst as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as Datum;
    *pindex += 1;
    tuple = heap_form_tuple(
        (*funcctx).tuple_desc,
        values.as_mut_ptr(),
        nulls.as_mut_slice(),
    );
    result = HeapTupleHeaderGetDatum((*tuple).t_data);
    (*funcctx).call_cntr = ((*funcctx).call_cntr).wrapping_add(1);
    let rsi_0 = (*fcinfo).resultinfo as *mut ReturnSetInfo;
    (*rsi_0).isDone = ExprMultipleResult;
    return result;
}

unsafe fn pg_timezone_names(fcinfo: FunctionCallInfo) -> Datum {
    let mut rsinfo: *mut ReturnSetInfo = (*fcinfo).resultinfo as *mut ReturnSetInfo;
    let mut tupdesc: TupleDesc = 0 as *mut TupleDescData;
    let mut tz: *mut pg_tz;
    let mut values: [Datum; 4] = [0; 4];
    let mut nulls: [bool; 4] = [false; 4];
    let mut tzoff: libc::c_int = 0;
    let mut tm: pg_tm = pg_tm {
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
    let mut fsec: fsec_t = 0;
    let mut tzn: *const libc::c_char = 0 as *const libc::c_char;
    let mut resInterval: *mut Interval;
    let mut itm: pg_tm = pg_tm {
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
    if rsinfo.is_null()
        || !((*(rsinfo as *const Node)).type_0 as libc::c_uint
            == T_ReturnSetInfo as libc::c_int as libc::c_uint)
    {
        let mut __errno_location_0: libc::c_int = 0;
        if if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
            errstart_cold(21 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
        } else {
            errstart(21 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
        } != 0
        {
            errcode(
                ('0' as i32 - '0' as i32 & 0x3f as libc::c_int)
                    + (('A' as i32 - '0' as i32 & 0x3f as libc::c_int) << 6 as libc::c_int)
                    + (('0' as i32 - '0' as i32 & 0x3f as libc::c_int) << 12 as libc::c_int)
                    + (('0' as i32 - '0' as i32 & 0x3f as libc::c_int) << 18 as libc::c_int)
                    + (('0' as i32 - '0' as i32 & 0x3f as libc::c_int) << 24 as libc::c_int),
            );
            errmsg0(
                b"set-valued function called in context that cannot accept a set\0" as *const u8
                    as *const libc::c_char,
            );
            errfinish(
                b"/home/petrosagg/projects/postgres-datetime/src/datetime.c\0" as *const u8
                    as *const libc::c_char,
                4808 as libc::c_int,
                (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"pg_timezone_names\0"))
                    .as_ptr(),
            );
        }
        if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
            unreachable!();
        }
    }
    if (*rsinfo).allowedModes & SFRM_Materialize as libc::c_int == 0 {
        let mut __errno_location_1: libc::c_int = 0;
        if if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
            errstart_cold(21 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
        } else {
            errstart(21 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
        } != 0
        {
            errcode(
                ('4' as i32 - '0' as i32 & 0x3f as libc::c_int)
                    + (('2' as i32 - '0' as i32 & 0x3f as libc::c_int) << 6 as libc::c_int)
                    + (('6' as i32 - '0' as i32 & 0x3f as libc::c_int) << 12 as libc::c_int)
                    + (('0' as i32 - '0' as i32 & 0x3f as libc::c_int) << 18 as libc::c_int)
                    + (('1' as i32 - '0' as i32 & 0x3f as libc::c_int) << 24 as libc::c_int),
            );
            errmsg0(
                b"materialize mode required, but it is not allowed in this context\0" as *const u8
                    as *const libc::c_char,
            );
            errfinish(
                b"/home/petrosagg/projects/postgres-datetime/src/datetime.c\0" as *const u8
                    as *const libc::c_char,
                4812 as libc::c_int,
                (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"pg_timezone_names\0"))
                    .as_ptr(),
            );
        }
        if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
            unreachable!();
        }
    }
    let oldcontext = MemoryContextSwitchTo((*(*rsinfo).econtext).ecxt_per_query_memory);
    if get_call_result_type(fcinfo, 0 as *mut Oid, &mut tupdesc) as libc::c_uint
        != TYPEFUNC_COMPOSITE as libc::c_int as libc::c_uint
    {
        let mut __errno_location_2: libc::c_int = 0;
        if if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
            errstart_cold(21 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
        } else {
            errstart(21 as libc::c_int, 0 as *const libc::c_char) as libc::c_int
        } != 0
        {
            errmsg_internal(
                b"return type must be a row type\0" as *const u8 as *const libc::c_char,
            );
            errfinish(
                b"/home/petrosagg/projects/postgres-datetime/src/datetime.c\0" as *const u8
                    as *const libc::c_char,
                4818 as libc::c_int,
                (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"pg_timezone_names\0"))
                    .as_ptr(),
            );
        }
        if 0 != 0 && 21 as libc::c_int >= 21 as libc::c_int {
            unreachable!();
        }
    }
    let randomAccess = (*rsinfo).allowedModes & SFRM_Materialize_Random as libc::c_int != 0;
    let tupstore = tuplestore_begin_heap(randomAccess, false, work_mem);
    (*rsinfo).returnMode = SFRM_Materialize;
    (*rsinfo).setResult = tupstore;
    (*rsinfo).setDesc = tupdesc;
    MemoryContextSwitchTo(oldcontext);
    let tzenum = pg_tzenumerate_start();
    loop {
        tz = pg_tzenumerate_next(tzenum);
        if tz.is_null() {
            break;
        }
        if timestamp2tm(
            GetCurrentTransactionStartTimestamp(),
            &mut tzoff,
            &mut tm,
            &mut fsec,
            &mut tzn,
            tz,
        ) != 0 as libc::c_int
        {
            continue;
        }
        if !tzn.is_null() && strlen(tzn) > 31 as libc::c_int as libc::c_ulong {
            continue;
        }
        let mut _vstart: *mut libc::c_void = nulls.as_mut_ptr() as *mut libc::c_void;
        let mut _val: libc::c_int = 0 as libc::c_int;
        let mut _len: Size = ::core::mem::size_of::<[bool; 4]>() as libc::c_ulong;
        if _vstart as uintptr_t
            & (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
            && _len
                & (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                == 0 as libc::c_int as libc::c_ulong
            && _val == 0 as libc::c_int
            && _len <= 1024 as libc::c_int as libc::c_ulong
            && 1024 as libc::c_int != 0 as libc::c_int
        {
            let mut _start: *mut libc::c_long = _vstart as *mut libc::c_long;
            let mut _stop: *mut libc::c_long =
                (_start as *mut libc::c_char).offset(_len as isize) as *mut libc::c_long;
            while _start < _stop {
                let fresh95 = _start;
                _start = _start.offset(1);
                *fresh95 = 0 as libc::c_int as libc::c_long;
            }
        } else {
            memset(_vstart, _val, _len);
        }
        values[0 as libc::c_int as usize] = cstring_to_text(pg_get_timezone_name(tz)) as Datum;
        values[1 as libc::c_int as usize] = cstring_to_text(if !tzn.is_null() {
            tzn
        } else {
            b"\0" as *const u8 as *const libc::c_char
        }) as Datum;
        let mut _vstart_0: *mut libc::c_void = &mut itm as *mut pg_tm as *mut libc::c_void;
        let mut _val_0: libc::c_int = 0 as libc::c_int;
        let mut _len_0: Size = ::core::mem::size_of::<pg_tm>() as libc::c_ulong;
        if _vstart_0 as uintptr_t
            & (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
            && _len_0
                & (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                == 0 as libc::c_int as libc::c_ulong
            && _val_0 == 0 as libc::c_int
            && _len_0 <= 1024 as libc::c_int as libc::c_ulong
            && 1024 as libc::c_int != 0 as libc::c_int
        {
            let mut _start_0: *mut libc::c_long = _vstart_0 as *mut libc::c_long;
            let mut _stop_0: *mut libc::c_long =
                (_start_0 as *mut libc::c_char).offset(_len_0 as isize) as *mut libc::c_long;
            while _start_0 < _stop_0 {
                let fresh96 = _start_0;
                _start_0 = _start_0.offset(1);
                *fresh96 = 0 as libc::c_int as libc::c_long;
            }
        } else {
            memset(_vstart_0, _val_0, _len_0);
        }
        itm.tm_sec = -tzoff;
        resInterval = palloc(::core::mem::size_of::<Interval>() as libc::c_ulong) as *mut Interval;
        tm2interval(&mut itm, 0 as libc::c_int, resInterval);
        values[2 as libc::c_int as usize] = resInterval as Datum;
        values[3 as libc::c_int as usize] = if tm.tm_isdst.is_some() {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } as Datum;
        tuplestore_putvalues(tupstore, tupdesc, values.as_mut_ptr(), nulls.as_mut_slice());
    }
    pg_tzenumerate_end(tzenum);
    return 0 as libc::c_int as Datum;
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
