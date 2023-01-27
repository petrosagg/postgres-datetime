use ::libc;

use crate::datetime::FieldType;

const HOURS_PER_DAY: libc::c_int = 24;
const MINS_PER_HOUR: libc::c_int = 60;
const SECS_PER_DAY: libc::c_int = 86400;
const SECS_PER_MINUTE: libc::c_int = 60;
const USECS_PER_DAY: libc::c_long = 86400000000;
const USECS_PER_HOUR: libc::c_long = 3600000000;
const USECS_PER_MINUTE: libc::c_long = 60000000;
const USECS_PER_SEC: libc::c_long = 1000000;
const POSTGRES_EPOCH_JDATE: libc::c_long = 2451545; /* == date2j(2000, 1, 1) */
const UNIX_EPOCH_JDATE: libc::c_long = 2440588; /* == date2j(1970, 1, 1) */

fn pg_tolower(mut ch: libc::c_uchar) -> libc::c_uchar {
    ch.make_ascii_lowercase();
    ch
}
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
fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool_0 {
    0
}
fn errstart_cold(elevel: libc::c_int, domain: *const libc::c_char) -> bool_0 {
    0
}
fn errfinish(filename: *const libc::c_char, lineno: libc::c_int, funcname: *const libc::c_char) {}
fn errcode(sqlerrcode: libc::c_int) -> libc::c_int {
    0
}
fn errmsg0(fmt: *const libc::c_char) -> libc::c_int {
    0
}
fn errmsg(fmt: *const libc::c_char, arg: *mut libc::c_void) -> libc::c_int {
    0
}
fn errmsg2(
    fmt: *const libc::c_char,
    arg1: *mut libc::c_void,
    arg2: *mut libc::c_void,
) -> libc::c_int {
    0
}
fn errdetail(fmt: *const libc::c_char, arg: *mut libc::c_void) -> libc::c_int {
    0
}
fn GetCurrentTransactionStartTimestamp() -> TimestampTz {
    11223344
}

fn pg_localtime(timep: *const pg_time_t, tz: *const pg_tz) -> *mut pg_tm {
    Box::into_raw(Box::new(pg_tm {
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
        tm_zone: std::ptr::null(),
    }))
}

fn pg_interpret_timezone_abbrev(
    abbrev: *const libc::c_char,
    timep: *const pg_time_t,
    gmtoff: *mut libc::c_long,
    isdst: *mut libc::c_int,
    tz: *const pg_tz,
) -> bool_0 {
    unimplemented!()
}
fn pg_next_dst_boundary(
    timep: *const pg_time_t,
    before_gmtoff: *mut libc::c_long,
    before_isdst: *mut libc::c_int,
    boundary: *mut pg_time_t,
    after_gmtoff: *mut libc::c_long,
    after_isdst: *mut libc::c_int,
    tz: *const pg_tz,
) -> libc::c_int {
    0
}
fn pg_tzset(tzname: *const libc::c_char) -> *mut pg_tz {
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
fn time_overflows(hour: libc::c_int, min: libc::c_int, sec: libc::c_int, fsec: fsec_t) -> bool_0 {
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
        return 1;
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
        return 1;
    }

    return 0;
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
            (*tm).tm_isdst = -1;
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
            (*tm).tm_isdst = -1;
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
    pub type AttrMissing;
    pub type PartitionDirectoryData;
    pub type RelationData;
    pub type ParseState;
    pub type JitInstrumentation;
    pub type JitContext;
    pub type dsa_area;
    pub type QueryEnvironment;
    pub type CopyMultiInsertBuffer;
    pub type FdwRoutine;
    pub type GlobalVisState;
    pub type SharedJitInstrumentation;
    pub type ExprEvalStep;
    pub type Tuplestorestate;
    pub type pg_tz;
    pub type pg_tzenum;
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
        isnull: *mut bool_0,
    ) -> HeapTuple;
    fn tuplestore_begin_heap(
        randomAccess: bool_0,
        interXact: bool_0,
        maxKBytes: libc::c_int,
    ) -> *mut Tuplestorestate;
    fn tuplestore_putvalues(
        state: *mut Tuplestorestate,
        tdesc: TupleDesc,
        values: *mut Datum,
        isnull: *mut bool_0,
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
    fn pg_get_timezone_offset(tz: *const pg_tz, gmtoff: *mut libc::c_long) -> bool_0;
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

pub type Oid = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uintptr_t = libc::c_ulong;
pub type bool_0 = libc::c_uchar;
pub type int16 = libc::c_short;
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
pub type bits8 = uint8;
pub type int64 = libc::c_long;
pub type uint64 = libc::c_ulong;
pub type Size = size_t;
pub type Index = libc::c_uint;
pub type TransactionId = uint32;
pub type CommandId = uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct varlena {
    pub vl_len_: [libc::c_char; 4],
    pub vl_dat: [libc::c_char; 0],
}
pub type text = varlena;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameData {
    pub data: [libc::c_char; 64],
}
pub type NameData = nameData;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryContextData {
    pub type_0: NodeTag,
    pub isReset: bool_0,
    pub allowInCritSection: bool_0,
    pub mem_allocated: Size,
    pub methods: *const MemoryContextMethods,
    pub parent: MemoryContext,
    pub firstchild: MemoryContext,
    pub prevchild: MemoryContext,
    pub nextchild: MemoryContext,
    pub name: *const libc::c_char,
    pub ident: *const libc::c_char,
    pub reset_cbs: *mut MemoryContextCallback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryContextCallback {
    pub func: MemoryContextCallbackFunction,
    pub arg: *mut libc::c_void,
    pub next: *mut MemoryContextCallback,
}
pub type MemoryContextCallbackFunction = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type MemoryContext = *mut MemoryContextData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryContextMethods {
    pub alloc: Option<unsafe extern "C" fn(MemoryContext, Size) -> *mut libc::c_void>,
    pub free_p: Option<unsafe extern "C" fn(MemoryContext, *mut libc::c_void) -> ()>,
    pub realloc:
        Option<unsafe extern "C" fn(MemoryContext, *mut libc::c_void, Size) -> *mut libc::c_void>,
    pub reset: Option<unsafe extern "C" fn(MemoryContext) -> ()>,
    pub delete_context: Option<unsafe extern "C" fn(MemoryContext) -> ()>,
    pub get_chunk_space: Option<unsafe extern "C" fn(MemoryContext, *mut libc::c_void) -> Size>,
    pub is_empty: Option<unsafe extern "C" fn(MemoryContext) -> bool_0>,
    pub stats: Option<
        unsafe extern "C" fn(
            MemoryContext,
            MemoryStatsPrintFunc,
            *mut libc::c_void,
            *mut MemoryContextCounters,
            bool_0,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryContextCounters {
    pub nblocks: Size,
    pub freechunks: Size,
    pub totalspace: Size,
    pub freespace: Size,
}
pub type MemoryStatsPrintFunc = Option<
    unsafe extern "C" fn(MemoryContext, *mut libc::c_void, *const libc::c_char, bool_0) -> (),
>;
pub type NodeTag = libc::c_uint;
pub const T_SupportRequestIndexCondition: NodeTag = 430;
pub const T_SupportRequestRows: NodeTag = 429;
pub const T_SupportRequestCost: NodeTag = 428;
pub const T_SupportRequestSelectivity: NodeTag = 427;
pub const T_SupportRequestSimplify: NodeTag = 426;
pub const T_CallContext: NodeTag = 425;
pub const T_ForeignKeyCacheInfo: NodeTag = 424;
pub const T_TsmRoutine: NodeTag = 423;
pub const T_TableAmRoutine: NodeTag = 422;
pub const T_IndexAmRoutine: NodeTag = 421;
pub const T_FdwRoutine: NodeTag = 420;
pub const T_InlineCodeBlock: NodeTag = 419;
pub const T_TIDBitmap: NodeTag = 418;
pub const T_WindowObjectData: NodeTag = 417;
pub const T_ReturnSetInfo: NodeTag = 416;
pub const T_EventTriggerData: NodeTag = 415;
pub const T_TriggerData: NodeTag = 414;
pub const T_TimeLineHistoryCmd: NodeTag = 413;
pub const T_StartReplicationCmd: NodeTag = 412;
pub const T_ReadReplicationSlotCmd: NodeTag = 411;
pub const T_DropReplicationSlotCmd: NodeTag = 410;
pub const T_CreateReplicationSlotCmd: NodeTag = 409;
pub const T_BaseBackupCmd: NodeTag = 408;
pub const T_IdentifySystemCmd: NodeTag = 407;
pub const T_PublicationTable: NodeTag = 406;
pub const T_PublicationObjSpec: NodeTag = 405;
pub const T_VacuumRelation: NodeTag = 404;
pub const T_PartitionCmd: NodeTag = 403;
pub const T_PartitionRangeDatum: NodeTag = 402;
pub const T_PartitionBoundSpec: NodeTag = 401;
pub const T_PartitionSpec: NodeTag = 400;
pub const T_PartitionElem: NodeTag = 399;
pub const T_TriggerTransition: NodeTag = 398;
pub const T_RoleSpec: NodeTag = 397;
pub const T_CommonTableExpr: NodeTag = 396;
pub const T_CTECycleClause: NodeTag = 395;
pub const T_CTESearchClause: NodeTag = 394;
pub const T_OnConflictClause: NodeTag = 393;
pub const T_InferClause: NodeTag = 392;
pub const T_WithClause: NodeTag = 391;
pub const T_XmlSerialize: NodeTag = 390;
pub const T_RowMarkClause: NodeTag = 389;
pub const T_LockingClause: NodeTag = 388;
pub const T_FunctionParameter: NodeTag = 387;
pub const T_TableLikeClause: NodeTag = 386;
pub const T_CreateOpClassItem: NodeTag = 385;
pub const T_AccessPriv: NodeTag = 384;
pub const T_ObjectWithArgs: NodeTag = 383;
pub const T_WindowClause: NodeTag = 382;
pub const T_GroupingSet: NodeTag = 381;
pub const T_SortGroupClause: NodeTag = 380;
pub const T_WithCheckOption: NodeTag = 379;
pub const T_TableSampleClause: NodeTag = 378;
pub const T_RangeTblFunction: NodeTag = 377;
pub const T_RangeTblEntry: NodeTag = 376;
pub const T_DefElem: NodeTag = 375;
pub const T_Constraint: NodeTag = 374;
pub const T_StatsElem: NodeTag = 373;
pub const T_IndexElem: NodeTag = 372;
pub const T_ColumnDef: NodeTag = 371;
pub const T_TypeName: NodeTag = 370;
pub const T_RangeTableFuncCol: NodeTag = 369;
pub const T_RangeTableFunc: NodeTag = 368;
pub const T_RangeTableSample: NodeTag = 367;
pub const T_RangeFunction: NodeTag = 366;
pub const T_RangeSubselect: NodeTag = 365;
pub const T_WindowDef: NodeTag = 364;
pub const T_SortBy: NodeTag = 363;
pub const T_CollateClause: NodeTag = 362;
pub const T_TypeCast: NodeTag = 361;
pub const T_MultiAssignRef: NodeTag = 360;
pub const T_ResTarget: NodeTag = 359;
pub const T_A_ArrayExpr: NodeTag = 358;
pub const T_A_Indirection: NodeTag = 357;
pub const T_A_Indices: NodeTag = 356;
pub const T_A_Star: NodeTag = 355;
pub const T_FuncCall: NodeTag = 354;
pub const T_A_Const: NodeTag = 353;
pub const T_ParamRef: NodeTag = 352;
pub const T_ColumnRef: NodeTag = 351;
pub const T_A_Expr: NodeTag = 350;
pub const T_AlterStatsStmt: NodeTag = 349;
pub const T_CallStmt: NodeTag = 348;
pub const T_AlterCollationStmt: NodeTag = 347;
pub const T_CreateStatsStmt: NodeTag = 346;
pub const T_DropSubscriptionStmt: NodeTag = 345;
pub const T_AlterSubscriptionStmt: NodeTag = 344;
pub const T_CreateSubscriptionStmt: NodeTag = 343;
pub const T_AlterPublicationStmt: NodeTag = 342;
pub const T_CreatePublicationStmt: NodeTag = 341;
pub const T_CreateAmStmt: NodeTag = 340;
pub const T_CreateTransformStmt: NodeTag = 339;
pub const T_AlterPolicyStmt: NodeTag = 338;
pub const T_CreatePolicyStmt: NodeTag = 337;
pub const T_AlterSystemStmt: NodeTag = 336;
pub const T_ReplicaIdentityStmt: NodeTag = 335;
pub const T_RefreshMatViewStmt: NodeTag = 334;
pub const T_AlterEventTrigStmt: NodeTag = 333;
pub const T_CreateEventTrigStmt: NodeTag = 332;
pub const T_AlterExtensionContentsStmt: NodeTag = 331;
pub const T_AlterExtensionStmt: NodeTag = 330;
pub const T_CreateExtensionStmt: NodeTag = 329;
pub const T_ImportForeignSchemaStmt: NodeTag = 328;
pub const T_CreateForeignTableStmt: NodeTag = 327;
pub const T_SecLabelStmt: NodeTag = 326;
pub const T_AlterTableMoveAllStmt: NodeTag = 325;
pub const T_AlterTableSpaceOptionsStmt: NodeTag = 324;
pub const T_DropUserMappingStmt: NodeTag = 323;
pub const T_AlterUserMappingStmt: NodeTag = 322;
pub const T_CreateUserMappingStmt: NodeTag = 321;
pub const T_AlterForeignServerStmt: NodeTag = 320;
pub const T_CreateForeignServerStmt: NodeTag = 319;
pub const T_AlterFdwStmt: NodeTag = 318;
pub const T_CreateFdwStmt: NodeTag = 317;
pub const T_AlterTSConfigurationStmt: NodeTag = 316;
pub const T_AlterTSDictionaryStmt: NodeTag = 315;
pub const T_AlterEnumStmt: NodeTag = 314;
pub const T_CreateRangeStmt: NodeTag = 313;
pub const T_CreateEnumStmt: NodeTag = 312;
pub const T_CompositeTypeStmt: NodeTag = 311;
pub const T_ReassignOwnedStmt: NodeTag = 310;
pub const T_DropOwnedStmt: NodeTag = 309;
pub const T_AlterTypeStmt: NodeTag = 308;
pub const T_AlterOperatorStmt: NodeTag = 307;
pub const T_AlterOwnerStmt: NodeTag = 306;
pub const T_AlterObjectSchemaStmt: NodeTag = 305;
pub const T_AlterObjectDependsStmt: NodeTag = 304;
pub const T_DropTableSpaceStmt: NodeTag = 303;
pub const T_CreateTableSpaceStmt: NodeTag = 302;
pub const T_DeclareCursorStmt: NodeTag = 301;
pub const T_DeallocateStmt: NodeTag = 300;
pub const T_ExecuteStmt: NodeTag = 299;
pub const T_PrepareStmt: NodeTag = 298;
pub const T_AlterOpFamilyStmt: NodeTag = 297;
pub const T_CreateOpFamilyStmt: NodeTag = 296;
pub const T_CreateOpClassStmt: NodeTag = 295;
pub const T_CreateCastStmt: NodeTag = 294;
pub const T_CreateConversionStmt: NodeTag = 293;
pub const T_AlterRoleSetStmt: NodeTag = 292;
pub const T_AlterDatabaseSetStmt: NodeTag = 291;
pub const T_AlterDatabaseStmt: NodeTag = 290;
pub const T_CreateSchemaStmt: NodeTag = 289;
pub const T_CheckPointStmt: NodeTag = 288;
pub const T_ReindexStmt: NodeTag = 287;
pub const T_ConstraintsSetStmt: NodeTag = 286;
pub const T_LockStmt: NodeTag = 285;
pub const T_DropRoleStmt: NodeTag = 284;
pub const T_AlterRoleStmt: NodeTag = 283;
pub const T_CreateRoleStmt: NodeTag = 282;
pub const T_CreatePLangStmt: NodeTag = 281;
pub const T_CreateTrigStmt: NodeTag = 280;
pub const T_DiscardStmt: NodeTag = 279;
pub const T_VariableShowStmt: NodeTag = 278;
pub const T_VariableSetStmt: NodeTag = 277;
pub const T_AlterSeqStmt: NodeTag = 276;
pub const T_CreateSeqStmt: NodeTag = 275;
pub const T_CreateTableAsStmt: NodeTag = 274;
pub const T_ExplainStmt: NodeTag = 273;
pub const T_VacuumStmt: NodeTag = 272;
pub const T_DropdbStmt: NodeTag = 271;
pub const T_CreatedbStmt: NodeTag = 270;
pub const T_CreateDomainStmt: NodeTag = 269;
pub const T_LoadStmt: NodeTag = 268;
pub const T_ViewStmt: NodeTag = 267;
pub const T_TransactionStmt: NodeTag = 266;
pub const T_UnlistenStmt: NodeTag = 265;
pub const T_ListenStmt: NodeTag = 264;
pub const T_NotifyStmt: NodeTag = 263;
pub const T_RuleStmt: NodeTag = 262;
pub const T_RenameStmt: NodeTag = 261;
pub const T_DoStmt: NodeTag = 260;
pub const T_AlterFunctionStmt: NodeTag = 259;
pub const T_CreateFunctionStmt: NodeTag = 258;
pub const T_IndexStmt: NodeTag = 257;
pub const T_FetchStmt: NodeTag = 256;
pub const T_CommentStmt: NodeTag = 255;
pub const T_TruncateStmt: NodeTag = 254;
pub const T_DropStmt: NodeTag = 253;
pub const T_DefineStmt: NodeTag = 252;
pub const T_CreateStmt: NodeTag = 251;
pub const T_CopyStmt: NodeTag = 250;
pub const T_ClusterStmt: NodeTag = 249;
pub const T_ClosePortalStmt: NodeTag = 248;
pub const T_AlterDefaultPrivilegesStmt: NodeTag = 247;
pub const T_GrantRoleStmt: NodeTag = 246;
pub const T_GrantStmt: NodeTag = 245;
pub const T_SetOperationStmt: NodeTag = 244;
pub const T_AlterDomainStmt: NodeTag = 243;
pub const T_AlterTableCmd: NodeTag = 242;
pub const T_AlterTableStmt: NodeTag = 241;
pub const T_PLAssignStmt: NodeTag = 240;
pub const T_ReturnStmt: NodeTag = 239;
pub const T_SelectStmt: NodeTag = 238;
pub const T_UpdateStmt: NodeTag = 237;
pub const T_DeleteStmt: NodeTag = 236;
pub const T_InsertStmt: NodeTag = 235;
pub const T_PlannedStmt: NodeTag = 234;
pub const T_Query: NodeTag = 233;
pub const T_RawStmt: NodeTag = 232;
pub const T_ExtensibleNode: NodeTag = 231;
pub const T_OidList: NodeTag = 230;
pub const T_IntList: NodeTag = 229;
pub const T_List: NodeTag = 228;
pub const T_BitString: NodeTag = 227;
pub const T_String: NodeTag = 226;
pub const T_Boolean: NodeTag = 225;
pub const T_Float: NodeTag = 224;
pub const T_Integer: NodeTag = 223;
pub const T_GenerationContext: NodeTag = 222;
pub const T_SlabContext: NodeTag = 221;
pub const T_AllocSetContext: NodeTag = 220;
pub const T_StatisticExtInfo: NodeTag = 219;
pub const T_GroupingSetData: NodeTag = 218;
pub const T_RollupData: NodeTag = 217;
pub const T_PlannerParamItem: NodeTag = 216;
pub const T_MinMaxAggInfo: NodeTag = 215;
pub const T_PlaceHolderInfo: NodeTag = 214;
pub const T_RowIdentityVarInfo: NodeTag = 213;
pub const T_AppendRelInfo: NodeTag = 212;
pub const T_SpecialJoinInfo: NodeTag = 211;
pub const T_PlaceHolderVar: NodeTag = 210;
pub const T_IndexClause: NodeTag = 209;
pub const T_RestrictInfo: NodeTag = 208;
pub const T_PathTarget: NodeTag = 207;
pub const T_PathKey: NodeTag = 206;
pub const T_EquivalenceMember: NodeTag = 205;
pub const T_EquivalenceClass: NodeTag = 204;
pub const T_LimitPath: NodeTag = 203;
pub const T_ModifyTablePath: NodeTag = 202;
pub const T_LockRowsPath: NodeTag = 201;
pub const T_RecursiveUnionPath: NodeTag = 200;
pub const T_SetOpPath: NodeTag = 199;
pub const T_WindowAggPath: NodeTag = 198;
pub const T_MinMaxAggPath: NodeTag = 197;
pub const T_GroupingSetsPath: NodeTag = 196;
pub const T_AggPath: NodeTag = 195;
pub const T_UpperUniquePath: NodeTag = 194;
pub const T_GroupPath: NodeTag = 193;
pub const T_IncrementalSortPath: NodeTag = 192;
pub const T_SortPath: NodeTag = 191;
pub const T_ProjectSetPath: NodeTag = 190;
pub const T_ProjectionPath: NodeTag = 189;
pub const T_GatherMergePath: NodeTag = 188;
pub const T_GatherPath: NodeTag = 187;
pub const T_UniquePath: NodeTag = 186;
pub const T_MemoizePath: NodeTag = 185;
pub const T_MaterialPath: NodeTag = 184;
pub const T_GroupResultPath: NodeTag = 183;
pub const T_MergeAppendPath: NodeTag = 182;
pub const T_AppendPath: NodeTag = 181;
pub const T_HashPath: NodeTag = 180;
pub const T_MergePath: NodeTag = 179;
pub const T_NestPath: NodeTag = 178;
pub const T_CustomPath: NodeTag = 177;
pub const T_ForeignPath: NodeTag = 176;
pub const T_SubqueryScanPath: NodeTag = 175;
pub const T_TidRangePath: NodeTag = 174;
pub const T_TidPath: NodeTag = 173;
pub const T_BitmapOrPath: NodeTag = 172;
pub const T_BitmapAndPath: NodeTag = 171;
pub const T_BitmapHeapPath: NodeTag = 170;
pub const T_IndexPath: NodeTag = 169;
pub const T_Path: NodeTag = 168;
pub const T_ParamPathInfo: NodeTag = 167;
pub const T_ForeignKeyOptInfo: NodeTag = 166;
pub const T_IndexOptInfo: NodeTag = 165;
pub const T_RelOptInfo: NodeTag = 164;
pub const T_PlannerGlobal: NodeTag = 163;
pub const T_PlannerInfo: NodeTag = 162;
pub const T_DomainConstraintState: NodeTag = 161;
pub const T_SubPlanState: NodeTag = 160;
pub const T_SetExprState: NodeTag = 159;
pub const T_WindowFuncExprState: NodeTag = 158;
pub const T_ExprState: NodeTag = 157;
pub const T_IntoClause: NodeTag = 156;
pub const T_OnConflictExpr: NodeTag = 155;
pub const T_FromExpr: NodeTag = 154;
pub const T_JoinExpr: NodeTag = 153;
pub const T_RangeTblRef: NodeTag = 152;
pub const T_TargetEntry: NodeTag = 151;
pub const T_InferenceElem: NodeTag = 150;
pub const T_NextValueExpr: NodeTag = 149;
pub const T_CurrentOfExpr: NodeTag = 148;
pub const T_SetToDefault: NodeTag = 147;
pub const T_CoerceToDomainValue: NodeTag = 146;
pub const T_CoerceToDomain: NodeTag = 145;
pub const T_BooleanTest: NodeTag = 144;
pub const T_NullTest: NodeTag = 143;
pub const T_XmlExpr: NodeTag = 142;
pub const T_SQLValueFunction: NodeTag = 141;
pub const T_MinMaxExpr: NodeTag = 140;
pub const T_CoalesceExpr: NodeTag = 139;
pub const T_RowCompareExpr: NodeTag = 138;
pub const T_RowExpr: NodeTag = 137;
pub const T_ArrayExpr: NodeTag = 136;
pub const T_CaseTestExpr: NodeTag = 135;
pub const T_CaseWhen: NodeTag = 134;
pub const T_CaseExpr: NodeTag = 133;
pub const T_CollateExpr: NodeTag = 132;
pub const T_ConvertRowtypeExpr: NodeTag = 131;
pub const T_ArrayCoerceExpr: NodeTag = 130;
pub const T_CoerceViaIO: NodeTag = 129;
pub const T_RelabelType: NodeTag = 128;
pub const T_FieldStore: NodeTag = 127;
pub const T_FieldSelect: NodeTag = 126;
pub const T_AlternativeSubPlan: NodeTag = 125;
pub const T_SubPlan: NodeTag = 124;
pub const T_SubLink: NodeTag = 123;
pub const T_BoolExpr: NodeTag = 122;
pub const T_ScalarArrayOpExpr: NodeTag = 121;
pub const T_NullIfExpr: NodeTag = 120;
pub const T_DistinctExpr: NodeTag = 119;
pub const T_OpExpr: NodeTag = 118;
pub const T_NamedArgExpr: NodeTag = 117;
pub const T_FuncExpr: NodeTag = 116;
pub const T_SubscriptingRef: NodeTag = 115;
pub const T_WindowFunc: NodeTag = 114;
pub const T_GroupingFunc: NodeTag = 113;
pub const T_Aggref: NodeTag = 112;
pub const T_Param: NodeTag = 111;
pub const T_Const: NodeTag = 110;
pub const T_Var: NodeTag = 109;
pub const T_TableFunc: NodeTag = 108;
pub const T_RangeVar: NodeTag = 107;
pub const T_Alias: NodeTag = 106;
pub const T_LimitState: NodeTag = 105;
pub const T_LockRowsState: NodeTag = 104;
pub const T_SetOpState: NodeTag = 103;
pub const T_HashState: NodeTag = 102;
pub const T_GatherMergeState: NodeTag = 101;
pub const T_GatherState: NodeTag = 100;
pub const T_UniqueState: NodeTag = 99;
pub const T_WindowAggState: NodeTag = 98;
pub const T_AggState: NodeTag = 97;
pub const T_GroupState: NodeTag = 96;
pub const T_IncrementalSortState: NodeTag = 95;
pub const T_SortState: NodeTag = 94;
pub const T_MemoizeState: NodeTag = 93;
pub const T_MaterialState: NodeTag = 92;
pub const T_HashJoinState: NodeTag = 91;
pub const T_MergeJoinState: NodeTag = 90;
pub const T_NestLoopState: NodeTag = 89;
pub const T_JoinState: NodeTag = 88;
pub const T_CustomScanState: NodeTag = 87;
pub const T_ForeignScanState: NodeTag = 86;
pub const T_WorkTableScanState: NodeTag = 85;
pub const T_NamedTuplestoreScanState: NodeTag = 84;
pub const T_CteScanState: NodeTag = 83;
pub const T_ValuesScanState: NodeTag = 82;
pub const T_TableFuncScanState: NodeTag = 81;
pub const T_FunctionScanState: NodeTag = 80;
pub const T_SubqueryScanState: NodeTag = 79;
pub const T_TidRangeScanState: NodeTag = 78;
pub const T_TidScanState: NodeTag = 77;
pub const T_BitmapHeapScanState: NodeTag = 76;
pub const T_BitmapIndexScanState: NodeTag = 75;
pub const T_IndexOnlyScanState: NodeTag = 74;
pub const T_IndexScanState: NodeTag = 73;
pub const T_SampleScanState: NodeTag = 72;
pub const T_SeqScanState: NodeTag = 71;
pub const T_ScanState: NodeTag = 70;
pub const T_BitmapOrState: NodeTag = 69;
pub const T_BitmapAndState: NodeTag = 68;
pub const T_RecursiveUnionState: NodeTag = 67;
pub const T_MergeAppendState: NodeTag = 66;
pub const T_AppendState: NodeTag = 65;
pub const T_ModifyTableState: NodeTag = 64;
pub const T_ProjectSetState: NodeTag = 63;
pub const T_ResultState: NodeTag = 62;
pub const T_PlanState: NodeTag = 61;
pub const T_PlanInvalItem: NodeTag = 60;
pub const T_PartitionPruneStepCombine: NodeTag = 59;
pub const T_PartitionPruneStepOp: NodeTag = 58;
pub const T_PartitionedRelPruneInfo: NodeTag = 57;
pub const T_PartitionPruneInfo: NodeTag = 56;
pub const T_PlanRowMark: NodeTag = 55;
pub const T_NestLoopParam: NodeTag = 54;
pub const T_Limit: NodeTag = 53;
pub const T_LockRows: NodeTag = 52;
pub const T_SetOp: NodeTag = 51;
pub const T_Hash: NodeTag = 50;
pub const T_GatherMerge: NodeTag = 49;
pub const T_Gather: NodeTag = 48;
pub const T_Unique: NodeTag = 47;
pub const T_WindowAgg: NodeTag = 46;
pub const T_Agg: NodeTag = 45;
pub const T_Group: NodeTag = 44;
pub const T_IncrementalSort: NodeTag = 43;
pub const T_Sort: NodeTag = 42;
pub const T_Memoize: NodeTag = 41;
pub const T_Material: NodeTag = 40;
pub const T_HashJoin: NodeTag = 39;
pub const T_MergeJoin: NodeTag = 38;
pub const T_NestLoop: NodeTag = 37;
pub const T_Join: NodeTag = 36;
pub const T_CustomScan: NodeTag = 35;
pub const T_ForeignScan: NodeTag = 34;
pub const T_WorkTableScan: NodeTag = 33;
pub const T_NamedTuplestoreScan: NodeTag = 32;
pub const T_CteScan: NodeTag = 31;
pub const T_TableFuncScan: NodeTag = 30;
pub const T_ValuesScan: NodeTag = 29;
pub const T_FunctionScan: NodeTag = 28;
pub const T_SubqueryScan: NodeTag = 27;
pub const T_TidRangeScan: NodeTag = 26;
pub const T_TidScan: NodeTag = 25;
pub const T_BitmapHeapScan: NodeTag = 24;
pub const T_BitmapIndexScan: NodeTag = 23;
pub const T_IndexOnlyScan: NodeTag = 22;
pub const T_IndexScan: NodeTag = 21;
pub const T_SampleScan: NodeTag = 20;
pub const T_SeqScan: NodeTag = 19;
pub const T_Scan: NodeTag = 18;
pub const T_BitmapOr: NodeTag = 17;
pub const T_BitmapAnd: NodeTag = 16;
pub const T_RecursiveUnion: NodeTag = 15;
pub const T_MergeAppend: NodeTag = 14;
pub const T_Append: NodeTag = 13;
pub const T_ModifyTable: NodeTag = 12;
pub const T_ProjectSet: NodeTag = 11;
pub const T_Result: NodeTag = 10;
pub const T_Plan: NodeTag = 9;
pub const T_TupleTableSlot: NodeTag = 8;
pub const T_EState: NodeTag = 7;
pub const T_ResultRelInfo: NodeTag = 6;
pub const T_OnConflictSetState: NodeTag = 5;
pub const T_JunkFilter: NodeTag = 4;
pub const T_ProjectionInfo: NodeTag = 3;
pub const T_ExprContext: NodeTag = 2;
pub const T_IndexInfo: NodeTag = 1;
pub const T_Invalid: NodeTag = 0;
pub type Datum = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NullableDatum {
    pub value: Datum,
    pub isnull: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockIdData {
    pub bi_hi: uint16,
    pub bi_lo: uint16,
}
pub type OffsetNumber = uint16;
#[derive(Copy, Clone)]
#[repr(C, align(2))]
pub struct ItemPointerData(pub ItemPointerData_Inner);
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ItemPointerData_Inner {
    pub ip_blkid: BlockIdData,
    pub ip_posid: OffsetNumber,
}
#[allow(dead_code, non_upper_case_globals)]
const ItemPointerData_PADDING: usize =
    ::core::mem::size_of::<ItemPointerData>() - ::core::mem::size_of::<ItemPointerData_Inner>();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeapTupleHeaderData {
    pub t_choice: C2RustUnnamed_0,
    pub t_ctid: ItemPointerData,
    pub t_infomask2: uint16,
    pub t_infomask: uint16,
    pub t_hoff: uint8,
    pub t_bits: [bits8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub t_heap: HeapTupleFields,
    pub t_datum: DatumTupleFields,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DatumTupleFields {
    pub datum_len_: int32,
    pub datum_typmod: int32,
    pub datum_typeid: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeapTupleFields {
    pub t_xmin: TransactionId,
    pub t_xmax: TransactionId,
    pub t_field3: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub t_cid: CommandId,
    pub t_xvac: TransactionId,
}
pub type HeapTupleHeader = *mut HeapTupleHeaderData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MinimalTupleData {
    pub t_len: uint32,
    pub mt_padding: [libc::c_char; 6],
    pub t_infomask2: uint16,
    pub t_infomask: uint16,
    pub t_hoff: uint8,
    pub t_bits: [bits8; 0],
}
pub type MinimalTuple = *mut MinimalTupleData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeapTupleData {
    pub t_len: uint32,
    pub t_self: ItemPointerData,
    pub t_tableOid: Oid,
    pub t_data: HeapTupleHeader,
}
pub type HeapTuple = *mut HeapTupleData;
pub type XLogRecPtr = uint64;
pub type AttrNumber = int16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_attribute {
    pub attrelid: Oid,
    pub attname: NameData,
    pub atttypid: Oid,
    pub attstattarget: int32,
    pub attlen: int16,
    pub attnum: int16,
    pub attndims: int32,
    pub attcacheoff: int32,
    pub atttypmod: int32,
    pub attbyval: bool_0,
    pub attalign: libc::c_char,
    pub attstorage: libc::c_char,
    pub attcompression: libc::c_char,
    pub attnotnull: bool_0,
    pub atthasdef: bool_0,
    pub atthasmissing: bool_0,
    pub attidentity: libc::c_char,
    pub attgenerated: libc::c_char,
    pub attisdropped: bool_0,
    pub attislocal: bool_0,
    pub attinhcount: int32,
    pub attcollation: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub type_0: NodeTag,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Bitmapset {
    pub nwords: libc::c_int,
    pub words: [bitmapword; 0],
}
pub type bitmapword = uint64;
pub type Cost = libc::c_double;
pub type Cardinality = libc::c_double;
pub type CmdType = libc::c_uint;
pub const CMD_NOTHING: CmdType = 6;
pub const CMD_UTILITY: CmdType = 5;
pub const CMD_DELETE: CmdType = 4;
pub const CMD_INSERT: CmdType = 3;
pub const CMD_UPDATE: CmdType = 2;
pub const CMD_SELECT: CmdType = 1;
pub const CMD_UNKNOWN: CmdType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union ListCell {
    pub ptr_value: *mut libc::c_void,
    pub int_value: libc::c_int,
    pub oid_value: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct List {
    pub type_0: NodeTag,
    pub length: libc::c_int,
    pub max_length: libc::c_int,
    pub elements: *mut ListCell,
    pub initial_elements: [ListCell; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttrDefault {
    pub adnum: AttrNumber,
    pub adbin: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConstrCheck {
    pub ccname: *mut libc::c_char,
    pub ccbin: *mut libc::c_char,
    pub ccvalid: bool_0,
    pub ccnoinherit: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleConstr {
    pub defval: *mut AttrDefault,
    pub check: *mut ConstrCheck,
    pub missing: *mut AttrMissing,
    pub num_defval: uint16,
    pub num_check: uint16,
    pub has_not_null: bool_0,
    pub has_generated_stored: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleDescData {
    pub natts: libc::c_int,
    pub tdtypeid: Oid,
    pub tdtypmod: int32,
    pub tdrefcount: libc::c_int,
    pub constr: *mut TupleConstr,
    pub attrs: [FormData_pg_attribute; 0],
}
pub type TupleDesc = *mut TupleDescData;
pub type Timestamp = int64;
pub type TimestampTz = int64;
pub type TimeOffset = int64;
pub type fsec_t = int32;
pub type DateADT = int32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Interval {
    pub time: TimeOffset,
    pub day: int32,
    pub month: int32,
}
pub type LockClauseStrength = libc::c_uint;
pub const LCS_FORUPDATE: LockClauseStrength = 4;
pub const LCS_FORNOKEYUPDATE: LockClauseStrength = 3;
pub const LCS_FORSHARE: LockClauseStrength = 2;
pub const LCS_FORKEYSHARE: LockClauseStrength = 1;
pub const LCS_NONE: LockClauseStrength = 0;
pub type LockWaitPolicy = libc::c_uint;
pub const LockWaitError: LockWaitPolicy = 2;
pub const LockWaitSkip: LockWaitPolicy = 1;
pub const LockWaitBlock: LockWaitPolicy = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Expr {
    pub type_0: NodeTag,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Const {
    pub xpr: Expr,
    pub consttype: Oid,
    pub consttypmod: int32,
    pub constcollid: Oid,
    pub constlen: libc::c_int,
    pub constvalue: Datum,
    pub constisnull: bool_0,
    pub constbyval: bool_0,
    pub location: libc::c_int,
}
pub type ParamKind = libc::c_uint;
pub const PARAM_MULTIEXPR: ParamKind = 3;
pub const PARAM_SUBLINK: ParamKind = 2;
pub const PARAM_EXEC: ParamKind = 1;
pub const PARAM_EXTERN: ParamKind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Param {
    pub xpr: Expr,
    pub paramkind: ParamKind,
    pub paramid: libc::c_int,
    pub paramtype: Oid,
    pub paramtypmod: int32,
    pub paramcollid: Oid,
    pub location: libc::c_int,
}
pub type CoercionForm = libc::c_uint;
pub const COERCE_SQL_SYNTAX: CoercionForm = 3;
pub const COERCE_IMPLICIT_CAST: CoercionForm = 2;
pub const COERCE_EXPLICIT_CAST: CoercionForm = 1;
pub const COERCE_EXPLICIT_CALL: CoercionForm = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncExpr {
    pub xpr: Expr,
    pub funcid: Oid,
    pub funcresulttype: Oid,
    pub funcretset: bool_0,
    pub funcvariadic: bool_0,
    pub funcformat: CoercionForm,
    pub funccollid: Oid,
    pub inputcollid: Oid,
    pub args: *mut List,
    pub location: libc::c_int,
}
pub type PartitionDirectory = *mut PartitionDirectoryData;
pub type Relation = *mut RelationData;
pub type RelationPtr = *mut Relation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttrMap {
    pub attnums: *mut AttrNumber,
    pub maplen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleTableSlotOps {
    pub base_slot_size: size_t,
    pub init: Option<unsafe extern "C" fn(*mut TupleTableSlot) -> ()>,
    pub release: Option<unsafe extern "C" fn(*mut TupleTableSlot) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut TupleTableSlot) -> ()>,
    pub getsomeattrs: Option<unsafe extern "C" fn(*mut TupleTableSlot, libc::c_int) -> ()>,
    pub getsysattr:
        Option<unsafe extern "C" fn(*mut TupleTableSlot, libc::c_int, *mut bool_0) -> Datum>,
    pub materialize: Option<unsafe extern "C" fn(*mut TupleTableSlot) -> ()>,
    pub copyslot: Option<unsafe extern "C" fn(*mut TupleTableSlot, *mut TupleTableSlot) -> ()>,
    pub get_heap_tuple: Option<unsafe extern "C" fn(*mut TupleTableSlot) -> HeapTuple>,
    pub get_minimal_tuple: Option<unsafe extern "C" fn(*mut TupleTableSlot) -> MinimalTuple>,
    pub copy_heap_tuple: Option<unsafe extern "C" fn(*mut TupleTableSlot) -> HeapTuple>,
    pub copy_minimal_tuple: Option<unsafe extern "C" fn(*mut TupleTableSlot) -> MinimalTuple>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleTableSlot {
    pub type_0: NodeTag,
    pub tts_flags: uint16,
    pub tts_nvalid: AttrNumber,
    pub tts_ops: *const TupleTableSlotOps,
    pub tts_tupleDescriptor: TupleDesc,
    pub tts_values: *mut Datum,
    pub tts_isnull: *mut bool_0,
    pub tts_mcxt: MemoryContext,
    pub tts_tid: ItemPointerData,
    pub tts_tableOid: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleConversionMap {
    pub indesc: TupleDesc,
    pub outdesc: TupleDesc,
    pub attrMap: *mut AttrMap,
    pub invalues: *mut Datum,
    pub inisnull: *mut bool_0,
    pub outvalues: *mut Datum,
    pub outisnull: *mut bool_0,
}
pub type instr_time = timespec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferUsage {
    pub shared_blks_hit: int64,
    pub shared_blks_read: int64,
    pub shared_blks_dirtied: int64,
    pub shared_blks_written: int64,
    pub local_blks_hit: int64,
    pub local_blks_read: int64,
    pub local_blks_dirtied: int64,
    pub local_blks_written: int64,
    pub temp_blks_read: int64,
    pub temp_blks_written: int64,
    pub blk_read_time: instr_time,
    pub blk_write_time: instr_time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WalUsage {
    pub wal_records: int64,
    pub wal_fpi: int64,
    pub wal_bytes: uint64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Instrumentation {
    pub need_timer: bool_0,
    pub need_bufusage: bool_0,
    pub need_walusage: bool_0,
    pub async_mode: bool_0,
    pub running: bool_0,
    pub starttime: instr_time,
    pub counter: instr_time,
    pub firsttuple: libc::c_double,
    pub tuplecount: libc::c_double,
    pub bufusage_start: BufferUsage,
    pub walusage_start: WalUsage,
    pub startup: libc::c_double,
    pub total: libc::c_double,
    pub ntuples: libc::c_double,
    pub ntuples2: libc::c_double,
    pub nloops: libc::c_double,
    pub nfiltered1: libc::c_double,
    pub nfiltered2: libc::c_double,
    pub bufusage: BufferUsage,
    pub walusage: WalUsage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WorkerInstrumentation {
    pub num_workers: libc::c_int,
    pub instrument: [Instrumentation; 0],
}
pub type fmNodePtr = *mut Node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FunctionCallInfoBaseData {
    pub flinfo: *mut FmgrInfo,
    pub context: fmNodePtr,
    pub resultinfo: fmNodePtr,
    pub fncollation: Oid,
    pub isnull: bool_0,
    pub nargs: libc::c_short,
    pub args: [NullableDatum; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FmgrInfo {
    pub fn_addr: PGFunction,
    pub fn_oid: Oid,
    pub fn_nargs: libc::c_short,
    pub fn_strict: bool_0,
    pub fn_retset: bool_0,
    pub fn_stats: libc::c_uchar,
    pub fn_extra: *mut libc::c_void,
    pub fn_mcxt: MemoryContext,
    pub fn_expr: fmNodePtr,
}
pub type PGFunction = Option<unsafe extern "C" fn(FunctionCallInfo) -> Datum>;
pub type FunctionCallInfo = *mut FunctionCallInfoBaseData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pairingheap_node {
    pub first_child: *mut pairingheap_node,
    pub next_sibling: *mut pairingheap_node,
    pub prev_or_parent: *mut pairingheap_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprState {
    pub type_0: NodeTag,
    pub flags: uint8,
    pub resnull: bool_0,
    pub resvalue: Datum,
    pub resultslot: *mut TupleTableSlot,
    pub steps: *mut ExprEvalStep,
    pub evalfunc: ExprStateEvalFunc,
    pub expr: *mut Expr,
    pub evalfunc_private: *mut libc::c_void,
    pub steps_len: libc::c_int,
    pub steps_alloc: libc::c_int,
    pub parent: *mut PlanState,
    pub ext_params: ParamListInfo,
    pub innermost_caseval: *mut Datum,
    pub innermost_casenull: *mut bool_0,
    pub innermost_domainval: *mut Datum,
    pub innermost_domainnull: *mut bool_0,
}
pub type ParamListInfo = *mut ParamListInfoData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParamListInfoData {
    pub paramFetch: ParamFetchHook,
    pub paramFetchArg: *mut libc::c_void,
    pub paramCompile: ParamCompileHook,
    pub paramCompileArg: *mut libc::c_void,
    pub parserSetup: ParserSetupHook,
    pub parserSetupArg: *mut libc::c_void,
    pub paramValuesStr: *mut libc::c_char,
    pub numParams: libc::c_int,
    pub params: [ParamExternData; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParamExternData {
    pub value: Datum,
    pub isnull: bool_0,
    pub pflags: uint16,
    pub ptype: Oid,
}
pub type ParserSetupHook = Option<unsafe extern "C" fn(*mut ParseState, *mut libc::c_void) -> ()>;
pub type ParamCompileHook = Option<
    unsafe extern "C" fn(ParamListInfo, *mut Param, *mut ExprState, *mut Datum, *mut bool_0) -> (),
>;
pub type ParamFetchHook = Option<
    unsafe extern "C" fn(
        ParamListInfo,
        libc::c_int,
        bool_0,
        *mut ParamExternData,
    ) -> *mut ParamExternData,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlanState {
    pub type_0: NodeTag,
    pub plan: *mut Plan,
    pub state: *mut EState,
    pub ExecProcNode: ExecProcNodeMtd,
    pub ExecProcNodeReal: ExecProcNodeMtd,
    pub instrument: *mut Instrumentation,
    pub worker_instrument: *mut WorkerInstrumentation,
    pub worker_jit_instrument: *mut SharedJitInstrumentation,
    pub qual: *mut ExprState,
    pub lefttree: *mut PlanState,
    pub righttree: *mut PlanState,
    pub initPlan: *mut List,
    pub subPlan: *mut List,
    pub chgParam: *mut Bitmapset,
    pub ps_ResultTupleDesc: TupleDesc,
    pub ps_ResultTupleSlot: *mut TupleTableSlot,
    pub ps_ExprContext: *mut ExprContext,
    pub ps_ProjInfo: *mut ProjectionInfo,
    pub async_capable: bool_0,
    pub scandesc: TupleDesc,
    pub scanops: *const TupleTableSlotOps,
    pub outerops: *const TupleTableSlotOps,
    pub innerops: *const TupleTableSlotOps,
    pub resultops: *const TupleTableSlotOps,
    pub scanopsfixed: bool_0,
    pub outeropsfixed: bool_0,
    pub inneropsfixed: bool_0,
    pub resultopsfixed: bool_0,
    pub scanopsset: bool_0,
    pub outeropsset: bool_0,
    pub inneropsset: bool_0,
    pub resultopsset: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProjectionInfo {
    pub type_0: NodeTag,
    pub pi_state: ExprState,
    pub pi_exprContext: *mut ExprContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprContext {
    pub type_0: NodeTag,
    pub ecxt_scantuple: *mut TupleTableSlot,
    pub ecxt_innertuple: *mut TupleTableSlot,
    pub ecxt_outertuple: *mut TupleTableSlot,
    pub ecxt_per_query_memory: MemoryContext,
    pub ecxt_per_tuple_memory: MemoryContext,
    pub ecxt_param_exec_vals: *mut ParamExecData,
    pub ecxt_param_list_info: ParamListInfo,
    pub ecxt_aggvalues: *mut Datum,
    pub ecxt_aggnulls: *mut bool_0,
    pub caseValue_datum: Datum,
    pub caseValue_isNull: bool_0,
    pub domainValue_datum: Datum,
    pub domainValue_isNull: bool_0,
    pub ecxt_estate: *mut EState,
    pub ecxt_callbacks: *mut ExprContext_CB,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprContext_CB {
    pub next: *mut ExprContext_CB,
    pub function: ExprContextCallbackFunction,
    pub arg: Datum,
}
pub type ExprContextCallbackFunction = Option<unsafe extern "C" fn(Datum) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EState {
    pub type_0: NodeTag,
    pub es_direction: ScanDirection,
    pub es_snapshot: Snapshot,
    pub es_crosscheck_snapshot: Snapshot,
    pub es_range_table: *mut List,
    pub es_range_table_size: Index,
    pub es_relations: *mut Relation,
    pub es_rowmarks: *mut *mut ExecRowMark,
    pub es_plannedstmt: *mut PlannedStmt,
    pub es_sourceText: *const libc::c_char,
    pub es_junkFilter: *mut JunkFilter,
    pub es_output_cid: CommandId,
    pub es_result_relations: *mut *mut ResultRelInfo,
    pub es_opened_result_relations: *mut List,
    pub es_partition_directory: PartitionDirectory,
    pub es_tuple_routing_result_relations: *mut List,
    pub es_trig_target_relations: *mut List,
    pub es_param_list_info: ParamListInfo,
    pub es_param_exec_vals: *mut ParamExecData,
    pub es_queryEnv: *mut QueryEnvironment,
    pub es_query_cxt: MemoryContext,
    pub es_tupleTable: *mut List,
    pub es_processed: uint64,
    pub es_top_eflags: libc::c_int,
    pub es_instrument: libc::c_int,
    pub es_finished: bool_0,
    pub es_exprcontexts: *mut List,
    pub es_subplanstates: *mut List,
    pub es_auxmodifytables: *mut List,
    pub es_per_tuple_exprcontext: *mut ExprContext,
    pub es_epq_active: *mut EPQState,
    pub es_use_parallel_mode: bool_0,
    pub es_query_dsa: *mut dsa_area,
    pub es_jit_flags: libc::c_int,
    pub es_jit: *mut JitContext,
    pub es_jit_worker_instr: *mut JitInstrumentation,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EPQState {
    pub parentestate: *mut EState,
    pub epqParam: libc::c_int,
    pub tuple_table: *mut List,
    pub relsubs_slot: *mut *mut TupleTableSlot,
    pub plan: *mut Plan,
    pub arowMarks: *mut List,
    pub origslot: *mut TupleTableSlot,
    pub recheckestate: *mut EState,
    pub relsubs_rowmark: *mut *mut ExecAuxRowMark,
    pub relsubs_done: *mut bool_0,
    pub recheckplanstate: *mut PlanState,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExecAuxRowMark {
    pub rowmark: *mut ExecRowMark,
    pub ctidAttNo: AttrNumber,
    pub toidAttNo: AttrNumber,
    pub wholeAttNo: AttrNumber,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExecRowMark {
    pub relation: Relation,
    pub relid: Oid,
    pub rti: Index,
    pub prti: Index,
    pub rowmarkId: Index,
    pub markType: RowMarkType,
    pub strength: LockClauseStrength,
    pub waitPolicy: LockWaitPolicy,
    pub ermActive: bool_0,
    pub curCtid: ItemPointerData,
    pub ermExtra: *mut libc::c_void,
}
pub type RowMarkType = libc::c_uint;
pub const ROW_MARK_COPY: RowMarkType = 5;
pub const ROW_MARK_REFERENCE: RowMarkType = 4;
pub const ROW_MARK_KEYSHARE: RowMarkType = 3;
pub const ROW_MARK_SHARE: RowMarkType = 2;
pub const ROW_MARK_NOKEYEXCLUSIVE: RowMarkType = 1;
pub const ROW_MARK_EXCLUSIVE: RowMarkType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Plan {
    pub type_0: NodeTag,
    pub startup_cost: Cost,
    pub total_cost: Cost,
    pub plan_rows: Cardinality,
    pub plan_width: libc::c_int,
    pub parallel_aware: bool_0,
    pub parallel_safe: bool_0,
    pub async_capable: bool_0,
    pub plan_node_id: libc::c_int,
    pub targetlist: *mut List,
    pub qual: *mut List,
    pub lefttree: *mut Plan,
    pub righttree: *mut Plan,
    pub initPlan: *mut List,
    pub extParam: *mut Bitmapset,
    pub allParam: *mut Bitmapset,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParamExecData {
    pub execPlan: *mut libc::c_void,
    pub value: Datum,
    pub isnull: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ResultRelInfo {
    pub type_0: NodeTag,
    pub ri_RangeTableIndex: Index,
    pub ri_RelationDesc: Relation,
    pub ri_NumIndices: libc::c_int,
    pub ri_IndexRelationDescs: RelationPtr,
    pub ri_IndexRelationInfo: *mut *mut IndexInfo,
    pub ri_RowIdAttNo: AttrNumber,
    pub ri_projectNew: *mut ProjectionInfo,
    pub ri_newTupleSlot: *mut TupleTableSlot,
    pub ri_oldTupleSlot: *mut TupleTableSlot,
    pub ri_projectNewInfoValid: bool_0,
    pub ri_TrigDesc: *mut TriggerDesc,
    pub ri_TrigFunctions: *mut FmgrInfo,
    pub ri_TrigWhenExprs: *mut *mut ExprState,
    pub ri_TrigInstrument: *mut Instrumentation,
    pub ri_ReturningSlot: *mut TupleTableSlot,
    pub ri_TrigOldSlot: *mut TupleTableSlot,
    pub ri_TrigNewSlot: *mut TupleTableSlot,
    pub ri_FdwRoutine: *mut FdwRoutine,
    pub ri_FdwState: *mut libc::c_void,
    pub ri_usesFdwDirectModify: bool_0,
    pub ri_NumSlots: libc::c_int,
    pub ri_NumSlotsInitialized: libc::c_int,
    pub ri_BatchSize: libc::c_int,
    pub ri_Slots: *mut *mut TupleTableSlot,
    pub ri_PlanSlots: *mut *mut TupleTableSlot,
    pub ri_WithCheckOptions: *mut List,
    pub ri_WithCheckOptionExprs: *mut List,
    pub ri_ConstraintExprs: *mut *mut ExprState,
    pub ri_GeneratedExprs: *mut *mut ExprState,
    pub ri_NumGeneratedNeeded: libc::c_int,
    pub ri_returningList: *mut List,
    pub ri_projectReturning: *mut ProjectionInfo,
    pub ri_onConflictArbiterIndexes: *mut List,
    pub ri_onConflict: *mut OnConflictSetState,
    pub ri_PartitionCheckExpr: *mut ExprState,
    pub ri_RootResultRelInfo: *mut ResultRelInfo,
    pub ri_RootToPartitionMap: *mut TupleConversionMap,
    pub ri_PartitionTupleSlot: *mut TupleTableSlot,
    pub ri_ChildToRootMap: *mut TupleConversionMap,
    pub ri_ChildToRootMapValid: bool_0,
    pub ri_CopyMultiInsertBuffer: *mut CopyMultiInsertBuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnConflictSetState {
    pub type_0: NodeTag,
    pub oc_Existing: *mut TupleTableSlot,
    pub oc_ProjSlot: *mut TupleTableSlot,
    pub oc_ProjInfo: *mut ProjectionInfo,
    pub oc_WhereClause: *mut ExprState,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TriggerDesc {
    pub triggers: *mut Trigger,
    pub numtriggers: libc::c_int,
    pub trig_insert_before_row: bool_0,
    pub trig_insert_after_row: bool_0,
    pub trig_insert_instead_row: bool_0,
    pub trig_insert_before_statement: bool_0,
    pub trig_insert_after_statement: bool_0,
    pub trig_update_before_row: bool_0,
    pub trig_update_after_row: bool_0,
    pub trig_update_instead_row: bool_0,
    pub trig_update_before_statement: bool_0,
    pub trig_update_after_statement: bool_0,
    pub trig_delete_before_row: bool_0,
    pub trig_delete_after_row: bool_0,
    pub trig_delete_instead_row: bool_0,
    pub trig_delete_before_statement: bool_0,
    pub trig_delete_after_statement: bool_0,
    pub trig_truncate_before_statement: bool_0,
    pub trig_truncate_after_statement: bool_0,
    pub trig_insert_new_table: bool_0,
    pub trig_update_old_table: bool_0,
    pub trig_update_new_table: bool_0,
    pub trig_delete_old_table: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Trigger {
    pub tgoid: Oid,
    pub tgname: *mut libc::c_char,
    pub tgfoid: Oid,
    pub tgtype: int16,
    pub tgenabled: libc::c_char,
    pub tgisinternal: bool_0,
    pub tgisclone: bool_0,
    pub tgconstrrelid: Oid,
    pub tgconstrindid: Oid,
    pub tgconstraint: Oid,
    pub tgdeferrable: bool_0,
    pub tginitdeferred: bool_0,
    pub tgnargs: int16,
    pub tgnattr: int16,
    pub tgattr: *mut int16,
    pub tgargs: *mut *mut libc::c_char,
    pub tgqual: *mut libc::c_char,
    pub tgoldtable: *mut libc::c_char,
    pub tgnewtable: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexInfo {
    pub type_0: NodeTag,
    pub ii_NumIndexAttrs: libc::c_int,
    pub ii_NumIndexKeyAttrs: libc::c_int,
    pub ii_IndexAttrNumbers: [AttrNumber; 32],
    pub ii_Expressions: *mut List,
    pub ii_ExpressionsState: *mut List,
    pub ii_Predicate: *mut List,
    pub ii_PredicateState: *mut ExprState,
    pub ii_ExclusionOps: *mut Oid,
    pub ii_ExclusionProcs: *mut Oid,
    pub ii_ExclusionStrats: *mut uint16,
    pub ii_UniqueOps: *mut Oid,
    pub ii_UniqueProcs: *mut Oid,
    pub ii_UniqueStrats: *mut uint16,
    pub ii_OpclassOptions: *mut Datum,
    pub ii_Unique: bool_0,
    pub ii_NullsNotDistinct: bool_0,
    pub ii_ReadyForInserts: bool_0,
    pub ii_CheckedUnchanged: bool_0,
    pub ii_IndexUnchanged: bool_0,
    pub ii_Concurrent: bool_0,
    pub ii_BrokenHotChain: bool_0,
    pub ii_ParallelWorkers: libc::c_int,
    pub ii_Am: Oid,
    pub ii_AmCache: *mut libc::c_void,
    pub ii_Context: MemoryContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JunkFilter {
    pub type_0: NodeTag,
    pub jf_targetList: *mut List,
    pub jf_cleanTupType: TupleDesc,
    pub jf_cleanMap: *mut AttrNumber,
    pub jf_resultSlot: *mut TupleTableSlot,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlannedStmt {
    pub type_0: NodeTag,
    pub commandType: CmdType,
    pub queryId: uint64,
    pub hasReturning: bool_0,
    pub hasModifyingCTE: bool_0,
    pub canSetTag: bool_0,
    pub transientPlan: bool_0,
    pub dependsOnRole: bool_0,
    pub parallelModeNeeded: bool_0,
    pub jitFlags: libc::c_int,
    pub planTree: *mut Plan,
    pub rtable: *mut List,
    pub resultRelations: *mut List,
    pub appendRelations: *mut List,
    pub subplans: *mut List,
    pub rewindPlanIDs: *mut Bitmapset,
    pub rowMarks: *mut List,
    pub relationOids: *mut List,
    pub invalItems: *mut List,
    pub paramExecTypes: *mut List,
    pub utilityStmt: *mut Node,
    pub stmt_location: libc::c_int,
    pub stmt_len: libc::c_int,
}
pub type Snapshot = *mut SnapshotData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SnapshotData {
    pub snapshot_type: SnapshotType,
    pub xmin: TransactionId,
    pub xmax: TransactionId,
    pub xip: *mut TransactionId,
    pub xcnt: uint32,
    pub subxip: *mut TransactionId,
    pub subxcnt: int32,
    pub suboverflowed: bool_0,
    pub takenDuringRecovery: bool_0,
    pub copied: bool_0,
    pub curcid: CommandId,
    pub speculativeToken: uint32,
    pub vistest: *mut GlobalVisState,
    pub active_count: uint32,
    pub regd_count: uint32,
    pub ph_node: pairingheap_node,
    pub whenTaken: TimestampTz,
    pub lsn: XLogRecPtr,
    pub snapXactCompletionCount: uint64,
}
pub type SnapshotType = libc::c_uint;
pub const SNAPSHOT_NON_VACUUMABLE: SnapshotType = 6;
pub const SNAPSHOT_HISTORIC_MVCC: SnapshotType = 5;
pub const SNAPSHOT_DIRTY: SnapshotType = 4;
pub const SNAPSHOT_TOAST: SnapshotType = 3;
pub const SNAPSHOT_ANY: SnapshotType = 2;
pub const SNAPSHOT_SELF: SnapshotType = 1;
pub const SNAPSHOT_MVCC: SnapshotType = 0;
pub type ScanDirection = libc::c_int;
pub const ForwardScanDirection: ScanDirection = 1;
pub const NoMovementScanDirection: ScanDirection = 0;
pub const BackwardScanDirection: ScanDirection = -1;
pub type ExecProcNodeMtd = Option<unsafe extern "C" fn(*mut PlanState) -> *mut TupleTableSlot>;
pub type ExprStateEvalFunc =
    Option<unsafe extern "C" fn(*mut ExprState, *mut ExprContext, *mut bool_0) -> Datum>;
pub type ExprDoneCond = libc::c_uint;
pub const ExprEndResult: ExprDoneCond = 2;
pub const ExprMultipleResult: ExprDoneCond = 1;
pub const ExprSingleResult: ExprDoneCond = 0;
pub type SetFunctionReturnMode = libc::c_uint;
pub const SFRM_Materialize_Preferred: SetFunctionReturnMode = 8;
pub const SFRM_Materialize_Random: SetFunctionReturnMode = 4;
pub const SFRM_Materialize: SetFunctionReturnMode = 2;
pub const SFRM_ValuePerCall: SetFunctionReturnMode = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReturnSetInfo {
    pub type_0: NodeTag,
    pub econtext: *mut ExprContext,
    pub expectedDesc: TupleDesc,
    pub allowedModes: libc::c_int,
    pub returnMode: SetFunctionReturnMode,
    pub isDone: ExprDoneCond,
    pub setResult: *mut Tuplestorestate,
    pub setDesc: TupleDesc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttInMetadata {
    pub tupdesc: TupleDesc,
    pub attinfuncs: *mut FmgrInfo,
    pub attioparams: *mut Oid,
    pub atttypmods: *mut int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncCallContext {
    pub call_cntr: uint64,
    pub max_calls: uint64,
    pub user_fctx: *mut libc::c_void,
    pub attinmeta: *mut AttInMetadata,
    pub multi_call_memory_ctx: MemoryContext,
    pub tuple_desc: TupleDesc,
}
pub type TypeFuncClass = libc::c_uint;
pub const TYPEFUNC_OTHER: TypeFuncClass = 4;
pub const TYPEFUNC_RECORD: TypeFuncClass = 3;
pub const TYPEFUNC_COMPOSITE_DOMAIN: TypeFuncClass = 2;
pub const TYPEFUNC_COMPOSITE: TypeFuncClass = 1;
pub const TYPEFUNC_SCALAR: TypeFuncClass = 0;
pub type pg_time_t = int64;
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
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct datetkn {
    pub token: [libc::c_char; 11],
    pub type_0: libc::c_char,
    pub value: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TimeZoneAbbrevTable {
    pub tblsize: Size,
    pub numabbrevs: libc::c_int,
    pub abbrevs: [datetkn; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynamicZoneAbbrev {
    pub tz: *mut pg_tz,
    pub zone: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tzEntry {
    pub abbrev: *mut libc::c_char,
    pub zone: *mut libc::c_char,
    pub offset: libc::c_int,
    pub is_dst: bool_0,
    pub lineno: libc::c_int,
    pub filename: *const libc::c_char,
}
#[inline]
unsafe extern "C" fn MemoryContextSwitchTo(mut context: MemoryContext) -> MemoryContext {
    let mut old: MemoryContext = CurrentMemoryContext;
    CurrentMemoryContext = context;
    return old;
}
#[inline]
unsafe extern "C" fn list_nth_cell(mut list: *const List, mut n: libc::c_int) -> *mut ListCell {
    return &mut *((*list).elements).offset(n as isize) as *mut ListCell;
}
#[no_mangle]
pub static mut day_tab: [[libc::c_int; 13]; 2] = [
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
#[no_mangle]
pub static mut months: [*const libc::c_char; 13] = [
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
#[no_mangle]
pub static mut days: [*const libc::c_char; 8] = [
    b"Sunday\0" as *const u8 as *const libc::c_char,
    b"Monday\0" as *const u8 as *const libc::c_char,
    b"Tuesday\0" as *const u8 as *const libc::c_char,
    b"Wednesday\0" as *const u8 as *const libc::c_char,
    b"Thursday\0" as *const u8 as *const libc::c_char,
    b"Friday\0" as *const u8 as *const libc::c_char,
    b"Saturday\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut datetktbl: [datetkn; 71] = unsafe {
    [
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"-infinity\0\0",
                ),
                type_0: 0 as libc::c_int as libc::c_char,
                value: 9 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"ad\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 18 as libc::c_int as libc::c_char,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"allballs\0\0\0",
                ),
                type_0: 0 as libc::c_int as libc::c_char,
                value: 16 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"am\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 9 as libc::c_int as libc::c_char,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"apr\0\0\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"april\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"at\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 8 as libc::c_int as libc::c_char,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"aug\0\0\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"august\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"bc\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 18 as libc::c_int as libc::c_char,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"d\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 21 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"dec\0\0\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 12 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"december\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 12 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"dow\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 32 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"doy\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 33 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"dst\0\0\0\0\0\0\0\0",
                ),
                type_0: 28 as libc::c_int as libc::c_char,
                value: 3600 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"epoch\0\0\0\0\0\0",
                ),
                type_0: 0 as libc::c_int as libc::c_char,
                value: 11 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"feb\0\0\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"february\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"fri\0\0\0\0\0\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"friday\0\0\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"h\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 20 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"infinity\0\0\0",
                ),
                type_0: 0 as libc::c_int as libc::c_char,
                value: 10 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"isodow\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 37 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"isoyear\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 36 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"j\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 31 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"jan\0\0\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"january\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"jd\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 31 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"jul\0\0\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 7 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"julian\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 31 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"july\0\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 7 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"jun\0\0\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 6 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"june\0\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 6 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"m\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 23 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mar\0\0\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"march\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"may\0\0\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mm\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 19 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mon\0\0\0\0\0\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"monday\0\0\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"nov\0\0\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 11 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"november\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 11 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"now\0\0\0\0\0\0\0\0",
                ),
                type_0: 0 as libc::c_int as libc::c_char,
                value: 12 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"oct\0\0\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 10 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"october\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 10 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"on\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 8 as libc::c_int as libc::c_char,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"pm\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 9 as libc::c_int as libc::c_char,
                value: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"s\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 18 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"sat\0\0\0\0\0\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 6 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"saturday\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 6 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"sep\0\0\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 9 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"sept\0\0\0\0\0\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 9 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"september\0\0",
                ),
                type_0: 1 as libc::c_int as libc::c_char,
                value: 9 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"sun\0\0\0\0\0\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"sunday\0\0\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"t\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 23 as libc::c_int as libc::c_char,
                value: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"thu\0\0\0\0\0\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"thur\0\0\0\0\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"thurs\0\0\0\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"thursday\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"today\0\0\0\0\0\0",
                ),
                type_0: 0 as libc::c_int as libc::c_char,
                value: 14 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"tomorrow\0\0\0",
                ),
                type_0: 0 as libc::c_int as libc::c_char,
                value: 15 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"tue\0\0\0\0\0\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"tues\0\0\0\0\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"tuesday\0\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"wed\0\0\0\0\0\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"wednesday\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"weds\0\0\0\0\0\0\0",
                ),
                type_0: 16 as libc::c_int as libc::c_char,
                value: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"y\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 25 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"yesterday\0\0",
                ),
                type_0: 0 as libc::c_int as libc::c_char,
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
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"@\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 8 as libc::c_int as libc::c_char,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"ago\0\0\0\0\0\0\0\0",
                ),
                type_0: 19 as libc::c_int as libc::c_char,
                value: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"c\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 27 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"cent\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 27 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"centuries\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 27 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"century\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 27 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"d\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 21 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"day\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 21 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"days\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 21 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"dec\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 26 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"decade\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 26 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"decades\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 26 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"decs\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 26 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"h\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 20 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"hour\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 20 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"hours\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 20 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"hr\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 20 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"hrs\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 20 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"m\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 19 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"microsecon\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 30 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mil\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 28 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"millennia\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 28 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"millennium\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 28 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"millisecon\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 29 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mils\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 28 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"min\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 19 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mins\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 19 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"minute\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 19 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"minutes\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 19 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mon\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 23 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mons\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 23 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"month\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 23 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"months\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 23 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"ms\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 29 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"msec\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 29 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"msecond\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 29 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"mseconds\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 29 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"msecs\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 29 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"qtr\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 24 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"quarter\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 24 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"s\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 18 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"sec\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 18 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"second\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 18 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"seconds\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 18 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"secs\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 18 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"timezone\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"timezone_h\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 34 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"timezone_m\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 35 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"us\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 30 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"usec\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 30 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"usecond\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 30 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"useconds\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 30 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"usecs\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 30 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"w\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 22 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"week\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 22 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"weeks\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 22 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"y\0\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 25 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"year\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 25 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"years\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 25 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"yr\0\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
                value: 25 as libc::c_int,
            };
            init
        },
        {
            let mut init = datetkn {
                token: *::core::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(
                    b"yrs\0\0\0\0\0\0\0\0",
                ),
                type_0: 17 as libc::c_int as libc::c_char,
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
#[no_mangle]
pub unsafe extern "C" fn date2j(
    mut y: libc::c_int,
    mut m: libc::c_int,
    mut d: libc::c_int,
) -> libc::c_int {
    let mut julian: libc::c_int = 0;
    let mut century: libc::c_int = 0;
    if m > 2 as libc::c_int {
        m += 1 as libc::c_int;
        y += 4800 as libc::c_int;
    } else {
        m += 13 as libc::c_int;
        y += 4799 as libc::c_int;
    }
    century = y / 100 as libc::c_int;
    julian = y * 365 as libc::c_int - 32167 as libc::c_int;
    julian += y / 4 as libc::c_int - century + century / 4 as libc::c_int;
    julian += 7834 as libc::c_int * m / 256 as libc::c_int + d;
    return julian;
}
#[no_mangle]
pub unsafe extern "C" fn j2date(
    mut jd: libc::c_int,
    mut year: *mut libc::c_int,
    mut month: *mut libc::c_int,
    mut day: *mut libc::c_int,
) {
    let mut julian: libc::c_uint = 0;
    let mut quad: libc::c_uint = 0;
    let mut extra: libc::c_uint = 0;
    let mut y: libc::c_int = 0;
    julian = jd as libc::c_uint;
    julian = julian.wrapping_add(32044 as libc::c_int as libc::c_uint);
    quad = julian.wrapping_div(146097 as libc::c_int as libc::c_uint);
    extra = julian
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
    y = julian
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
#[no_mangle]
pub unsafe extern "C" fn j2day(mut date: libc::c_int) -> libc::c_int {
    date += 1 as libc::c_int;
    date %= 7 as libc::c_int;
    if date < 0 as libc::c_int {
        date += 7 as libc::c_int;
    }
    return date;
}
#[no_mangle]
pub unsafe extern "C" fn GetCurrentDateTime(mut tm: *mut pg_tm) {
    let mut fsec: fsec_t = 0;
    GetCurrentTimeUsec(tm, &mut fsec, 0 as *mut libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn GetCurrentTimeUsec(
    mut tm: *mut pg_tm,
    mut fsec: *mut fsec_t,
    mut tzp: *mut libc::c_int,
) {
    let mut cur_ts: TimestampTz = GetCurrentTransactionStartTimestamp();
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
        tm_isdst: 0,
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
unsafe extern "C" fn AppendSeconds(
    mut cp: *mut libc::c_char,
    mut sec: libc::c_int,
    mut fsec: fsec_t,
    mut precision: libc::c_int,
    mut fillzeros: bool_0,
) -> *mut libc::c_char {
    if fillzeros != 0 {
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
        let mut gotnonzero: bool_0 = 0 as libc::c_int as bool_0;
        let fresh0 = cp;
        cp = cp.offset(1);
        *fresh0 = '.' as i32 as libc::c_char;
        loop {
            let fresh1 = precision;
            precision = precision - 1;
            if !(fresh1 != 0) {
                break;
            }
            let mut oldval: int32 = value;
            let mut remainder: int32 = 0;
            value /= 10 as libc::c_int;
            remainder = oldval - value * 10 as libc::c_int;
            if remainder != 0 {
                gotnonzero = 1 as libc::c_int as bool_0;
            }
            if gotnonzero != 0 {
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
unsafe extern "C" fn AppendTimestampSeconds(
    mut cp: *mut libc::c_char,
    mut tm: *mut pg_tm,
    mut fsec: fsec_t,
) -> *mut libc::c_char {
    return AppendSeconds(
        cp,
        (*tm).tm_sec,
        fsec,
        6 as libc::c_int,
        1 as libc::c_int as bool_0,
    );
}
unsafe extern "C" fn AdjustFractSeconds(
    mut frac: libc::c_double,
    mut tm: *mut pg_tm,
    mut fsec: *mut fsec_t,
    mut scale: libc::c_int,
) {
    let mut sec: libc::c_int = 0;
    if frac == 0 as libc::c_int as libc::c_double {
        return;
    }
    frac *= scale as libc::c_double;
    sec = frac as libc::c_int;
    (*tm).tm_sec += sec;
    frac -= sec as libc::c_double;
    *fsec =
        (*fsec as libc::c_double + rint(frac * 1000000 as libc::c_int as libc::c_double)) as fsec_t;
}
unsafe extern "C" fn AdjustFractDays(
    mut frac: libc::c_double,
    mut tm: *mut pg_tm,
    mut fsec: *mut fsec_t,
    mut scale: libc::c_int,
) {
    let mut extra_days: libc::c_int = 0;
    if frac == 0 as libc::c_int as libc::c_double {
        return;
    }
    frac *= scale as libc::c_double;
    extra_days = frac as libc::c_int;
    (*tm).tm_mday += extra_days;
    frac -= extra_days as libc::c_double;
    AdjustFractSeconds(frac, tm, fsec, 86400 as libc::c_int);
}
unsafe extern "C" fn ParseFractionalSecond(
    mut cp: *mut libc::c_char,
    mut fsec: *mut fsec_t,
) -> libc::c_int {
    let mut frac: libc::c_double = 0.;
    *__errno_location() = 0 as libc::c_int;
    frac = strtod(cp, &mut cp);
    if *cp as libc::c_int != '\0' as i32 || *__errno_location() != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    *fsec = rint(frac * 1000000 as libc::c_int as libc::c_double) as fsec_t;
    return 0 as libc::c_int;
}

/// ParseDateTime()
///  Break string into tokens based on a date/time context.
///  Returns 0 if successful, DTERR code if bogus input detected.
///
/// timestr - the input string
/// workbuf - workspace for field string storage. This must be
///   larger than the largest legal input for this datetime type --
///   some additional space will be needed to NUL terminate fields.
/// buflen - the size of workbuf
/// field[] - pointers to field strings are returned in this array
/// ftype[] - field type indicators are returned in this array
/// maxfields - dimensions of the above two arrays
/// *numfields - set to the actual number of fields detected
///
/// The fields extracted from the input are stored as separate,
/// null-terminated strings in the workspace at workbuf. Any text is
/// converted to lower case.
///
/// Several field types are assigned:
///  DTK_NUMBER - digits and (possibly) a decimal point
///  DTK_DATE - digits and two delimiters, or digits and text
///  DTK_TIME - digits, colon delimiters, and possibly a decimal point
///  DTK_STRING - text (no digits or punctuation)
///  DTK_SPECIAL - leading "+" or "-" followed by text
///  DTK_TZ - leading "+" or "-" followed by digits (also eats ':', '.', '-')
///
/// Note that some field types can hold unexpected items:
///  DTK_NUMBER can hold date fields (yy.ddd)
///  DTK_STRING can hold months (January) and time zones (PST)
///  DTK_DATE can hold time zone names (America/New_York, GMT-8)
#[no_mangle]
pub fn ParseDateTime(
    mut timestr: &str,
    mut fields: &mut Vec<String>,
    mut ftypes: &mut Vec<FieldType>,
    mut maxfields: libc::c_int,
) -> libc::c_int {
    let mut nf: libc::c_int = 0 as libc::c_int;
    let mut cp = timestr.chars().peekable();

    // outer loop through fields
    while cp.peek().is_some() {
        // Ignore spaces between fields
        if cp.peek().unwrap().is_ascii_whitespace() {
            cp.next();
            continue;
        }

        // Record start of current field
        if nf >= maxfields {
            return -(1 as libc::c_int);
        }
        let mut fdata = String::new();
        let mut ftype = FieldType::Number;

        // leading digit? then date or time
        if cp.peek().unwrap().is_ascii_digit() {
            while let Some(c) = cp.next_if(|c| c.is_ascii_digit()) {
                fdata.push(c);
            }

            // time field?
            if *cp.peek().unwrap() == ':' {
                ftype = FieldType::Time;

                while let Some(c) = cp.next_if(|&c| c.is_ascii_digit() || c == ':' || c == '.') {
                    fdata.push(c);
                }
            // date field? allow embedded text month
            } else if matches!(*cp.peek().unwrap(), '-' | '/' | '.') {
                // save delimiting character to use later
                let mut delim = *cp.peek().unwrap();

                fdata.push(cp.next().unwrap());

                // second field is all digits? then no embedded text month
                if cp.peek().unwrap().is_ascii_digit() {
                    ftype = match delim {
                        '.' => FieldType::Number,
                        _ => FieldType::Date,
                    };
                    while let Some(c) = cp.next_if(|c| c.is_ascii_digit()) {
                        fdata.push(c);
                    }

                    // insist that the delimiters match to get a three-field date.
                    if *cp.peek().unwrap() == delim {
                        ftype = FieldType::Date;

                        fdata.push(cp.next().unwrap());
                        while let Some(c) = cp.next_if(|&c| c.is_ascii_digit() || c == delim) {
                            fdata.push(c);
                        }
                    }
                } else {
                    ftype = FieldType::Date;
                    while let Some(c) = cp.next_if(|&c| c.is_ascii_alphanumeric() || c == delim) {
                        fdata.push(c.to_ascii_lowercase());
                    }
                }
            // otherwise, number only and will determine year, month, day, or concatenated fields
            // later..
            } else {
                ftype = FieldType::Number;
            }
        // Leading decimal point? Then fractional seconds...
        } else if *cp.peek().unwrap() == '.' {
            fdata.push(cp.next().unwrap());
            while let Some(c) = cp.next_if(|&c| c.is_ascii_digit()) {
                fdata.push(c);
            }
            ftype = FieldType::Number;
        // text? then date string, month, day of week, special, or timezone
        } else if cp.peek().unwrap().is_ascii_alphabetic() {
            let mut is_date: bool_0 = 0;
            ftype = FieldType::String;
            while let Some(c) = cp.next_if(|&c| c.is_ascii_alphabetic()) {
                fdata.push(c.to_ascii_lowercase());
            }
            // Dates can have embedded '-', '/', or '.' separators.  It could also be a timezone
            // name containing embedded '/', '+', '-', '_', or ':' (but '_' or ':' can't be the
            // first punctuation). If the next character is a digit or '+', we need to check
            // whether what we have so far is a recognized non-timezone keyword --- if so, don't
            // believe that this is the start of a timezone.
            is_date = 0 as libc::c_int as bool_0;
            if matches!(*cp.peek().unwrap(), '-' | '/' | '.') {
                is_date = 1 as libc::c_int as bool_0;
            } else if *cp.peek().unwrap() == '+' || cp.peek().unwrap().is_ascii_digit() {
                // we need search only the core token table, not TZ names
                let cdata = std::ffi::CString::new(fdata.clone()).unwrap();
                unsafe {
                    if (datebsearch(cdata.as_ptr(), datetktbl.as_ptr(), szdatetktbl)).is_null() {
                        is_date = 1 as libc::c_int as bool_0;
                    }
                }
            }
            if is_date != 0 {
                ftype = FieldType::Date;
                fdata.push(cp.next().unwrap().to_ascii_lowercase());
                while let Some(c) = cp.next_if(|&c| {
                    c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '/' | '_' | '.' | ':')
                }) {
                    fdata.push(c.to_ascii_lowercase());
                }
            }
        // sign? then special or numeric timezone
        } else if matches!(*cp.peek().unwrap(), '+' | '-') {
            fdata.push(cp.next().unwrap());
            // soak up leading whitespace
            while cp.next_if(|c| c.is_ascii_whitespace()).is_some() {}
            // numeric timezone?
            // note that "DTK_TZ" could also be a signed float or yyyy-mm
            if cp.peek().unwrap().is_ascii_digit() {
                ftype = FieldType::Tz;
                fdata.push(cp.next().unwrap());
                while let Some(c) =
                    cp.next_if(|&c| c.is_ascii_digit() || matches!(c, ':' | '.' | '-'))
                {
                    fdata.push(c.to_ascii_lowercase());
                }
            // special?
            } else if cp.peek().unwrap().is_ascii_alphabetic() {
                ftype = FieldType::Special;
                while let Some(c) = cp.next_if(|&c| c.is_ascii_alphabetic()) {
                    fdata.push(c.to_ascii_lowercase());
                }
            // otherwise something wrong...
            } else {
                return -(1 as libc::c_int);
            }
        // ignore other punctuation but use as delimiter
        } else if cp.peek().unwrap().is_ascii_punctuation() {
            cp.next();
            continue;
        // otherwise, something is not right...
        } else {
            return -(1 as libc::c_int);
        }
        nf += 1;
        fields.push(fdata);
        ftypes.push(ftype);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DecodeDateTime(
    mut field: *mut *mut libc::c_char,
    mut ftype: *mut libc::c_int,
    mut nf: libc::c_int,
    mut dtype: *mut libc::c_int,
    mut tm: *mut pg_tm,
    mut fsec: *mut fsec_t,
    mut tzp: *mut libc::c_int,
) -> libc::c_int {
    let mut fmask: libc::c_int = 0 as libc::c_int;
    let mut tmask: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut ptype: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut dterr: libc::c_int = 0;
    let mut mer: libc::c_int = 2 as libc::c_int;
    let mut haveTextMonth: bool_0 = 0 as libc::c_int as bool_0;
    let mut isjulian: bool_0 = 0 as libc::c_int as bool_0;
    let mut is2digits: bool_0 = 0 as libc::c_int as bool_0;
    let mut bc: bool_0 = 0 as libc::c_int as bool_0;
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
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    *dtype = 2 as libc::c_int;
    (*tm).tm_hour = 0 as libc::c_int;
    (*tm).tm_min = 0 as libc::c_int;
    (*tm).tm_sec = 0 as libc::c_int;
    *fsec = 0 as libc::c_int;
    (*tm).tm_isdst = -(1 as libc::c_int);
    if !tzp.is_null() {
        *tzp = 0 as libc::c_int;
    }
    let mut current_block_236: u64;
    i = 0 as libc::c_int;
    while i < nf {
        match *ftype.offset(i as isize) {
            2 => {
                if ptype == 31 as libc::c_int {
                    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut val_0: libc::c_int = 0;
                    if tzp.is_null() {
                        return -(1 as libc::c_int);
                    }
                    *__errno_location() = 0 as libc::c_int;
                    val_0 = strtoint(*field.offset(i as isize), &mut cp, 10 as libc::c_int);
                    if *__errno_location() == 34 as libc::c_int || val_0 < 0 as libc::c_int {
                        return -(2 as libc::c_int);
                    }
                    j2date(
                        val_0,
                        &mut (*tm).tm_year,
                        &mut (*tm).tm_mon,
                        &mut (*tm).tm_mday,
                    );
                    isjulian = 1 as libc::c_int as bool_0;
                    dterr = DecodeTimezone(cp, tzp);
                    if dterr != 0 {
                        return dterr;
                    }
                    tmask = (0x1 as libc::c_int) << 2 as libc::c_int
                        | (0x1 as libc::c_int) << 1 as libc::c_int
                        | (0x1 as libc::c_int) << 3 as libc::c_int
                        | ((0x1 as libc::c_int) << 10 as libc::c_int
                            | (0x1 as libc::c_int) << 11 as libc::c_int
                            | ((0x1 as libc::c_int) << 12 as libc::c_int
                                | (0x1 as libc::c_int) << 13 as libc::c_int
                                | (0x1 as libc::c_int) << 14 as libc::c_int))
                        | (0x1 as libc::c_int) << 5 as libc::c_int;
                    ptype = 0 as libc::c_int;
                } else if ptype != 0 as libc::c_int
                    || fmask
                        & ((0x1 as libc::c_int) << 1 as libc::c_int
                            | (0x1 as libc::c_int) << 3 as libc::c_int)
                        == (0x1 as libc::c_int) << 1 as libc::c_int
                            | (0x1 as libc::c_int) << 3 as libc::c_int
                {
                    if tzp.is_null() {
                        return -(1 as libc::c_int);
                    }
                    if *(*__ctype_b_loc())
                        .offset(**field.offset(i as isize) as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                        || ptype != 0 as libc::c_int
                    {
                        let mut cp_0: *mut libc::c_char = 0 as *mut libc::c_char;
                        if ptype != 0 as libc::c_int {
                            if ptype != 3 as libc::c_int {
                                return -(1 as libc::c_int);
                            }
                            ptype = 0 as libc::c_int;
                        }
                        if fmask
                            & ((0x1 as libc::c_int) << 10 as libc::c_int
                                | (0x1 as libc::c_int) << 11 as libc::c_int
                                | ((0x1 as libc::c_int) << 12 as libc::c_int
                                    | (0x1 as libc::c_int) << 13 as libc::c_int
                                    | (0x1 as libc::c_int) << 14 as libc::c_int))
                            == (0x1 as libc::c_int) << 10 as libc::c_int
                                | (0x1 as libc::c_int) << 11 as libc::c_int
                                | ((0x1 as libc::c_int) << 12 as libc::c_int
                                    | (0x1 as libc::c_int) << 13 as libc::c_int
                                    | (0x1 as libc::c_int) << 14 as libc::c_int)
                        {
                            return -(1 as libc::c_int);
                        }
                        cp_0 = strchr(*field.offset(i as isize), '-' as i32);
                        if cp_0.is_null() {
                            return -(1 as libc::c_int);
                        }
                        dterr = DecodeTimezone(cp_0, tzp);
                        if dterr != 0 {
                            return dterr;
                        }
                        *cp_0 = '\0' as i32 as libc::c_char;
                        dterr = DecodeNumberField(
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
                        tmask |= (0x1 as libc::c_int) << 5 as libc::c_int;
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
                        tmask = (0x1 as libc::c_int) << 5 as libc::c_int;
                    }
                } else {
                    dterr = DecodeDate(
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
                if ptype != 0 as libc::c_int {
                    if ptype != 3 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                    ptype = 0 as libc::c_int;
                }
                dterr = DecodeTime(
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
                if time_overflows((*tm).tm_hour, (*tm).tm_min, (*tm).tm_sec, *fsec) != 0 {
                    return -(2 as libc::c_int);
                }
                current_block_236 = 13797367574128857302;
            }
            4 => {
                let mut tz: libc::c_int = 0;
                if tzp.is_null() {
                    return -(1 as libc::c_int);
                }
                dterr = DecodeTimezone(*field.offset(i as isize), &mut tz);
                if dterr != 0 {
                    return dterr;
                }
                *tzp = tz;
                tmask = (0x1 as libc::c_int) << 5 as libc::c_int;
                current_block_236 = 13797367574128857302;
            }
            0 => {
                if ptype != 0 as libc::c_int {
                    let mut cp_1: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut val_1: libc::c_int = 0;
                    *__errno_location() = 0 as libc::c_int;
                    val_1 = strtoint(*field.offset(i as isize), &mut cp_1, 10 as libc::c_int);
                    if *__errno_location() == 34 as libc::c_int {
                        return -(2 as libc::c_int);
                    }
                    if *cp_1 as libc::c_int == '.' as i32 {
                        match ptype {
                            31 | 3 | 18 => {}
                            _ => return -(1 as libc::c_int),
                        }
                    } else if *cp_1 as libc::c_int != '\0' as i32 {
                        return -(1 as libc::c_int);
                    }
                    match ptype {
                        25 => {
                            (*tm).tm_year = val_1;
                            tmask = (0x1 as libc::c_int) << 2 as libc::c_int;
                        }
                        23 => {
                            if fmask & (0x1 as libc::c_int) << 1 as libc::c_int != 0 as libc::c_int
                                && fmask & (0x1 as libc::c_int) << 10 as libc::c_int
                                    != 0 as libc::c_int
                            {
                                (*tm).tm_min = val_1;
                                tmask = (0x1 as libc::c_int) << 11 as libc::c_int;
                            } else {
                                (*tm).tm_mon = val_1;
                                tmask = (0x1 as libc::c_int) << 1 as libc::c_int;
                            }
                        }
                        21 => {
                            (*tm).tm_mday = val_1;
                            tmask = (0x1 as libc::c_int) << 3 as libc::c_int;
                        }
                        20 => {
                            (*tm).tm_hour = val_1;
                            tmask = (0x1 as libc::c_int) << 10 as libc::c_int;
                        }
                        19 => {
                            (*tm).tm_min = val_1;
                            tmask = (0x1 as libc::c_int) << 11 as libc::c_int;
                        }
                        18 => {
                            (*tm).tm_sec = val_1;
                            tmask = (0x1 as libc::c_int) << 12 as libc::c_int;
                            if *cp_1 as libc::c_int == '.' as i32 {
                                dterr = ParseFractionalSecond(cp_1, fsec);
                                if dterr != 0 {
                                    return dterr;
                                }
                                tmask = (0x1 as libc::c_int) << 12 as libc::c_int
                                    | (0x1 as libc::c_int) << 13 as libc::c_int
                                    | (0x1 as libc::c_int) << 14 as libc::c_int;
                            }
                        }
                        4 => {
                            tmask = (0x1 as libc::c_int) << 5 as libc::c_int;
                            dterr = DecodeTimezone(*field.offset(i as isize), tzp);
                            if dterr != 0 {
                                return dterr;
                            }
                        }
                        31 => {
                            if val_1 < 0 as libc::c_int {
                                return -(2 as libc::c_int);
                            }
                            tmask = (0x1 as libc::c_int) << 2 as libc::c_int
                                | (0x1 as libc::c_int) << 1 as libc::c_int
                                | (0x1 as libc::c_int) << 3 as libc::c_int;
                            j2date(
                                val_1,
                                &mut (*tm).tm_year,
                                &mut (*tm).tm_mon,
                                &mut (*tm).tm_mday,
                            );
                            isjulian = 1 as libc::c_int as bool_0;
                            if *cp_1 as libc::c_int == '.' as i32 {
                                let mut time: libc::c_double = 0.;
                                *__errno_location() = 0 as libc::c_int;
                                time = strtod(cp_1, &mut cp_1);
                                if *cp_1 as libc::c_int != '\0' as i32
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
                                tmask |= (0x1 as libc::c_int) << 10 as libc::c_int
                                    | (0x1 as libc::c_int) << 11 as libc::c_int
                                    | ((0x1 as libc::c_int) << 12 as libc::c_int
                                        | (0x1 as libc::c_int) << 13 as libc::c_int
                                        | (0x1 as libc::c_int) << 14 as libc::c_int);
                            }
                        }
                        3 => {
                            dterr = DecodeNumberField(
                                strlen(*field.offset(i as isize)) as libc::c_int,
                                *field.offset(i as isize),
                                fmask
                                    | ((0x1 as libc::c_int) << 2 as libc::c_int
                                        | (0x1 as libc::c_int) << 1 as libc::c_int
                                        | (0x1 as libc::c_int) << 3 as libc::c_int),
                                &mut tmask,
                                tm,
                                fsec,
                                &mut is2digits,
                            );
                            if dterr < 0 as libc::c_int {
                                return dterr;
                            }
                            if tmask
                                != (0x1 as libc::c_int) << 10 as libc::c_int
                                    | (0x1 as libc::c_int) << 11 as libc::c_int
                                    | ((0x1 as libc::c_int) << 12 as libc::c_int
                                        | (0x1 as libc::c_int) << 13 as libc::c_int
                                        | (0x1 as libc::c_int) << 14 as libc::c_int)
                            {
                                return -(1 as libc::c_int);
                            }
                        }
                        _ => return -(1 as libc::c_int),
                    }
                    ptype = 0 as libc::c_int;
                    *dtype = 2 as libc::c_int;
                } else {
                    let mut cp_2: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut flen: libc::c_int = 0;
                    flen = strlen(*field.offset(i as isize)) as libc::c_int;
                    cp_2 = strchr(*field.offset(i as isize), '.' as i32);
                    if !cp_2.is_null()
                        && fmask
                            & ((0x1 as libc::c_int) << 2 as libc::c_int
                                | (0x1 as libc::c_int) << 1 as libc::c_int
                                | (0x1 as libc::c_int) << 3 as libc::c_int)
                            == 0
                    {
                        dterr = DecodeDate(
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
                        dterr = DecodeNumberField(
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
                    } else if flen >= 6 as libc::c_int
                        && (fmask
                            & ((0x1 as libc::c_int) << 2 as libc::c_int
                                | (0x1 as libc::c_int) << 1 as libc::c_int
                                | (0x1 as libc::c_int) << 3 as libc::c_int)
                            == 0
                            || fmask
                                & ((0x1 as libc::c_int) << 10 as libc::c_int
                                    | (0x1 as libc::c_int) << 11 as libc::c_int
                                    | ((0x1 as libc::c_int) << 12 as libc::c_int
                                        | (0x1 as libc::c_int) << 13 as libc::c_int
                                        | (0x1 as libc::c_int) << 14 as libc::c_int))
                                == 0)
                    {
                        dterr = DecodeNumberField(
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
                        dterr = DecodeNumber(
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
                type_0 = DecodeTimezoneAbbrev(i, *field.offset(i as isize), &mut val, &mut valtz);
                if type_0 == 31 as libc::c_int {
                    type_0 = DecodeSpecial(i, *field.offset(i as isize), &mut val);
                }
                if type_0 == 8 as libc::c_int {
                    current_block_236 = 12209867499936983673;
                } else {
                    tmask = (0x1 as libc::c_int) << type_0;
                    match type_0 {
                        0 => match val {
                            12 => {
                                tmask = (0x1 as libc::c_int) << 2 as libc::c_int
                                    | (0x1 as libc::c_int) << 1 as libc::c_int
                                    | (0x1 as libc::c_int) << 3 as libc::c_int
                                    | ((0x1 as libc::c_int) << 10 as libc::c_int
                                        | (0x1 as libc::c_int) << 11 as libc::c_int
                                        | ((0x1 as libc::c_int) << 12 as libc::c_int
                                            | (0x1 as libc::c_int) << 13 as libc::c_int
                                            | (0x1 as libc::c_int) << 14 as libc::c_int))
                                    | (0x1 as libc::c_int) << 5 as libc::c_int;
                                *dtype = 2 as libc::c_int;
                                GetCurrentTimeUsec(tm, fsec, tzp);
                            }
                            13 => {
                                tmask = (0x1 as libc::c_int) << 2 as libc::c_int
                                    | (0x1 as libc::c_int) << 1 as libc::c_int
                                    | (0x1 as libc::c_int) << 3 as libc::c_int;
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
                                tmask = (0x1 as libc::c_int) << 2 as libc::c_int
                                    | (0x1 as libc::c_int) << 1 as libc::c_int
                                    | (0x1 as libc::c_int) << 3 as libc::c_int;
                                *dtype = 2 as libc::c_int;
                                GetCurrentDateTime(&mut cur_tm);
                                (*tm).tm_year = cur_tm.tm_year;
                                (*tm).tm_mon = cur_tm.tm_mon;
                                (*tm).tm_mday = cur_tm.tm_mday;
                            }
                            15 => {
                                tmask = (0x1 as libc::c_int) << 2 as libc::c_int
                                    | (0x1 as libc::c_int) << 1 as libc::c_int
                                    | (0x1 as libc::c_int) << 3 as libc::c_int;
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
                                tmask = (0x1 as libc::c_int) << 10 as libc::c_int
                                    | (0x1 as libc::c_int) << 11 as libc::c_int
                                    | ((0x1 as libc::c_int) << 12 as libc::c_int
                                        | (0x1 as libc::c_int) << 13 as libc::c_int
                                        | (0x1 as libc::c_int) << 14 as libc::c_int)
                                    | (0x1 as libc::c_int) << 5 as libc::c_int;
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
                        1 => {
                            if fmask & (0x1 as libc::c_int) << 1 as libc::c_int != 0
                                && haveTextMonth == 0
                                && fmask & (0x1 as libc::c_int) << 3 as libc::c_int == 0
                                && (*tm).tm_mon >= 1 as libc::c_int
                                && (*tm).tm_mon <= 31 as libc::c_int
                            {
                                (*tm).tm_mday = (*tm).tm_mon;
                                tmask = (0x1 as libc::c_int) << 3 as libc::c_int;
                            }
                            haveTextMonth = 1 as libc::c_int as bool_0;
                            (*tm).tm_mon = val;
                        }
                        28 => {
                            tmask |= (0x1 as libc::c_int) << 6 as libc::c_int;
                            (*tm).tm_isdst = 1 as libc::c_int;
                            if tzp.is_null() {
                                return -(1 as libc::c_int);
                            }
                            *tzp -= val;
                        }
                        6 => {
                            tmask |= (0x1 as libc::c_int) << 5 as libc::c_int;
                            (*tm).tm_isdst = 1 as libc::c_int;
                            if tzp.is_null() {
                                return -(1 as libc::c_int);
                            }
                            *tzp = -val;
                        }
                        5 => {
                            (*tm).tm_isdst = 0 as libc::c_int;
                            if tzp.is_null() {
                                return -(1 as libc::c_int);
                            }
                            *tzp = -val;
                        }
                        7 => {
                            tmask |= (0x1 as libc::c_int) << 5 as libc::c_int;
                            if tzp.is_null() {
                                return -(1 as libc::c_int);
                            }
                            abbrevTz = valtz;
                            abbrev = *field.offset(i as isize);
                        }
                        9 => {
                            mer = val;
                        }
                        18 => {
                            bc = (val == 1 as libc::c_int) as libc::c_int as bool_0;
                        }
                        16 => {
                            (*tm).tm_wday = val;
                        }
                        17 => {
                            tmask = 0 as libc::c_int;
                            ptype = val;
                        }
                        23 => {
                            tmask = 0 as libc::c_int;
                            if fmask
                                & ((0x1 as libc::c_int) << 2 as libc::c_int
                                    | (0x1 as libc::c_int) << 1 as libc::c_int
                                    | (0x1 as libc::c_int) << 3 as libc::c_int)
                                != (0x1 as libc::c_int) << 2 as libc::c_int
                                    | (0x1 as libc::c_int) << 1 as libc::c_int
                                    | (0x1 as libc::c_int) << 3 as libc::c_int
                            {
                                return -(1 as libc::c_int);
                            }
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
                        31 => {
                            namedTz = pg_tzset(*field.offset(i as isize));
                            if namedTz.is_null() {
                                return -(1 as libc::c_int);
                            }
                            tmask = (0x1 as libc::c_int) << 5 as libc::c_int;
                        }
                        _ => return -(1 as libc::c_int),
                    }
                    current_block_236 = 13797367574128857302;
                }
            }
            _ => return -(1 as libc::c_int),
        }
        match current_block_236 {
            13797367574128857302 => {
                if tmask & fmask != 0 {
                    return -(1 as libc::c_int);
                }
                fmask |= tmask;
            }
            _ => {}
        }
        i += 1;
    }
    dterr = ValidateDate(fmask, isjulian, is2digits, bc, tm);
    if dterr != 0 {
        return dterr;
    }
    if mer != 2 as libc::c_int && (*tm).tm_hour > 24 as libc::c_int / 2 as libc::c_int {
        return -(2 as libc::c_int);
    }
    if mer == 0 as libc::c_int && (*tm).tm_hour == 24 as libc::c_int / 2 as libc::c_int {
        (*tm).tm_hour = 0 as libc::c_int;
    } else if mer == 1 as libc::c_int && (*tm).tm_hour != 24 as libc::c_int / 2 as libc::c_int {
        (*tm).tm_hour += 24 as libc::c_int / 2 as libc::c_int;
    }
    if *dtype == 2 as libc::c_int {
        if fmask
            & ((0x1 as libc::c_int) << 2 as libc::c_int
                | (0x1 as libc::c_int) << 1 as libc::c_int
                | (0x1 as libc::c_int) << 3 as libc::c_int)
            != (0x1 as libc::c_int) << 2 as libc::c_int
                | (0x1 as libc::c_int) << 1 as libc::c_int
                | (0x1 as libc::c_int) << 3 as libc::c_int
        {
            if fmask
                & ((0x1 as libc::c_int) << 10 as libc::c_int
                    | (0x1 as libc::c_int) << 11 as libc::c_int
                    | ((0x1 as libc::c_int) << 12 as libc::c_int
                        | (0x1 as libc::c_int) << 13 as libc::c_int
                        | (0x1 as libc::c_int) << 14 as libc::c_int))
                == (0x1 as libc::c_int) << 10 as libc::c_int
                    | (0x1 as libc::c_int) << 11 as libc::c_int
                    | ((0x1 as libc::c_int) << 12 as libc::c_int
                        | (0x1 as libc::c_int) << 13 as libc::c_int
                        | (0x1 as libc::c_int) << 14 as libc::c_int)
            {
                return 1 as libc::c_int;
            }
            return -(1 as libc::c_int);
        }
        if !namedTz.is_null() {
            if fmask & (0x1 as libc::c_int) << 28 as libc::c_int != 0 {
                return -(1 as libc::c_int);
            }
            *tzp = DetermineTimeZoneOffset(tm, namedTz);
        }
        if !abbrevTz.is_null() {
            if fmask & (0x1 as libc::c_int) << 28 as libc::c_int != 0 {
                return -(1 as libc::c_int);
            }
            *tzp = DetermineTimeZoneAbbrevOffset(tm, abbrev, abbrevTz);
        }
        if !tzp.is_null() && fmask & (0x1 as libc::c_int) << 5 as libc::c_int == 0 {
            if fmask & (0x1 as libc::c_int) << 28 as libc::c_int != 0 {
                return -(1 as libc::c_int);
            }
            *tzp = DetermineTimeZoneOffset(tm, session_timezone);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DetermineTimeZoneOffset(
    mut tm: *mut pg_tm,
    mut tzp: *mut pg_tz,
) -> libc::c_int {
    let mut t: pg_time_t = 0;
    return DetermineTimeZoneOffsetInternal(tm, tzp, &mut t);
}
unsafe extern "C" fn DetermineTimeZoneOffsetInternal(
    mut tm: *mut pg_tm,
    mut tzp: *mut pg_tz,
    mut tp: *mut pg_time_t,
) -> libc::c_int {
    let mut date: libc::c_int = 0;
    let mut sec: libc::c_int = 0;
    let mut day: pg_time_t = 0;
    let mut mytime: pg_time_t = 0;
    let mut prevtime: pg_time_t = 0;
    let mut boundary: pg_time_t = 0;
    let mut beforetime: pg_time_t = 0;
    let mut aftertime: pg_time_t = 0;
    let mut before_gmtoff: libc::c_long = 0;
    let mut after_gmtoff: libc::c_long = 0;
    let mut before_isdst: libc::c_int = 0;
    let mut after_isdst: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    if ((*tm).tm_year > -(4713 as libc::c_int)
        || (*tm).tm_year == -(4713 as libc::c_int) && (*tm).tm_mon >= 11 as libc::c_int)
        && ((*tm).tm_year < 5874898 as libc::c_int
            || (*tm).tm_year == 5874898 as libc::c_int && (*tm).tm_mon < 6 as libc::c_int)
    {
        date = date2j((*tm).tm_year, (*tm).tm_mon, (*tm).tm_mday) - 2440588 as libc::c_int;
        day = date as pg_time_t * 86400 as libc::c_int as libc::c_long;
        if !(day / 86400 as libc::c_int as libc::c_long != date as libc::c_long) {
            sec = (*tm).tm_sec
                + ((*tm).tm_min + (*tm).tm_hour * 60 as libc::c_int) * 60 as libc::c_int;
            mytime = day + sec as libc::c_long;
            if !(mytime < 0 as libc::c_int as libc::c_long
                && day > 0 as libc::c_int as libc::c_long)
            {
                prevtime = mytime - 86400 as libc::c_int as libc::c_long;
                if !(mytime < 0 as libc::c_int as libc::c_long
                    && prevtime > 0 as libc::c_int as libc::c_long)
                {
                    res = pg_next_dst_boundary(
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
                            (*tm).tm_isdst = before_isdst;
                            *tp = mytime - before_gmtoff;
                            return -(before_gmtoff as libc::c_int);
                        }
                        beforetime = mytime - before_gmtoff;
                        if !(before_gmtoff > 0 as libc::c_int as libc::c_long
                            && mytime < 0 as libc::c_int as libc::c_long
                            && beforetime > 0 as libc::c_int as libc::c_long
                            || before_gmtoff <= 0 as libc::c_int as libc::c_long
                                && mytime > 0 as libc::c_int as libc::c_long
                                && beforetime < 0 as libc::c_int as libc::c_long)
                        {
                            aftertime = mytime - after_gmtoff;
                            if !(after_gmtoff > 0 as libc::c_int as libc::c_long
                                && mytime < 0 as libc::c_int as libc::c_long
                                && aftertime > 0 as libc::c_int as libc::c_long
                                || after_gmtoff <= 0 as libc::c_int as libc::c_long
                                    && mytime > 0 as libc::c_int as libc::c_long
                                    && aftertime < 0 as libc::c_int as libc::c_long)
                            {
                                if beforetime < boundary && aftertime < boundary {
                                    (*tm).tm_isdst = before_isdst;
                                    *tp = beforetime;
                                    return -(before_gmtoff as libc::c_int);
                                }
                                if beforetime > boundary && aftertime >= boundary {
                                    (*tm).tm_isdst = after_isdst;
                                    *tp = aftertime;
                                    return -(after_gmtoff as libc::c_int);
                                }
                                if beforetime > aftertime {
                                    (*tm).tm_isdst = before_isdst;
                                    *tp = beforetime;
                                    return -(before_gmtoff as libc::c_int);
                                }
                                (*tm).tm_isdst = after_isdst;
                                *tp = aftertime;
                                return -(after_gmtoff as libc::c_int);
                            }
                        }
                    }
                }
            }
        }
    }
    (*tm).tm_isdst = 0 as libc::c_int;
    *tp = 0 as libc::c_int as pg_time_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DetermineTimeZoneAbbrevOffset(
    mut tm: *mut pg_tm,
    mut abbr: *const libc::c_char,
    mut tzp: *mut pg_tz,
) -> libc::c_int {
    let mut t: pg_time_t = 0;
    let mut zone_offset: libc::c_int = 0;
    let mut abbr_offset: libc::c_int = 0;
    let mut abbr_isdst: libc::c_int = 0;
    zone_offset = DetermineTimeZoneOffsetInternal(tm, tzp, &mut t);
    if DetermineTimeZoneAbbrevOffsetInternal(t, abbr, tzp, &mut abbr_offset, &mut abbr_isdst) != 0 {
        (*tm).tm_isdst = abbr_isdst;
        return abbr_offset;
    }
    return zone_offset;
}
#[no_mangle]
pub unsafe extern "C" fn DetermineTimeZoneAbbrevOffsetTS(
    mut ts: TimestampTz,
    mut abbr: *const libc::c_char,
    mut tzp: *mut pg_tz,
    mut isdst: *mut libc::c_int,
) -> libc::c_int {
    let mut t: pg_time_t = timestamptz_to_time_t(ts);
    let mut zone_offset: libc::c_int = 0;
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
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut fsec: fsec_t = 0;
    if DetermineTimeZoneAbbrevOffsetInternal(t, abbr, tzp, &mut abbr_offset, isdst) != 0 {
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
    zone_offset = DetermineTimeZoneOffset(&mut tm, tzp);
    *isdst = tm.tm_isdst;
    return zone_offset;
}
unsafe extern "C" fn DetermineTimeZoneAbbrevOffsetInternal(
    mut t: pg_time_t,
    mut abbr: *const libc::c_char,
    mut tzp: *mut pg_tz,
    mut offset: *mut libc::c_int,
    mut isdst: *mut libc::c_int,
) -> bool_0 {
    let mut upabbr: [libc::c_char; 256] = [0; 256];
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut gmtoff: libc::c_long = 0;
    strlcpy(
        upabbr.as_mut_ptr(),
        abbr,
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
    p = upabbr.as_mut_ptr() as *mut libc::c_uchar;
    while *p != 0 {
        *p = pg_toupper(*p);
        p = p.offset(1);
    }
    if pg_interpret_timezone_abbrev(upabbr.as_mut_ptr(), &mut t, &mut gmtoff, isdst, tzp) != 0 {
        *offset = -gmtoff as libc::c_int;
        return 1 as libc::c_int as bool_0;
    }
    return 0 as libc::c_int as bool_0;
}
#[no_mangle]
pub unsafe extern "C" fn DecodeTimeOnly(
    mut field: *mut *mut libc::c_char,
    mut ftype: *mut libc::c_int,
    mut nf: libc::c_int,
    mut dtype: *mut libc::c_int,
    mut tm: *mut pg_tm,
    mut fsec: *mut fsec_t,
    mut tzp: *mut libc::c_int,
) -> libc::c_int {
    let mut fmask: libc::c_int = 0 as libc::c_int;
    let mut tmask: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut ptype: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut dterr: libc::c_int = 0;
    let mut isjulian: bool_0 = 0 as libc::c_int as bool_0;
    let mut is2digits: bool_0 = 0 as libc::c_int as bool_0;
    let mut bc: bool_0 = 0 as libc::c_int as bool_0;
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
    (*tm).tm_isdst = -(1 as libc::c_int);
    if !tzp.is_null() {
        *tzp = 0 as libc::c_int;
    }
    let mut current_block_201: u64;
    i = 0 as libc::c_int;
    while i < nf {
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
                    dterr = DecodeDate(
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
                    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
                    if fmask
                        & ((0x1 as libc::c_int) << 10 as libc::c_int
                            | (0x1 as libc::c_int) << 11 as libc::c_int
                            | ((0x1 as libc::c_int) << 12 as libc::c_int
                                | (0x1 as libc::c_int) << 13 as libc::c_int
                                | (0x1 as libc::c_int) << 14 as libc::c_int))
                        == (0x1 as libc::c_int) << 10 as libc::c_int
                            | (0x1 as libc::c_int) << 11 as libc::c_int
                            | ((0x1 as libc::c_int) << 12 as libc::c_int
                                | (0x1 as libc::c_int) << 13 as libc::c_int
                                | (0x1 as libc::c_int) << 14 as libc::c_int)
                    {
                        return -(1 as libc::c_int);
                    }
                    cp = strchr(*field.offset(i as isize), '-' as i32);
                    if cp.is_null() {
                        return -(1 as libc::c_int);
                    }
                    dterr = DecodeTimezone(cp, tzp);
                    if dterr != 0 {
                        return dterr;
                    }
                    *cp = '\0' as i32 as libc::c_char;
                    dterr = DecodeNumberField(
                        strlen(*field.offset(i as isize)) as libc::c_int,
                        *field.offset(i as isize),
                        fmask
                            | ((0x1 as libc::c_int) << 2 as libc::c_int
                                | (0x1 as libc::c_int) << 1 as libc::c_int
                                | (0x1 as libc::c_int) << 3 as libc::c_int),
                        &mut tmask,
                        tm,
                        fsec,
                        &mut is2digits,
                    );
                    if dterr < 0 as libc::c_int {
                        return dterr;
                    }
                    *ftype.offset(i as isize) = dterr;
                    tmask |= (0x1 as libc::c_int) << 5 as libc::c_int;
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
                    tmask = (0x1 as libc::c_int) << 5 as libc::c_int;
                }
                current_block_201 = 18009804086567542307;
            }
            3 => {
                dterr = DecodeTime(
                    *field.offset(i as isize),
                    fmask
                        | ((0x1 as libc::c_int) << 2 as libc::c_int
                            | (0x1 as libc::c_int) << 1 as libc::c_int
                            | (0x1 as libc::c_int) << 3 as libc::c_int),
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
                dterr = DecodeTimezone(*field.offset(i as isize), &mut tz);
                if dterr != 0 {
                    return dterr;
                }
                *tzp = tz;
                tmask = (0x1 as libc::c_int) << 5 as libc::c_int;
                current_block_201 = 18009804086567542307;
            }
            0 => {
                if ptype != 0 as libc::c_int {
                    let mut cp_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut val_0: libc::c_int = 0;
                    match ptype {
                        31 | 25 | 23 | 21 => {
                            if tzp.is_null() {
                                return -(1 as libc::c_int);
                            }
                        }
                        _ => {}
                    }
                    *__errno_location() = 0 as libc::c_int;
                    val_0 = strtoint(*field.offset(i as isize), &mut cp_0, 10 as libc::c_int);
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
                            tmask = (0x1 as libc::c_int) << 2 as libc::c_int;
                        }
                        23 => {
                            if fmask & (0x1 as libc::c_int) << 1 as libc::c_int != 0 as libc::c_int
                                && fmask & (0x1 as libc::c_int) << 10 as libc::c_int
                                    != 0 as libc::c_int
                            {
                                (*tm).tm_min = val_0;
                                tmask = (0x1 as libc::c_int) << 11 as libc::c_int;
                            } else {
                                (*tm).tm_mon = val_0;
                                tmask = (0x1 as libc::c_int) << 1 as libc::c_int;
                            }
                        }
                        21 => {
                            (*tm).tm_mday = val_0;
                            tmask = (0x1 as libc::c_int) << 3 as libc::c_int;
                        }
                        20 => {
                            (*tm).tm_hour = val_0;
                            tmask = (0x1 as libc::c_int) << 10 as libc::c_int;
                        }
                        19 => {
                            (*tm).tm_min = val_0;
                            tmask = (0x1 as libc::c_int) << 11 as libc::c_int;
                        }
                        18 => {
                            (*tm).tm_sec = val_0;
                            tmask = (0x1 as libc::c_int) << 12 as libc::c_int;
                            if *cp_0 as libc::c_int == '.' as i32 {
                                dterr = ParseFractionalSecond(cp_0, fsec);
                                if dterr != 0 {
                                    return dterr;
                                }
                                tmask = (0x1 as libc::c_int) << 12 as libc::c_int
                                    | (0x1 as libc::c_int) << 13 as libc::c_int
                                    | (0x1 as libc::c_int) << 14 as libc::c_int;
                            }
                        }
                        4 => {
                            tmask = (0x1 as libc::c_int) << 5 as libc::c_int;
                            dterr = DecodeTimezone(*field.offset(i as isize), tzp);
                            if dterr != 0 {
                                return dterr;
                            }
                        }
                        31 => {
                            if val_0 < 0 as libc::c_int {
                                return -(2 as libc::c_int);
                            }
                            tmask = (0x1 as libc::c_int) << 2 as libc::c_int
                                | (0x1 as libc::c_int) << 1 as libc::c_int
                                | (0x1 as libc::c_int) << 3 as libc::c_int;
                            j2date(
                                val_0,
                                &mut (*tm).tm_year,
                                &mut (*tm).tm_mon,
                                &mut (*tm).tm_mday,
                            );
                            isjulian = 1 as libc::c_int as bool_0;
                            if *cp_0 as libc::c_int == '.' as i32 {
                                let mut time: libc::c_double = 0.;
                                *__errno_location() = 0 as libc::c_int;
                                time = strtod(cp_0, &mut cp_0);
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
                                tmask |= (0x1 as libc::c_int) << 10 as libc::c_int
                                    | (0x1 as libc::c_int) << 11 as libc::c_int
                                    | ((0x1 as libc::c_int) << 12 as libc::c_int
                                        | (0x1 as libc::c_int) << 13 as libc::c_int
                                        | (0x1 as libc::c_int) << 14 as libc::c_int);
                            }
                        }
                        3 => {
                            dterr = DecodeNumberField(
                                strlen(*field.offset(i as isize)) as libc::c_int,
                                *field.offset(i as isize),
                                fmask
                                    | ((0x1 as libc::c_int) << 2 as libc::c_int
                                        | (0x1 as libc::c_int) << 1 as libc::c_int
                                        | (0x1 as libc::c_int) << 3 as libc::c_int),
                                &mut tmask,
                                tm,
                                fsec,
                                &mut is2digits,
                            );
                            if dterr < 0 as libc::c_int {
                                return dterr;
                            }
                            *ftype.offset(i as isize) = dterr;
                            if tmask
                                != (0x1 as libc::c_int) << 10 as libc::c_int
                                    | (0x1 as libc::c_int) << 11 as libc::c_int
                                    | ((0x1 as libc::c_int) << 12 as libc::c_int
                                        | (0x1 as libc::c_int) << 13 as libc::c_int
                                        | (0x1 as libc::c_int) << 14 as libc::c_int)
                            {
                                return -(1 as libc::c_int);
                            }
                        }
                        _ => return -(1 as libc::c_int),
                    }
                    ptype = 0 as libc::c_int;
                    *dtype = 2 as libc::c_int;
                } else {
                    let mut cp_1: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut flen: libc::c_int = 0;
                    flen = strlen(*field.offset(i as isize)) as libc::c_int;
                    cp_1 = strchr(*field.offset(i as isize), '.' as i32);
                    if !cp_1.is_null() {
                        if i == 0 as libc::c_int
                            && nf >= 2 as libc::c_int
                            && *ftype.offset((nf - 1 as libc::c_int) as isize) == 2 as libc::c_int
                        {
                            dterr = DecodeDate(
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
                            dterr = DecodeNumberField(
                                flen,
                                *field.offset(i as isize),
                                fmask
                                    | ((0x1 as libc::c_int) << 2 as libc::c_int
                                        | (0x1 as libc::c_int) << 1 as libc::c_int
                                        | (0x1 as libc::c_int) << 3 as libc::c_int),
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
                        dterr = DecodeNumberField(
                            flen,
                            *field.offset(i as isize),
                            fmask
                                | ((0x1 as libc::c_int) << 2 as libc::c_int
                                    | (0x1 as libc::c_int) << 1 as libc::c_int
                                    | (0x1 as libc::c_int) << 3 as libc::c_int),
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
                        dterr = DecodeNumber(
                            flen,
                            *field.offset(i as isize),
                            0 as libc::c_int as bool_0,
                            fmask
                                | ((0x1 as libc::c_int) << 2 as libc::c_int
                                    | (0x1 as libc::c_int) << 1 as libc::c_int
                                    | (0x1 as libc::c_int) << 3 as libc::c_int),
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
                type_0 = DecodeTimezoneAbbrev(i, *field.offset(i as isize), &mut val, &mut valtz);
                if type_0 == 31 as libc::c_int {
                    type_0 = DecodeSpecial(i, *field.offset(i as isize), &mut val);
                }
                if type_0 == 8 as libc::c_int {
                    current_block_201 = 13536709405535804910;
                } else {
                    tmask = (0x1 as libc::c_int) << type_0;
                    match type_0 {
                        0 => match val {
                            12 => {
                                tmask = (0x1 as libc::c_int) << 10 as libc::c_int
                                    | (0x1 as libc::c_int) << 11 as libc::c_int
                                    | ((0x1 as libc::c_int) << 12 as libc::c_int
                                        | (0x1 as libc::c_int) << 13 as libc::c_int
                                        | (0x1 as libc::c_int) << 14 as libc::c_int);
                                *dtype = 3 as libc::c_int;
                                GetCurrentTimeUsec(tm, fsec, 0 as *mut libc::c_int);
                            }
                            16 => {
                                tmask = (0x1 as libc::c_int) << 10 as libc::c_int
                                    | (0x1 as libc::c_int) << 11 as libc::c_int
                                    | ((0x1 as libc::c_int) << 12 as libc::c_int
                                        | (0x1 as libc::c_int) << 13 as libc::c_int
                                        | (0x1 as libc::c_int) << 14 as libc::c_int)
                                    | (0x1 as libc::c_int) << 5 as libc::c_int;
                                *dtype = 3 as libc::c_int;
                                (*tm).tm_hour = 0 as libc::c_int;
                                (*tm).tm_min = 0 as libc::c_int;
                                (*tm).tm_sec = 0 as libc::c_int;
                                (*tm).tm_isdst = 0 as libc::c_int;
                            }
                            _ => return -(1 as libc::c_int),
                        },
                        28 => {
                            tmask |= (0x1 as libc::c_int) << 6 as libc::c_int;
                            (*tm).tm_isdst = 1 as libc::c_int;
                            if tzp.is_null() {
                                return -(1 as libc::c_int);
                            }
                            *tzp -= val;
                        }
                        6 => {
                            tmask |= (0x1 as libc::c_int) << 5 as libc::c_int;
                            (*tm).tm_isdst = 1 as libc::c_int;
                            if tzp.is_null() {
                                return -(1 as libc::c_int);
                            }
                            *tzp = -val;
                            *ftype.offset(i as isize) = 4 as libc::c_int;
                        }
                        5 => {
                            (*tm).tm_isdst = 0 as libc::c_int;
                            if tzp.is_null() {
                                return -(1 as libc::c_int);
                            }
                            *tzp = -val;
                            *ftype.offset(i as isize) = 4 as libc::c_int;
                        }
                        7 => {
                            tmask |= (0x1 as libc::c_int) << 5 as libc::c_int;
                            if tzp.is_null() {
                                return -(1 as libc::c_int);
                            }
                            abbrevTz = valtz;
                            abbrev = *field.offset(i as isize);
                            *ftype.offset(i as isize) = 4 as libc::c_int;
                        }
                        9 => {
                            mer = val;
                        }
                        18 => {
                            bc = (val == 1 as libc::c_int) as libc::c_int as bool_0;
                        }
                        17 => {
                            tmask = 0 as libc::c_int;
                            ptype = val;
                        }
                        23 => {
                            tmask = 0 as libc::c_int;
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
                        31 => {
                            namedTz = pg_tzset(*field.offset(i as isize));
                            if namedTz.is_null() {
                                return -(1 as libc::c_int);
                            }
                            tmask = (0x1 as libc::c_int) << 5 as libc::c_int;
                        }
                        _ => return -(1 as libc::c_int),
                    }
                    current_block_201 = 18009804086567542307;
                }
            }
            _ => return -(1 as libc::c_int),
        }
        match current_block_201 {
            18009804086567542307 => {
                if tmask & fmask != 0 {
                    return -(1 as libc::c_int);
                }
                fmask |= tmask;
            }
            _ => {}
        }
        i += 1;
    }
    dterr = ValidateDate(fmask, isjulian, is2digits, bc, tm);
    if dterr != 0 {
        return dterr;
    }
    if mer != 2 as libc::c_int && (*tm).tm_hour > 24 as libc::c_int / 2 as libc::c_int {
        return -(2 as libc::c_int);
    }
    if mer == 0 as libc::c_int && (*tm).tm_hour == 24 as libc::c_int / 2 as libc::c_int {
        (*tm).tm_hour = 0 as libc::c_int;
    } else if mer == 1 as libc::c_int && (*tm).tm_hour != 24 as libc::c_int / 2 as libc::c_int {
        (*tm).tm_hour += 24 as libc::c_int / 2 as libc::c_int;
    }
    if time_overflows((*tm).tm_hour, (*tm).tm_min, (*tm).tm_sec, *fsec) != 0 {
        return -(2 as libc::c_int);
    }
    if fmask
        & ((0x1 as libc::c_int) << 10 as libc::c_int
            | (0x1 as libc::c_int) << 11 as libc::c_int
            | ((0x1 as libc::c_int) << 12 as libc::c_int
                | (0x1 as libc::c_int) << 13 as libc::c_int
                | (0x1 as libc::c_int) << 14 as libc::c_int))
        != (0x1 as libc::c_int) << 10 as libc::c_int
            | (0x1 as libc::c_int) << 11 as libc::c_int
            | ((0x1 as libc::c_int) << 12 as libc::c_int
                | (0x1 as libc::c_int) << 13 as libc::c_int
                | (0x1 as libc::c_int) << 14 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    if !namedTz.is_null() {
        let mut gmtoff: libc::c_long = 0;
        if fmask & (0x1 as libc::c_int) << 28 as libc::c_int != 0 {
            return -(1 as libc::c_int);
        }
        if pg_get_timezone_offset(namedTz, &mut gmtoff) != 0 {
            *tzp = -(gmtoff as libc::c_int);
        } else {
            if fmask
                & ((0x1 as libc::c_int) << 2 as libc::c_int
                    | (0x1 as libc::c_int) << 1 as libc::c_int
                    | (0x1 as libc::c_int) << 3 as libc::c_int)
                != (0x1 as libc::c_int) << 2 as libc::c_int
                    | (0x1 as libc::c_int) << 1 as libc::c_int
                    | (0x1 as libc::c_int) << 3 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            *tzp = DetermineTimeZoneOffset(tm, namedTz);
        }
    }
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
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *const libc::c_char,
        };
        let mut tmp: *mut pg_tm = &mut tt;
        if fmask & (0x1 as libc::c_int) << 28 as libc::c_int != 0 {
            return -(1 as libc::c_int);
        }
        if fmask
            & ((0x1 as libc::c_int) << 2 as libc::c_int
                | (0x1 as libc::c_int) << 1 as libc::c_int
                | (0x1 as libc::c_int) << 3 as libc::c_int)
            == 0 as libc::c_int
        {
            GetCurrentDateTime(tmp);
        } else {
            if fmask
                & ((0x1 as libc::c_int) << 2 as libc::c_int
                    | (0x1 as libc::c_int) << 1 as libc::c_int
                    | (0x1 as libc::c_int) << 3 as libc::c_int)
                != (0x1 as libc::c_int) << 2 as libc::c_int
                    | (0x1 as libc::c_int) << 1 as libc::c_int
                    | (0x1 as libc::c_int) << 3 as libc::c_int
            {
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
    if !tzp.is_null() && fmask & (0x1 as libc::c_int) << 5 as libc::c_int == 0 {
        let mut tt_0: pg_tm = pg_tm {
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
        let mut tmp_0: *mut pg_tm = &mut tt_0;
        if fmask & (0x1 as libc::c_int) << 28 as libc::c_int != 0 {
            return -(1 as libc::c_int);
        }
        if fmask
            & ((0x1 as libc::c_int) << 2 as libc::c_int
                | (0x1 as libc::c_int) << 1 as libc::c_int
                | (0x1 as libc::c_int) << 3 as libc::c_int)
            == 0 as libc::c_int
        {
            GetCurrentDateTime(tmp_0);
        } else {
            if fmask
                & ((0x1 as libc::c_int) << 2 as libc::c_int
                    | (0x1 as libc::c_int) << 1 as libc::c_int
                    | (0x1 as libc::c_int) << 3 as libc::c_int)
                != (0x1 as libc::c_int) << 2 as libc::c_int
                    | (0x1 as libc::c_int) << 1 as libc::c_int
                    | (0x1 as libc::c_int) << 3 as libc::c_int
            {
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
unsafe extern "C" fn DecodeDate(
    mut str: *mut libc::c_char,
    mut fmask: libc::c_int,
    mut tmask: *mut libc::c_int,
    mut is2digits: *mut bool_0,
    mut tm: *mut pg_tm,
) -> libc::c_int {
    let mut fsec: fsec_t = 0;
    let mut nf: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut dterr: libc::c_int = 0;
    let mut haveTextMonth: bool_0 = 0 as libc::c_int as bool_0;
    let mut type_0: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut dmask: libc::c_int = 0 as libc::c_int;
    let mut field: [*mut libc::c_char; 25] = [0 as *mut libc::c_char; 25];
    *tmask = 0 as libc::c_int;
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
    i = 0 as libc::c_int;
    while i < nf {
        if *(*__ctype_b_loc()).offset(*field[i as usize] as libc::c_uchar as libc::c_int as isize)
            as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            type_0 = DecodeSpecial(i, field[i as usize], &mut val);
            if !(type_0 == 8 as libc::c_int) {
                dmask = (0x1 as libc::c_int) << type_0;
                match type_0 {
                    1 => {
                        (*tm).tm_mon = val;
                        haveTextMonth = 1 as libc::c_int as bool_0;
                    }
                    _ => return -(1 as libc::c_int),
                }
                if fmask & dmask != 0 {
                    return -(1 as libc::c_int);
                }
                fmask |= dmask;
                *tmask |= dmask;
                field[i as usize] = 0 as *mut libc::c_char;
            }
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < nf {
        if !(field[i as usize]).is_null() {
            len = strlen(field[i as usize]) as libc::c_int;
            if len <= 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            dterr = DecodeNumber(
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
            if fmask & dmask != 0 {
                return -(1 as libc::c_int);
            }
            fmask |= dmask;
            *tmask |= dmask;
        }
        i += 1;
    }
    if fmask
        & !((0x1 as libc::c_int) << 15 as libc::c_int | (0x1 as libc::c_int) << 5 as libc::c_int)
        != (0x1 as libc::c_int) << 2 as libc::c_int
            | (0x1 as libc::c_int) << 1 as libc::c_int
            | (0x1 as libc::c_int) << 3 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ValidateDate(
    mut fmask: libc::c_int,
    mut isjulian: bool_0,
    mut is2digits: bool_0,
    mut bc: bool_0,
    mut tm: *mut pg_tm,
) -> libc::c_int {
    if fmask & (0x1 as libc::c_int) << 2 as libc::c_int != 0 {
        if !(isjulian != 0) {
            if bc != 0 {
                if (*tm).tm_year <= 0 as libc::c_int {
                    return -(2 as libc::c_int);
                }
                (*tm).tm_year = -((*tm).tm_year - 1 as libc::c_int);
            } else if is2digits != 0 {
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
    if fmask & (0x1 as libc::c_int) << 15 as libc::c_int != 0 {
        j2date(
            date2j((*tm).tm_year, 1 as libc::c_int, 1 as libc::c_int) + (*tm).tm_yday
                - 1 as libc::c_int,
            &mut (*tm).tm_year,
            &mut (*tm).tm_mon,
            &mut (*tm).tm_mday,
        );
    }
    if fmask & (0x1 as libc::c_int) << 1 as libc::c_int != 0 {
        if (*tm).tm_mon < 1 as libc::c_int || (*tm).tm_mon > 12 as libc::c_int {
            return -(3 as libc::c_int);
        }
    }
    if fmask & (0x1 as libc::c_int) << 3 as libc::c_int != 0 {
        if (*tm).tm_mday < 1 as libc::c_int || (*tm).tm_mday > 31 as libc::c_int {
            return -(3 as libc::c_int);
        }
    }
    if fmask
        & ((0x1 as libc::c_int) << 2 as libc::c_int
            | (0x1 as libc::c_int) << 1 as libc::c_int
            | (0x1 as libc::c_int) << 3 as libc::c_int)
        == (0x1 as libc::c_int) << 2 as libc::c_int
            | (0x1 as libc::c_int) << 1 as libc::c_int
            | (0x1 as libc::c_int) << 3 as libc::c_int
    {
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
unsafe extern "C" fn DecodeTime(
    mut str: *mut libc::c_char,
    mut fmask: libc::c_int,
    mut range: libc::c_int,
    mut tmask: *mut libc::c_int,
    mut tm: *mut pg_tm,
    mut fsec: *mut fsec_t,
) -> libc::c_int {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dterr: libc::c_int = 0;
    *tmask = (0x1 as libc::c_int) << 10 as libc::c_int
        | (0x1 as libc::c_int) << 11 as libc::c_int
        | ((0x1 as libc::c_int) << 12 as libc::c_int
            | (0x1 as libc::c_int) << 13 as libc::c_int
            | (0x1 as libc::c_int) << 14 as libc::c_int);
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
        dterr = ParseFractionalSecond(cp, fsec);
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
            dterr = ParseFractionalSecond(cp, fsec);
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
unsafe extern "C" fn DecodeNumber(
    mut flen: libc::c_int,
    mut str: *mut libc::c_char,
    mut haveTextMonth: bool_0,
    mut fmask: libc::c_int,
    mut tmask: *mut libc::c_int,
    mut tm: *mut pg_tm,
    mut fsec: *mut fsec_t,
    mut is2digits: *mut bool_0,
) -> libc::c_int {
    let mut val: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dterr: libc::c_int = 0;
    *tmask = 0 as libc::c_int;
    *__errno_location() = 0 as libc::c_int;
    val = strtoint(str, &mut cp, 10 as libc::c_int);
    if *__errno_location() == 34 as libc::c_int {
        return -(2 as libc::c_int);
    }
    if cp == str {
        return -(1 as libc::c_int);
    }
    if *cp as libc::c_int == '.' as i32 {
        if cp.offset_from(str) as libc::c_long > 2 as libc::c_int as libc::c_long {
            dterr = DecodeNumberField(
                flen,
                str,
                fmask
                    | ((0x1 as libc::c_int) << 2 as libc::c_int
                        | (0x1 as libc::c_int) << 1 as libc::c_int
                        | (0x1 as libc::c_int) << 3 as libc::c_int),
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
        dterr = ParseFractionalSecond(cp, fsec);
        if dterr != 0 {
            return dterr;
        }
    } else if *cp as libc::c_int != '\0' as i32 {
        return -(1 as libc::c_int);
    }
    if flen == 3 as libc::c_int
        && fmask
            & ((0x1 as libc::c_int) << 2 as libc::c_int
                | (0x1 as libc::c_int) << 1 as libc::c_int
                | (0x1 as libc::c_int) << 3 as libc::c_int)
            == (0x1 as libc::c_int) << 2 as libc::c_int
        && val >= 1 as libc::c_int
        && val <= 366 as libc::c_int
    {
        *tmask = (0x1 as libc::c_int) << 15 as libc::c_int
            | (0x1 as libc::c_int) << 1 as libc::c_int
            | (0x1 as libc::c_int) << 3 as libc::c_int;
        (*tm).tm_yday = val;
        return 0 as libc::c_int;
    }
    match fmask
        & ((0x1 as libc::c_int) << 2 as libc::c_int
            | (0x1 as libc::c_int) << 1 as libc::c_int
            | (0x1 as libc::c_int) << 3 as libc::c_int)
    {
        0 => {
            if flen >= 3 as libc::c_int || DateOrder == 0 as libc::c_int {
                *tmask = (0x1 as libc::c_int) << 2 as libc::c_int;
                (*tm).tm_year = val;
            } else if DateOrder == 1 as libc::c_int {
                *tmask = (0x1 as libc::c_int) << 3 as libc::c_int;
                (*tm).tm_mday = val;
            } else {
                *tmask = (0x1 as libc::c_int) << 1 as libc::c_int;
                (*tm).tm_mon = val;
            }
        }
        4 => {
            *tmask = (0x1 as libc::c_int) << 1 as libc::c_int;
            (*tm).tm_mon = val;
        }
        2 => {
            if haveTextMonth != 0 {
                if flen >= 3 as libc::c_int || DateOrder == 0 as libc::c_int {
                    *tmask = (0x1 as libc::c_int) << 2 as libc::c_int;
                    (*tm).tm_year = val;
                } else {
                    *tmask = (0x1 as libc::c_int) << 3 as libc::c_int;
                    (*tm).tm_mday = val;
                }
            } else {
                *tmask = (0x1 as libc::c_int) << 3 as libc::c_int;
                (*tm).tm_mday = val;
            }
        }
        6 => {
            if haveTextMonth != 0 {
                if flen >= 3 as libc::c_int && *is2digits as libc::c_int != 0 {
                    *tmask = (0x1 as libc::c_int) << 3 as libc::c_int;
                    (*tm).tm_mday = (*tm).tm_year;
                    (*tm).tm_year = val;
                    *is2digits = 0 as libc::c_int as bool_0;
                } else {
                    *tmask = (0x1 as libc::c_int) << 3 as libc::c_int;
                    (*tm).tm_mday = val;
                }
            } else {
                *tmask = (0x1 as libc::c_int) << 3 as libc::c_int;
                (*tm).tm_mday = val;
            }
        }
        8 => {
            *tmask = (0x1 as libc::c_int) << 1 as libc::c_int;
            (*tm).tm_mon = val;
        }
        10 => {
            *tmask = (0x1 as libc::c_int) << 2 as libc::c_int;
            (*tm).tm_year = val;
        }
        14 => {
            dterr = DecodeNumberField(flen, str, fmask, tmask, tm, fsec, is2digits);
            if dterr < 0 as libc::c_int {
                return dterr;
            }
            return 0 as libc::c_int;
        }
        _ => return -(1 as libc::c_int),
    }
    if *tmask == (0x1 as libc::c_int) << 2 as libc::c_int {
        *is2digits = (flen <= 2 as libc::c_int) as libc::c_int as bool_0;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn DecodeNumberField(
    mut len: libc::c_int,
    mut str: *mut libc::c_char,
    mut fmask: libc::c_int,
    mut tmask: *mut libc::c_int,
    mut tm: *mut pg_tm,
    mut fsec: *mut fsec_t,
    mut is2digits: *mut bool_0,
) -> libc::c_int {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = strchr(str, '.' as i32);
    if !cp.is_null() {
        let mut frac: libc::c_double = 0.;
        *__errno_location() = 0 as libc::c_int;
        frac = strtod(cp, 0 as *mut *mut libc::c_char);
        if *__errno_location() != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        *fsec = rint(frac * 1000000 as libc::c_int as libc::c_double) as fsec_t;
        *cp = '\0' as i32 as libc::c_char;
        len = strlen(str) as libc::c_int;
    } else if fmask
        & ((0x1 as libc::c_int) << 2 as libc::c_int
            | (0x1 as libc::c_int) << 1 as libc::c_int
            | (0x1 as libc::c_int) << 3 as libc::c_int)
        != (0x1 as libc::c_int) << 2 as libc::c_int
            | (0x1 as libc::c_int) << 1 as libc::c_int
            | (0x1 as libc::c_int) << 3 as libc::c_int
    {
        if len >= 6 as libc::c_int {
            *tmask = (0x1 as libc::c_int) << 2 as libc::c_int
                | (0x1 as libc::c_int) << 1 as libc::c_int
                | (0x1 as libc::c_int) << 3 as libc::c_int;
            (*tm).tm_mday = atoi(str.offset((len - 2 as libc::c_int) as isize));
            *str.offset((len - 2 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
            (*tm).tm_mon = atoi(str.offset((len - 4 as libc::c_int) as isize));
            *str.offset((len - 4 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
            (*tm).tm_year = atoi(str);
            if len - 4 as libc::c_int == 2 as libc::c_int {
                *is2digits = 1 as libc::c_int as bool_0;
            }
            return 2 as libc::c_int;
        }
    }
    if fmask
        & ((0x1 as libc::c_int) << 10 as libc::c_int
            | (0x1 as libc::c_int) << 11 as libc::c_int
            | ((0x1 as libc::c_int) << 12 as libc::c_int
                | (0x1 as libc::c_int) << 13 as libc::c_int
                | (0x1 as libc::c_int) << 14 as libc::c_int))
        != (0x1 as libc::c_int) << 10 as libc::c_int
            | (0x1 as libc::c_int) << 11 as libc::c_int
            | ((0x1 as libc::c_int) << 12 as libc::c_int
                | (0x1 as libc::c_int) << 13 as libc::c_int
                | (0x1 as libc::c_int) << 14 as libc::c_int)
    {
        if len == 6 as libc::c_int {
            *tmask = (0x1 as libc::c_int) << 10 as libc::c_int
                | (0x1 as libc::c_int) << 11 as libc::c_int
                | ((0x1 as libc::c_int) << 12 as libc::c_int
                    | (0x1 as libc::c_int) << 13 as libc::c_int
                    | (0x1 as libc::c_int) << 14 as libc::c_int);
            (*tm).tm_sec = atoi(str.offset(4 as libc::c_int as isize));
            *str.offset(4 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            (*tm).tm_min = atoi(str.offset(2 as libc::c_int as isize));
            *str.offset(2 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            (*tm).tm_hour = atoi(str);
            return 3 as libc::c_int;
        } else {
            if len == 4 as libc::c_int {
                *tmask = (0x1 as libc::c_int) << 10 as libc::c_int
                    | (0x1 as libc::c_int) << 11 as libc::c_int
                    | ((0x1 as libc::c_int) << 12 as libc::c_int
                        | (0x1 as libc::c_int) << 13 as libc::c_int
                        | (0x1 as libc::c_int) << 14 as libc::c_int);
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
#[no_mangle]
pub unsafe extern "C" fn DecodeTimezone(
    mut str: *mut libc::c_char,
    mut tzp: *mut libc::c_int,
) -> libc::c_int {
    let mut tz: libc::c_int = 0;
    let mut hr: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    let mut sec: libc::c_int = 0 as libc::c_int;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if *str as libc::c_int != '+' as i32 && *str as libc::c_int != '-' as i32 {
        return -(1 as libc::c_int);
    }
    *__errno_location() = 0 as libc::c_int;
    hr = strtoint(
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
#[no_mangle]
pub unsafe extern "C" fn DecodeTimezoneAbbrev(
    mut field: libc::c_int,
    mut lowtoken: *mut libc::c_char,
    mut offset: *mut libc::c_int,
    mut tz: *mut *mut pg_tz,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut tp: *const datetkn = 0 as *const datetkn;
    tp = abbrevcache[field as usize];
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
        type_0 = 31 as libc::c_int;
        *offset = 0 as libc::c_int;
        *tz = 0 as *mut pg_tz;
    } else {
        abbrevcache[field as usize] = tp;
        type_0 = (*tp).type_0 as libc::c_int;
        if type_0 == 7 as libc::c_int {
            *offset = 0 as libc::c_int;
            *tz = FetchDynamicTimeZone(zoneabbrevtbl, tp);
        } else {
            *offset = (*tp).value;
            *tz = 0 as *mut pg_tz;
        }
    }
    return type_0;
}
#[no_mangle]
pub unsafe extern "C" fn DecodeSpecial(
    mut field: libc::c_int,
    mut lowtoken: *mut libc::c_char,
    mut val: *mut libc::c_int,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut tp: *const datetkn = 0 as *const datetkn;
    tp = datecache[field as usize];
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
        type_0 = 31 as libc::c_int;
        *val = 0 as libc::c_int;
    } else {
        datecache[field as usize] = tp;
        type_0 = (*tp).type_0 as libc::c_int;
        *val = (*tp).value;
    }
    return type_0;
}
#[inline]
unsafe extern "C" fn ClearPgTm(mut tm: *mut pg_tm, mut fsec: *mut fsec_t) {
    (*tm).tm_year = 0 as libc::c_int;
    (*tm).tm_mon = 0 as libc::c_int;
    (*tm).tm_mday = 0 as libc::c_int;
    (*tm).tm_hour = 0 as libc::c_int;
    (*tm).tm_min = 0 as libc::c_int;
    (*tm).tm_sec = 0 as libc::c_int;
    *fsec = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DecodeInterval(
    mut field: *mut *mut libc::c_char,
    mut ftype: *mut libc::c_int,
    mut nf: libc::c_int,
    mut range: libc::c_int,
    mut dtype: *mut libc::c_int,
    mut tm: *mut pg_tm,
    mut fsec: *mut fsec_t,
) -> libc::c_int {
    let mut is_before: bool_0 = 0 as libc::c_int as bool_0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fmask: libc::c_int = 0 as libc::c_int;
    let mut tmask: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut dterr: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut fval: libc::c_double = 0.;
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
                    tmask = 0 as libc::c_int;
                    match type_0 {
                        17 => {
                            type_0 = val;
                        }
                        19 => {
                            is_before = 1 as libc::c_int as bool_0;
                            type_0 = val;
                        }
                        0 => {
                            tmask = (0x1 as libc::c_int) << 2 as libc::c_int
                                | (0x1 as libc::c_int) << 1 as libc::c_int
                                | (0x1 as libc::c_int) << 3 as libc::c_int
                                | ((0x1 as libc::c_int) << 10 as libc::c_int
                                    | (0x1 as libc::c_int) << 11 as libc::c_int
                                    | ((0x1 as libc::c_int) << 12 as libc::c_int
                                        | (0x1 as libc::c_int) << 13 as libc::c_int
                                        | (0x1 as libc::c_int) << 14 as libc::c_int));
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
                    let mut val2: libc::c_int = 0;
                    val2 = strtoint(
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
                tmask = 0 as libc::c_int;
                match type_0 {
                    30 => {
                        *fsec = (*fsec as libc::c_double + rint(val as libc::c_double + fval))
                            as fsec_t;
                        tmask = (0x1 as libc::c_int) << 14 as libc::c_int;
                    }
                    29 => {
                        (*tm).tm_sec += val / 1000 as libc::c_int;
                        val -= val / 1000 as libc::c_int * 1000 as libc::c_int;
                        *fsec = (*fsec as libc::c_double
                            + rint(
                                (val as libc::c_double + fval)
                                    * 1000 as libc::c_int as libc::c_double,
                            )) as fsec_t;
                        tmask = (0x1 as libc::c_int) << 13 as libc::c_int;
                    }
                    18 => {
                        (*tm).tm_sec += val;
                        *fsec = (*fsec as libc::c_double
                            + rint(fval * 1000000 as libc::c_int as libc::c_double))
                            as fsec_t;
                        if fval == 0 as libc::c_int as libc::c_double {
                            tmask = (0x1 as libc::c_int) << 12 as libc::c_int;
                        } else {
                            tmask = (0x1 as libc::c_int) << 12 as libc::c_int
                                | (0x1 as libc::c_int) << 13 as libc::c_int
                                | (0x1 as libc::c_int) << 14 as libc::c_int;
                        }
                    }
                    19 => {
                        (*tm).tm_min += val;
                        AdjustFractSeconds(fval, tm, fsec, 60 as libc::c_int);
                        tmask = (0x1 as libc::c_int) << 11 as libc::c_int;
                    }
                    20 => {
                        (*tm).tm_hour += val;
                        AdjustFractSeconds(fval, tm, fsec, 3600 as libc::c_int);
                        tmask = (0x1 as libc::c_int) << 10 as libc::c_int;
                        type_0 = 21 as libc::c_int;
                    }
                    21 => {
                        (*tm).tm_mday += val;
                        AdjustFractSeconds(fval, tm, fsec, 86400 as libc::c_int);
                        tmask = (0x1 as libc::c_int) << 3 as libc::c_int;
                    }
                    22 => {
                        (*tm).tm_mday += val * 7 as libc::c_int;
                        AdjustFractDays(fval, tm, fsec, 7 as libc::c_int);
                        tmask = (0x1 as libc::c_int) << 24 as libc::c_int;
                    }
                    23 => {
                        (*tm).tm_mon += val;
                        AdjustFractDays(fval, tm, fsec, 30 as libc::c_int);
                        tmask = (0x1 as libc::c_int) << 1 as libc::c_int;
                    }
                    25 => {
                        (*tm).tm_year += val;
                        (*tm).tm_mon = ((*tm).tm_mon as libc::c_double
                            + rint(fval * 12 as libc::c_int as libc::c_double))
                            as libc::c_int;
                        tmask = (0x1 as libc::c_int) << 2 as libc::c_int;
                    }
                    26 => {
                        (*tm).tm_year += val * 10 as libc::c_int;
                        (*tm).tm_mon = ((*tm).tm_mon as libc::c_double
                            + rint(
                                fval * 12 as libc::c_int as libc::c_double
                                    * 10 as libc::c_int as libc::c_double,
                            )) as libc::c_int;
                        tmask = (0x1 as libc::c_int) << 25 as libc::c_int;
                    }
                    27 => {
                        (*tm).tm_year += val * 100 as libc::c_int;
                        (*tm).tm_mon = ((*tm).tm_mon as libc::c_double
                            + rint(
                                fval * 12 as libc::c_int as libc::c_double
                                    * 100 as libc::c_int as libc::c_double,
                            )) as libc::c_int;
                        tmask = (0x1 as libc::c_int) << 26 as libc::c_int;
                    }
                    28 => {
                        (*tm).tm_year += val * 1000 as libc::c_int;
                        (*tm).tm_mon = ((*tm).tm_mon as libc::c_double
                            + rint(
                                fval * 12 as libc::c_int as libc::c_double
                                    * 1000 as libc::c_int as libc::c_double,
                            )) as libc::c_int;
                        tmask = (0x1 as libc::c_int) << 27 as libc::c_int;
                    }
                    _ => return -(1 as libc::c_int),
                }
                current_block_109 = 2793352396589381719;
            }
            _ => {}
        }
        match current_block_109 {
            2793352396589381719 => {
                if tmask & fmask != 0 {
                    return -(1 as libc::c_int);
                }
                fmask |= tmask;
            }
            _ => {}
        }
        i -= 1;
    }
    if fmask == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if *fsec != 0 as libc::c_int {
        let mut sec: libc::c_int = 0;
        sec = (*fsec as libc::c_long / 1000000 as libc::c_long) as libc::c_int;
        *fsec = (*fsec as libc::c_long - sec as libc::c_long * 1000000 as libc::c_long) as fsec_t;
        (*tm).tm_sec += sec;
    }
    if IntervalStyle == 2 as libc::c_int
        && **field.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
    {
        let mut more_signs: bool_0 = 0 as libc::c_int as bool_0;
        i = 1 as libc::c_int;
        while i < nf {
            if **field.offset(i as isize) as libc::c_int == '-' as i32
                || **field.offset(i as isize) as libc::c_int == '+' as i32
            {
                more_signs = 1 as libc::c_int as bool_0;
                break;
            } else {
                i += 1;
            }
        }
        if more_signs == 0 {
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
    if is_before != 0 {
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
unsafe extern "C" fn ParseISO8601Number(
    mut str: *mut libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut ipart: *mut libc::c_int,
    mut fpart: *mut libc::c_double,
) -> libc::c_int {
    let mut val: libc::c_double = 0.;
    if !(*(*__ctype_b_loc()).offset(*str as libc::c_uchar as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        != 0
        || *str as libc::c_int == '-' as i32
        || *str as libc::c_int == '.' as i32)
    {
        return -(1 as libc::c_int);
    }
    *__errno_location() = 0 as libc::c_int;
    val = strtod(str, endptr);
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
unsafe extern "C" fn ISO8601IntegerWidth(mut fieldstart: *mut libc::c_char) -> libc::c_int {
    if *fieldstart as libc::c_int == '-' as i32 {
        fieldstart = fieldstart.offset(1);
    }
    return strspn(
        fieldstart,
        b"0123456789\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DecodeISO8601Interval(
    mut str: *mut libc::c_char,
    mut dtype: *mut libc::c_int,
    mut tm: *mut pg_tm,
    mut fsec: *mut fsec_t,
) -> libc::c_int {
    let mut datepart: bool_0 = 1 as libc::c_int as bool_0;
    let mut havefield: bool_0 = 0 as libc::c_int as bool_0;
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
        let mut fieldstart: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut val: libc::c_int = 0;
        let mut fval: libc::c_double = 0.;
        let mut unit: libc::c_char = 0;
        let mut dterr: libc::c_int = 0;
        if *str as libc::c_int == 'T' as i32 {
            datepart = 0 as libc::c_int as bool_0;
            havefield = 0 as libc::c_int as bool_0;
            str = str.offset(1);
        } else {
            fieldstart = str;
            dterr = ParseISO8601Number(str, &mut str, &mut val, &mut fval);
            if dterr != 0 {
                return dterr;
            }
            let fresh43 = str;
            str = str.offset(1);
            unit = *fresh43;
            if datepart != 0 {
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
                        if ISO8601IntegerWidth(fieldstart) == 8 as libc::c_int && havefield == 0 {
                            (*tm).tm_year += val / 10000 as libc::c_int;
                            (*tm).tm_mon += val / 100 as libc::c_int % 100 as libc::c_int;
                            (*tm).tm_mday += val % 100 as libc::c_int;
                            AdjustFractSeconds(fval, tm, fsec, 86400 as libc::c_int);
                            if unit as libc::c_int == '\0' as i32 {
                                return 0 as libc::c_int;
                            }
                            datepart = 0 as libc::c_int as bool_0;
                            havefield = 0 as libc::c_int as bool_0;
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
                        if havefield != 0 {
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
                            datepart = 0 as libc::c_int as bool_0;
                            havefield = 0 as libc::c_int as bool_0;
                            continue;
                        } else {
                            dterr = ParseISO8601Number(str, &mut str, &mut val, &mut fval);
                            if dterr != 0 {
                                return dterr;
                            }
                            (*tm).tm_mon += val;
                            AdjustFractDays(fval, tm, fsec, 30 as libc::c_int);
                            if *str as libc::c_int == '\0' as i32 {
                                return 0 as libc::c_int;
                            }
                            if *str as libc::c_int == 'T' as i32 {
                                datepart = 0 as libc::c_int as bool_0;
                                havefield = 0 as libc::c_int as bool_0;
                                continue;
                            } else {
                                if *str as libc::c_int != '-' as i32 {
                                    return -(1 as libc::c_int);
                                }
                                str = str.offset(1);
                                dterr = ParseISO8601Number(str, &mut str, &mut val, &mut fval);
                                if dterr != 0 {
                                    return dterr;
                                }
                                (*tm).tm_mday += val;
                                AdjustFractSeconds(fval, tm, fsec, 86400 as libc::c_int);
                                if *str as libc::c_int == '\0' as i32 {
                                    return 0 as libc::c_int;
                                }
                                if *str as libc::c_int == 'T' as i32 {
                                    datepart = 0 as libc::c_int as bool_0;
                                    havefield = 0 as libc::c_int as bool_0;
                                    continue;
                                } else {
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
            } else {
                let mut current_block_97: u64;
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
                        if ISO8601IntegerWidth(fieldstart) == 6 as libc::c_int && havefield == 0 {
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
                        if havefield != 0 {
                            return -(1 as libc::c_int);
                        }
                        (*tm).tm_hour += val;
                        AdjustFractSeconds(fval, tm, fsec, 3600 as libc::c_int);
                        if unit as libc::c_int == '\0' as i32 {
                            return 0 as libc::c_int;
                        }
                        dterr = ParseISO8601Number(str, &mut str, &mut val, &mut fval);
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
                        dterr = ParseISO8601Number(str, &mut str, &mut val, &mut fval);
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
            havefield = 1 as libc::c_int as bool_0;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DecodeUnits(
    mut field: libc::c_int,
    mut lowtoken: *mut libc::c_char,
    mut val: *mut libc::c_int,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut tp: *const datetkn = 0 as *const datetkn;
    tp = deltacache[field as usize];
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
        type_0 = 31 as libc::c_int;
        *val = 0 as libc::c_int;
    } else {
        deltacache[field as usize] = tp;
        type_0 = (*tp).type_0 as libc::c_int;
        *val = (*tp).value;
    }
    return type_0;
}
#[no_mangle]
pub unsafe extern "C" fn DateTimeParseError(
    mut dterr: libc::c_int,
    mut str: *const libc::c_char,
    mut datatype: *const libc::c_char,
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
unsafe extern "C" fn datebsearch(
    mut key: *const libc::c_char,
    mut base: *const datetkn,
    mut nel: libc::c_int,
) -> *const datetkn {
    if nel > 0 as libc::c_int {
        let mut last: *const datetkn = base
            .offset(nel as isize)
            .offset(-(1 as libc::c_int as isize));
        let mut position: *const datetkn = 0 as *const datetkn;
        let mut result: libc::c_int = 0;
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
unsafe extern "C" fn EncodeTimezone(
    mut str: *mut libc::c_char,
    mut tz: libc::c_int,
    mut style: libc::c_int,
) -> *mut libc::c_char {
    let mut hour: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    let mut sec: libc::c_int = 0;
    sec = abs(tz);
    min = sec / 60 as libc::c_int;
    sec -= min * 60 as libc::c_int;
    hour = min / 60 as libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn EncodeDateOnly(
    mut tm: *mut pg_tm,
    mut style: libc::c_int,
    mut str: *mut libc::c_char,
) {
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
#[no_mangle]
pub unsafe extern "C" fn EncodeTimeOnly(
    mut tm: *mut pg_tm,
    mut fsec: fsec_t,
    mut print_tz: bool_0,
    mut tz: libc::c_int,
    mut style: libc::c_int,
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
    str = AppendSeconds(
        str,
        (*tm).tm_sec,
        fsec,
        6 as libc::c_int,
        1 as libc::c_int as bool_0,
    );
    if print_tz != 0 {
        str = EncodeTimezone(str, tz, style);
    }
    *str = '\0' as i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn EncodeDateTime(
    mut tm: *mut pg_tm,
    mut fsec: fsec_t,
    mut print_tz: bool_0,
    mut tz: libc::c_int,
    mut tzn: *const libc::c_char,
    mut style: libc::c_int,
    mut str: *mut libc::c_char,
) {
    let mut day: libc::c_int = 0;
    if (*tm).tm_isdst < 0 as libc::c_int {
        print_tz = 0 as libc::c_int as bool_0;
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
            if print_tz != 0 {
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
            if print_tz != 0 {
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
            if print_tz != 0 {
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
            day = date2j((*tm).tm_year, (*tm).tm_mon, (*tm).tm_mday);
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
            if print_tz != 0 {
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
unsafe extern "C" fn AddISO8601IntPart(
    mut cp: *mut libc::c_char,
    mut value: libc::c_int,
    mut units: libc::c_char,
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
unsafe extern "C" fn AddPostgresIntPart(
    mut cp: *mut libc::c_char,
    mut value: libc::c_int,
    mut units: *const libc::c_char,
    mut is_zero: *mut bool_0,
    mut is_before: *mut bool_0,
) -> *mut libc::c_char {
    if value == 0 as libc::c_int {
        return cp;
    }
    pg_sprintf(
        cp,
        b"%s%s%d %s%s\0" as *const u8 as *const libc::c_char,
        if *is_zero == 0 {
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
    *is_before = (value < 0 as libc::c_int) as libc::c_int as bool_0;
    *is_zero = 0 as libc::c_int as bool_0;
    return cp.offset(strlen(cp) as isize);
}
unsafe extern "C" fn AddVerboseIntPart(
    mut cp: *mut libc::c_char,
    mut value: libc::c_int,
    mut units: *const libc::c_char,
    mut is_zero: *mut bool_0,
    mut is_before: *mut bool_0,
) -> *mut libc::c_char {
    if value == 0 as libc::c_int {
        return cp;
    }
    if *is_zero != 0 {
        *is_before = (value < 0 as libc::c_int) as libc::c_int as bool_0;
        value = abs(value);
    } else if *is_before != 0 {
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
    *is_zero = 0 as libc::c_int as bool_0;
    return cp.offset(strlen(cp) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn EncodeInterval(
    mut tm: *mut pg_tm,
    mut fsec: fsec_t,
    mut style: libc::c_int,
    mut str: *mut libc::c_char,
) {
    let mut cp: *mut libc::c_char = str;
    let mut year: libc::c_int = (*tm).tm_year;
    let mut mon: libc::c_int = (*tm).tm_mon;
    let mut mday: libc::c_int = (*tm).tm_mday;
    let mut hour: libc::c_int = (*tm).tm_hour;
    let mut min: libc::c_int = (*tm).tm_min;
    let mut sec: libc::c_int = (*tm).tm_sec;
    let mut is_before: bool_0 = 0 as libc::c_int as bool_0;
    let mut is_zero: bool_0 = 1 as libc::c_int as bool_0;
    match style {
        2 => {
            let mut has_negative: bool_0 = (year < 0 as libc::c_int
                || mon < 0 as libc::c_int
                || mday < 0 as libc::c_int
                || hour < 0 as libc::c_int
                || min < 0 as libc::c_int
                || sec < 0 as libc::c_int
                || fsec < 0 as libc::c_int)
                as libc::c_int as bool_0;
            let mut has_positive: bool_0 = (year > 0 as libc::c_int
                || mon > 0 as libc::c_int
                || mday > 0 as libc::c_int
                || hour > 0 as libc::c_int
                || min > 0 as libc::c_int
                || sec > 0 as libc::c_int
                || fsec > 0 as libc::c_int)
                as libc::c_int as bool_0;
            let mut has_year_month: bool_0 =
                (year != 0 as libc::c_int || mon != 0 as libc::c_int) as libc::c_int as bool_0;
            let mut has_day_time: bool_0 = (mday != 0 as libc::c_int
                || hour != 0 as libc::c_int
                || min != 0 as libc::c_int
                || sec != 0 as libc::c_int
                || fsec != 0 as libc::c_int)
                as libc::c_int as bool_0;
            let mut has_day: bool_0 = (mday != 0 as libc::c_int) as libc::c_int as bool_0;
            let mut sql_standard_value: bool_0 = (!(has_negative as libc::c_int != 0
                && has_positive as libc::c_int != 0)
                && !(has_year_month as libc::c_int != 0 && has_day_time as libc::c_int != 0))
                as libc::c_int as bool_0;
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
            if has_negative == 0 && has_positive == 0 {
                pg_sprintf(cp, b"0\0" as *const u8 as *const libc::c_char);
            } else if sql_standard_value == 0 {
                let mut year_sign: libc::c_char =
                    (if year < 0 as libc::c_int || mon < 0 as libc::c_int {
                        '-' as i32
                    } else {
                        '+' as i32
                    }) as libc::c_char;
                let mut day_sign: libc::c_char = (if mday < 0 as libc::c_int {
                    '-' as i32
                } else {
                    '+' as i32
                }) as libc::c_char;
                let mut sec_sign: libc::c_char = (if hour < 0 as libc::c_int
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
                cp = AppendSeconds(cp, sec, fsec, 6 as libc::c_int, 1 as libc::c_int as bool_0);
                *cp = '\0' as i32 as libc::c_char;
            } else if has_year_month != 0 {
                pg_sprintf(
                    cp,
                    b"%d-%d\0" as *const u8 as *const libc::c_char,
                    year,
                    mon,
                );
            } else if has_day != 0 {
                pg_sprintf(
                    cp,
                    b"%d %d:%02d:\0" as *const u8 as *const libc::c_char,
                    mday,
                    hour,
                    min,
                );
                cp = cp.offset(strlen(cp) as isize);
                cp = AppendSeconds(cp, sec, fsec, 6 as libc::c_int, 1 as libc::c_int as bool_0);
                *cp = '\0' as i32 as libc::c_char;
            } else {
                pg_sprintf(
                    cp,
                    b"%d:%02d:\0" as *const u8 as *const libc::c_char,
                    hour,
                    min,
                );
                cp = cp.offset(strlen(cp) as isize);
                cp = AppendSeconds(cp, sec, fsec, 6 as libc::c_int, 1 as libc::c_int as bool_0);
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
                    cp = AppendSeconds(cp, sec, fsec, 6 as libc::c_int, 0 as libc::c_int as bool_0);
                    let fresh88 = cp;
                    cp = cp.offset(1);
                    *fresh88 = 'S' as i32 as libc::c_char;
                    let fresh89 = cp;
                    cp = cp.offset(1);
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
                let mut minus: bool_0 = (hour < 0 as libc::c_int
                    || min < 0 as libc::c_int
                    || sec < 0 as libc::c_int
                    || fsec < 0 as libc::c_int)
                    as libc::c_int as bool_0;
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
                cp = AppendSeconds(cp, sec, fsec, 6 as libc::c_int, 1 as libc::c_int as bool_0);
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
                    if is_zero != 0 {
                        is_before = 1 as libc::c_int as bool_0;
                    } else if is_before == 0 {
                        let fresh91 = cp;
                        cp = cp.offset(1);
                        *fresh91 = '-' as i32 as libc::c_char;
                    }
                } else if is_before != 0 {
                    let fresh92 = cp;
                    cp = cp.offset(1);
                    *fresh92 = '-' as i32 as libc::c_char;
                }
                cp = AppendSeconds(cp, sec, fsec, 6 as libc::c_int, 0 as libc::c_int as bool_0);
                pg_sprintf(
                    cp,
                    b" sec%s\0" as *const u8 as *const libc::c_char,
                    if abs(sec) != 1 as libc::c_int || fsec != 0 as libc::c_int {
                        b"s\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                );
                is_zero = 0 as libc::c_int as bool_0;
            }
            if is_zero != 0 {
                strcat(cp, b" 0\0" as *const u8 as *const libc::c_char);
            }
            if is_before != 0 {
                strcat(cp, b" ago\0" as *const u8 as *const libc::c_char);
            }
        }
    };
}
unsafe extern "C" fn CheckDateTokenTable(
    mut tablename: *const libc::c_char,
    mut base: *const datetkn,
    mut nel: libc::c_int,
) -> bool_0 {
    let mut ok: bool_0 = 1 as libc::c_int as bool_0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
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
            ok = 0 as libc::c_int as bool_0;
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
                ok = 0 as libc::c_int as bool_0;
            }
            i += 1;
        }
    }
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn CheckDateTokenTables() -> bool_0 {
    let mut ok: bool_0 = 1 as libc::c_int as bool_0;
    ok = (ok as libc::c_int
        & CheckDateTokenTable(
            b"datetktbl\0" as *const u8 as *const libc::c_char,
            datetktbl.as_ptr(),
            szdatetktbl,
        ) as libc::c_int) as bool_0;
    ok = (ok as libc::c_int
        & CheckDateTokenTable(
            b"deltatktbl\0" as *const u8 as *const libc::c_char,
            deltatktbl.as_ptr(),
            szdeltatktbl,
        ) as libc::c_int) as bool_0;
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn TemporalSimplify(mut max_precis: int32, mut node: *mut Node) -> *mut Node {
    let mut expr: *mut FuncExpr = node as *mut FuncExpr;
    let mut ret: *mut Node = 0 as *mut Node;
    let mut typmod: *mut Node = 0 as *mut Node;
    typmod = (*list_nth_cell((*expr).args, 1 as libc::c_int)).ptr_value as *mut Node;
    if (*(typmod as *const Node)).type_0 as libc::c_uint == T_Const as libc::c_int as libc::c_uint
        && (*(typmod as *mut Const)).constisnull == 0
    {
        let mut source: *mut Node =
            (*list_nth_cell((*expr).args, 0 as libc::c_int)).ptr_value as *mut Node;
        let mut old_precis: int32 = exprTypmod(source);
        let mut new_precis: int32 = (*(typmod as *mut Const)).constvalue as int32;
        if new_precis < 0 as libc::c_int
            || new_precis == max_precis
            || old_precis >= 0 as libc::c_int && new_precis >= old_precis
        {
            ret = relabel_to_typmod(source, new_precis);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ConvertTimeZoneAbbrevs(
    mut abbrevs: *mut tzEntry,
    mut n: libc::c_int,
) -> *mut TimeZoneAbbrevTable {
    let mut tbl: *mut TimeZoneAbbrevTable = 0 as *mut TimeZoneAbbrevTable;
    let mut tbl_size: Size = 0;
    let mut i: libc::c_int = 0;
    tbl_size = (12 as libc::c_ulong).wrapping_add(
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<datetkn>() as libc::c_ulong),
    );
    tbl_size = tbl_size.wrapping_add((8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
        & !((8 as libc::c_int - 1 as libc::c_int) as uintptr_t);
    i = 0 as libc::c_int;
    while i < n {
        let mut abbr: *mut tzEntry = abbrevs.offset(i as isize);
        if !((*abbr).zone).is_null() {
            let mut dsize: Size = 0;
            dsize = (8 as libc::c_ulong)
                .wrapping_add(strlen((*abbr).zone))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            tbl_size = (tbl_size as libc::c_ulong).wrapping_add(
                dsize.wrapping_add((8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                    & !((8 as libc::c_int - 1 as libc::c_int) as uintptr_t),
            ) as Size as Size;
        }
        i += 1;
    }
    tbl = malloc(tbl_size) as *mut TimeZoneAbbrevTable;
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
    i = 0 as libc::c_int;
    while i < n {
        let mut abbr_0: *mut tzEntry = abbrevs.offset(i as isize);
        let mut dtoken: *mut datetkn = ((*tbl).abbrevs).as_mut_ptr().offset(i as isize);
        strlcpy(
            ((*dtoken).token).as_mut_ptr(),
            (*abbr_0).abbrev,
            (10 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        );
        if !((*abbr_0).zone).is_null() {
            let mut dtza: *mut DynamicZoneAbbrev = 0 as *mut DynamicZoneAbbrev;
            let mut dsize_0: Size = 0;
            dtza = (tbl as *mut libc::c_char).offset(tbl_size as isize) as *mut DynamicZoneAbbrev;
            (*dtza).tz = 0 as *mut pg_tz;
            strcpy(((*dtza).zone).as_mut_ptr(), (*abbr_0).zone);
            (*dtoken).type_0 = 7 as libc::c_int as libc::c_char;
            (*dtoken).value = tbl_size as int32;
            dsize_0 = (8 as libc::c_ulong)
                .wrapping_add(strlen((*abbr_0).zone))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            tbl_size = (tbl_size as libc::c_ulong).wrapping_add(
                dsize_0.wrapping_add((8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                    & !((8 as libc::c_int - 1 as libc::c_int) as uintptr_t),
            ) as Size as Size;
        } else {
            (*dtoken).type_0 = (if (*abbr_0).is_dst as libc::c_int != 0 {
                6 as libc::c_int
            } else {
                5 as libc::c_int
            }) as libc::c_char;
            (*dtoken).value = (*abbr_0).offset;
        }
        i += 1;
    }
    return tbl;
}
#[no_mangle]
pub unsafe extern "C" fn InstallTimeZoneAbbrevs(mut tbl: *mut TimeZoneAbbrevTable) {
    zoneabbrevtbl = tbl;
    memset(
        abbrevcache.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[*const datetkn; 25]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn FetchDynamicTimeZone(
    mut tbl: *mut TimeZoneAbbrevTable,
    mut tp: *const datetkn,
) -> *mut pg_tz {
    let mut dtza: *mut DynamicZoneAbbrev = 0 as *mut DynamicZoneAbbrev;
    dtza = (tbl as *mut libc::c_char).offset((*tp).value as isize) as *mut DynamicZoneAbbrev;
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
#[no_mangle]
pub unsafe extern "C" fn pg_timezone_abbrevs(mut fcinfo: FunctionCallInfo) -> Datum {
    let mut funcctx: *mut FuncCallContext = 0 as *mut FuncCallContext;
    let mut pindex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut result: Datum = 0;
    let mut tuple: HeapTuple = 0 as *mut HeapTupleData;
    let mut values: [Datum; 3] = [0; 3];
    let mut nulls: [bool_0; 3] = [0; 3];
    let mut tp: *const datetkn = 0 as *const datetkn;
    let mut buffer: [libc::c_char; 11] = [0; 11];
    let mut gmtoffset: libc::c_int = 0;
    let mut is_dst: bool_0 = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tm: pg_tm = pg_tm {
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
    let mut resInterval: *mut Interval = 0 as *mut Interval;
    if ((*(*fcinfo).flinfo).fn_extra).is_null() {
        let mut tupdesc: TupleDesc = 0 as *mut TupleDescData;
        let mut oldcontext: MemoryContext = 0 as *mut MemoryContextData;
        funcctx = init_MultiFuncCall(fcinfo);
        oldcontext = MemoryContextSwitchTo((*funcctx).multi_call_memory_ctx);
        pindex = palloc(::core::mem::size_of::<libc::c_int>() as libc::c_ulong) as *mut libc::c_int;
        *pindex = 0 as libc::c_int;
        (*funcctx).user_fctx = pindex as *mut libc::c_void;
        tupdesc = CreateTemplateTupleDesc(3 as libc::c_int);
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
        let mut rsi: *mut ReturnSetInfo = 0 as *mut ReturnSetInfo;
        end_MultiFuncCall(fcinfo, funcctx);
        rsi = (*fcinfo).resultinfo as *mut ReturnSetInfo;
        (*rsi).isDone = ExprEndResult;
        (*fcinfo).isnull = 1 as libc::c_int as bool_0;
        return 0 as libc::c_int as Datum;
    }
    tp = ((*zoneabbrevtbl).abbrevs)
        .as_mut_ptr()
        .offset(*pindex as isize);
    match (*tp).type_0 as libc::c_int {
        5 => {
            gmtoffset = (*tp).value;
            is_dst = 0 as libc::c_int as bool_0;
        }
        6 => {
            gmtoffset = (*tp).value;
            is_dst = 1 as libc::c_int as bool_0;
        }
        7 => {
            let mut tzp: *mut pg_tz = 0 as *mut pg_tz;
            let mut now: TimestampTz = 0;
            let mut isdst: libc::c_int = 0;
            tzp = FetchDynamicTimeZone(zoneabbrevtbl, tp);
            now = GetCurrentTransactionStartTimestamp();
            gmtoffset =
                -DetermineTimeZoneAbbrevOffsetTS(now, ((*tp).token).as_ptr(), tzp, &mut isdst);
            is_dst = isdst as bool_0;
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
            is_dst = 0 as libc::c_int as bool_0;
        }
    }
    let mut _vstart: *mut libc::c_void = nulls.as_mut_ptr() as *mut libc::c_void;
    let mut _val: libc::c_int = 0 as libc::c_int;
    let mut _len: Size = ::core::mem::size_of::<[bool_0; 3]>() as libc::c_ulong;
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
    resInterval = palloc(::core::mem::size_of::<Interval>() as libc::c_ulong) as *mut Interval;
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
        nulls.as_mut_ptr(),
    );
    result = HeapTupleHeaderGetDatum((*tuple).t_data);
    let mut rsi_0: *mut ReturnSetInfo = 0 as *mut ReturnSetInfo;
    (*funcctx).call_cntr = ((*funcctx).call_cntr).wrapping_add(1);
    rsi_0 = (*fcinfo).resultinfo as *mut ReturnSetInfo;
    (*rsi_0).isDone = ExprMultipleResult;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn pg_timezone_names(mut fcinfo: FunctionCallInfo) -> Datum {
    let mut rsinfo: *mut ReturnSetInfo = (*fcinfo).resultinfo as *mut ReturnSetInfo;
    let mut randomAccess: bool_0 = 0;
    let mut tupdesc: TupleDesc = 0 as *mut TupleDescData;
    let mut tupstore: *mut Tuplestorestate = 0 as *mut Tuplestorestate;
    let mut tzenum: *mut pg_tzenum = 0 as *mut pg_tzenum;
    let mut tz: *mut pg_tz = 0 as *mut pg_tz;
    let mut values: [Datum; 4] = [0; 4];
    let mut nulls: [bool_0; 4] = [0; 4];
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
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut fsec: fsec_t = 0;
    let mut tzn: *const libc::c_char = 0 as *const libc::c_char;
    let mut resInterval: *mut Interval = 0 as *mut Interval;
    let mut itm: pg_tm = pg_tm {
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
    let mut oldcontext: MemoryContext = 0 as *mut MemoryContextData;
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
    oldcontext = MemoryContextSwitchTo((*(*rsinfo).econtext).ecxt_per_query_memory);
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
    randomAccess = ((*rsinfo).allowedModes & SFRM_Materialize_Random as libc::c_int
        != 0 as libc::c_int) as libc::c_int as bool_0;
    tupstore = tuplestore_begin_heap(randomAccess, 0 as libc::c_int as bool_0, work_mem);
    (*rsinfo).returnMode = SFRM_Materialize;
    (*rsinfo).setResult = tupstore;
    (*rsinfo).setDesc = tupdesc;
    MemoryContextSwitchTo(oldcontext);
    tzenum = pg_tzenumerate_start();
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
        let mut _len: Size = ::core::mem::size_of::<[bool_0; 4]>() as libc::c_ulong;
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
        values[3 as libc::c_int as usize] = (if tm.tm_isdst > 0 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as Datum;
        tuplestore_putvalues(tupstore, tupdesc, values.as_mut_ptr(), nulls.as_mut_ptr());
    }
    pg_tzenumerate_end(tzenum);
    return 0 as libc::c_int as Datum;
}
unsafe extern "C" fn run_static_initializers() {
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
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
