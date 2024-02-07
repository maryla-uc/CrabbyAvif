/* automatically generated by rust-bindgen 0.69.2 */

pub const __BIONIC__: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __bos_level: u32 = 0;
pub const __ANDROID_API_FUTURE__: u32 = 10000;
pub const __ANDROID_API__: u32 = 10000;
pub const __ANDROID_API_G__: u32 = 9;
pub const __ANDROID_API_I__: u32 = 14;
pub const __ANDROID_API_J__: u32 = 16;
pub const __ANDROID_API_J_MR1__: u32 = 17;
pub const __ANDROID_API_J_MR2__: u32 = 18;
pub const __ANDROID_API_K__: u32 = 19;
pub const __ANDROID_API_L__: u32 = 21;
pub const __ANDROID_API_L_MR1__: u32 = 22;
pub const __ANDROID_API_M__: u32 = 23;
pub const __ANDROID_API_N__: u32 = 24;
pub const __ANDROID_API_N_MR1__: u32 = 25;
pub const __ANDROID_API_O__: u32 = 26;
pub const __ANDROID_API_O_MR1__: u32 = 27;
pub const __ANDROID_API_P__: u32 = 28;
pub const __ANDROID_API_Q__: u32 = 29;
pub const __ANDROID_API_R__: u32 = 30;
pub const __ANDROID_API_S__: u32 = 31;
pub const __ANDROID_API_T__: u32 = 33;
pub const __ANDROID_API_U__: u32 = 34;
pub const __ANDROID_NDK__: u32 = 1;
pub const __NDK_MAJOR__: u32 = 25;
pub const __NDK_MINOR__: u32 = 2;
pub const __NDK_BETA__: u32 = 0;
pub const __NDK_BUILD__: u32 = 9519653;
pub const __NDK_CANARY__: u32 = 0;
pub const WCHAR_MIN: u8 = 0u8;
pub const INT8_MIN: i32 = -128;
pub const INT8_MAX: u32 = 127;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST8_MAX: u32 = 127;
pub const UINT8_MAX: u32 = 255;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_FAST8_MAX: u32 = 255;
pub const INT16_MIN: i32 = -32768;
pub const INT16_MAX: u32 = 32767;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const UINT16_MAX: u32 = 65535;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const INT32_MIN: i32 = -2147483648;
pub const INT32_MAX: u32 = 2147483647;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const UINT32_MAX: u32 = 4294967295;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const WINT_MAX: u32 = 4294967295;
pub const WINT_MIN: u32 = 0;
pub const __BITS_PER_LONG: u32 = 64;
pub const __FD_SETSIZE: u32 = 1024;
pub const __bool_true_false_are_defined: u32 = 1;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
extern "C" {
    pub fn android_get_application_target_sdk_version() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn android_get_device_api_level() -> ::std::os::raw::c_int;
}
pub type wchar_t = ::std::os::raw::c_uint;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __uintptr_t = ::std::os::raw::c_ulong;
pub type int_least8_t = i8;
pub type uint_least8_t = u8;
pub type int_least16_t = i16;
pub type uint_least16_t = u16;
pub type int_least32_t = i32;
pub type uint_least32_t = u32;
pub type int_least64_t = i64;
pub type uint_least64_t = u64;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type int_fast64_t = i64;
pub type uint_fast64_t = u64;
pub type int_fast16_t = i64;
pub type uint_fast16_t = u64;
pub type int_fast32_t = i64;
pub type uint_fast32_t = u64;
pub type uintmax_t = u64;
pub type intmax_t = i64;
pub type __s8 = ::std::os::raw::c_schar;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __s16 = ::std::os::raw::c_short;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __s32 = ::std::os::raw::c_int;
pub type __u32 = ::std::os::raw::c_uint;
pub type __s64 = ::std::os::raw::c_longlong;
pub type __u64 = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fd_set {
    pub fds_bits: [::std::os::raw::c_ulong; 16usize],
}
pub type __kernel_sighandler_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
pub type __kernel_key_t = ::std::os::raw::c_int;
pub type __kernel_mqd_t = ::std::os::raw::c_int;
pub type __kernel_old_uid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_gid_t = ::std::os::raw::c_ushort;
pub type __kernel_long_t = ::std::os::raw::c_long;
pub type __kernel_ulong_t = ::std::os::raw::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = ::std::os::raw::c_uint;
pub type __kernel_pid_t = ::std::os::raw::c_int;
pub type __kernel_ipc_pid_t = ::std::os::raw::c_int;
pub type __kernel_uid_t = ::std::os::raw::c_uint;
pub type __kernel_gid_t = ::std::os::raw::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = ::std::os::raw::c_int;
pub type __kernel_uid32_t = ::std::os::raw::c_uint;
pub type __kernel_gid32_t = ::std::os::raw::c_uint;
pub type __kernel_old_dev_t = ::std::os::raw::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fsid_t {
    pub val: [::std::os::raw::c_int; 2usize],
}
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = ::std::os::raw::c_longlong;
pub type __kernel_old_time_t = __kernel_long_t;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_time64_t = ::std::os::raw::c_longlong;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::std::os::raw::c_int;
pub type __kernel_clockid_t = ::std::os::raw::c_int;
pub type __kernel_caddr_t = *mut ::std::os::raw::c_char;
pub type __kernel_uid16_t = ::std::os::raw::c_ushort;
pub type __kernel_gid16_t = ::std::os::raw::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __poll_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pthread_attr_t {
    pub flags: u32,
    pub stack_base: *mut ::std::os::raw::c_void,
    pub stack_size: usize,
    pub guard_size: usize,
    pub sched_policy: i32,
    pub sched_priority: i32,
    pub __reserved: [::std::os::raw::c_char; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pthread_barrier_t {
    pub __private: [i64; 4usize],
}
pub type pthread_barrierattr_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pthread_cond_t {
    pub __private: [i32; 12usize],
}
pub type pthread_condattr_t = ::std::os::raw::c_long;
pub type pthread_key_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pthread_mutex_t {
    pub __private: [i32; 10usize],
}
pub type pthread_mutexattr_t = ::std::os::raw::c_long;
pub type pthread_once_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pthread_rwlock_t {
    pub __private: [i32; 14usize],
}
pub type pthread_rwlockattr_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pthread_spinlock_t {
    pub __private: i64,
}
pub type pthread_t = ::std::os::raw::c_long;
pub type __gid_t = __kernel_gid32_t;
pub type gid_t = __gid_t;
pub type __uid_t = __kernel_uid32_t;
pub type uid_t = __uid_t;
pub type __pid_t = __kernel_pid_t;
pub type pid_t = __pid_t;
pub type __id_t = u32;
pub type id_t = __id_t;
pub type blkcnt_t = ::std::os::raw::c_ulong;
pub type blksize_t = ::std::os::raw::c_ulong;
pub type caddr_t = __kernel_caddr_t;
pub type clock_t = __kernel_clock_t;
pub type __clockid_t = __kernel_clockid_t;
pub type clockid_t = __clockid_t;
pub type daddr_t = __kernel_daddr_t;
pub type fsblkcnt_t = ::std::os::raw::c_ulong;
pub type fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __mode_t = __kernel_mode_t;
pub type mode_t = __mode_t;
pub type __key_t = __kernel_key_t;
pub type key_t = __key_t;
pub type __ino_t = __kernel_ino_t;
pub type ino_t = __ino_t;
pub type ino64_t = u64;
pub type __nlink_t = u32;
pub type nlink_t = __nlink_t;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type timer_t = __timer_t;
pub type __suseconds_t = __kernel_suseconds_t;
pub type suseconds_t = __suseconds_t;
pub type __useconds_t = u32;
pub type useconds_t = __useconds_t;
pub type dev_t = u64;
pub type __time_t = __kernel_time_t;
pub type time_t = __time_t;
pub type off_t = i64;
pub type loff_t = off_t;
pub type off64_t = loff_t;
pub type __socklen_t = u32;
pub type socklen_t = __socklen_t;
pub type __va_list = [u64; 4usize];
pub type uint_t = ::std::os::raw::c_uint;
pub type uint = ::std::os::raw::c_uint;
pub type u_char = ::std::os::raw::c_uchar;
pub type u_short = ::std::os::raw::c_ushort;
pub type u_int = ::std::os::raw::c_uint;
pub type u_long = ::std::os::raw::c_ulong;
pub type u_int32_t = u32;
pub type u_int16_t = u16;
pub type u_int8_t = u8;
pub type u_int64_t = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AMediaCrypto {
    _unused: [u8; 0],
}
pub type AMediaUUID = [u8; 16usize];
extern "C" {
    pub fn AMediaCrypto_isCryptoSchemeSupported(uuid: *mut u8) -> bool;
}
extern "C" {
    pub fn AMediaCrypto_requiresSecureDecoderComponent(mime: *const ::std::os::raw::c_char)
        -> bool;
}
extern "C" {
    pub fn AMediaCrypto_new(
        uuid: *mut u8,
        initData: *const ::std::os::raw::c_void,
        initDataSize: usize,
    ) -> *mut AMediaCrypto;
}
extern "C" {
    pub fn AMediaCrypto_delete(crypto: *mut AMediaCrypto);
}
pub const media_status_t_AMEDIA_OK: media_status_t = 0;
pub const media_status_t_AMEDIACODEC_ERROR_INSUFFICIENT_RESOURCE: media_status_t = 1100;
pub const media_status_t_AMEDIACODEC_ERROR_RECLAIMED: media_status_t = 1101;
pub const media_status_t_AMEDIA_ERROR_BASE: media_status_t = -10000;
pub const media_status_t_AMEDIA_ERROR_UNKNOWN: media_status_t = -10000;
pub const media_status_t_AMEDIA_ERROR_MALFORMED: media_status_t = -10001;
pub const media_status_t_AMEDIA_ERROR_UNSUPPORTED: media_status_t = -10002;
pub const media_status_t_AMEDIA_ERROR_INVALID_OBJECT: media_status_t = -10003;
pub const media_status_t_AMEDIA_ERROR_INVALID_PARAMETER: media_status_t = -10004;
pub const media_status_t_AMEDIA_ERROR_INVALID_OPERATION: media_status_t = -10005;
pub const media_status_t_AMEDIA_ERROR_END_OF_STREAM: media_status_t = -10006;
pub const media_status_t_AMEDIA_ERROR_IO: media_status_t = -10007;
pub const media_status_t_AMEDIA_ERROR_WOULD_BLOCK: media_status_t = -10008;
pub const media_status_t_AMEDIA_DRM_ERROR_BASE: media_status_t = -20000;
pub const media_status_t_AMEDIA_DRM_NOT_PROVISIONED: media_status_t = -20001;
pub const media_status_t_AMEDIA_DRM_RESOURCE_BUSY: media_status_t = -20002;
pub const media_status_t_AMEDIA_DRM_DEVICE_REVOKED: media_status_t = -20003;
pub const media_status_t_AMEDIA_DRM_SHORT_BUFFER: media_status_t = -20004;
pub const media_status_t_AMEDIA_DRM_SESSION_NOT_OPENED: media_status_t = -20005;
pub const media_status_t_AMEDIA_DRM_TAMPER_DETECTED: media_status_t = -20006;
pub const media_status_t_AMEDIA_DRM_VERIFY_FAILED: media_status_t = -20007;
pub const media_status_t_AMEDIA_DRM_NEED_KEY: media_status_t = -20008;
pub const media_status_t_AMEDIA_DRM_LICENSE_EXPIRED: media_status_t = -20009;
pub const media_status_t_AMEDIA_IMGREADER_ERROR_BASE: media_status_t = -30000;
pub const media_status_t_AMEDIA_IMGREADER_NO_BUFFER_AVAILABLE: media_status_t = -30001;
pub const media_status_t_AMEDIA_IMGREADER_MAX_IMAGES_ACQUIRED: media_status_t = -30002;
pub const media_status_t_AMEDIA_IMGREADER_CANNOT_LOCK_IMAGE: media_status_t = -30003;
pub const media_status_t_AMEDIA_IMGREADER_CANNOT_UNLOCK_IMAGE: media_status_t = -30004;
pub const media_status_t_AMEDIA_IMGREADER_IMAGE_NOT_LOCKED: media_status_t = -30005;
pub type media_status_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AMediaFormat {
    _unused: [u8; 0],
}
extern "C" {
    pub fn AMediaFormat_new() -> *mut AMediaFormat;
}
extern "C" {
    pub fn AMediaFormat_delete(arg1: *mut AMediaFormat) -> media_status_t;
}
extern "C" {
    pub fn AMediaFormat_toString(arg1: *mut AMediaFormat) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn AMediaFormat_getInt32(
        arg1: *mut AMediaFormat,
        name: *const ::std::os::raw::c_char,
        out: *mut i32,
    ) -> bool;
}
extern "C" {
    pub fn AMediaFormat_getInt64(
        arg1: *mut AMediaFormat,
        name: *const ::std::os::raw::c_char,
        out: *mut i64,
    ) -> bool;
}
extern "C" {
    pub fn AMediaFormat_getFloat(
        arg1: *mut AMediaFormat,
        name: *const ::std::os::raw::c_char,
        out: *mut f32,
    ) -> bool;
}
extern "C" {
    pub fn AMediaFormat_getSize(
        arg1: *mut AMediaFormat,
        name: *const ::std::os::raw::c_char,
        out: *mut usize,
    ) -> bool;
}
extern "C" {
    pub fn AMediaFormat_getBuffer(
        arg1: *mut AMediaFormat,
        name: *const ::std::os::raw::c_char,
        data: *mut *mut ::std::os::raw::c_void,
        size: *mut usize,
    ) -> bool;
}
extern "C" {
    pub fn AMediaFormat_getString(
        arg1: *mut AMediaFormat,
        name: *const ::std::os::raw::c_char,
        out: *mut *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn AMediaFormat_setInt32(
        arg1: *mut AMediaFormat,
        name: *const ::std::os::raw::c_char,
        value: i32,
    );
}
extern "C" {
    pub fn AMediaFormat_setInt64(
        arg1: *mut AMediaFormat,
        name: *const ::std::os::raw::c_char,
        value: i64,
    );
}
extern "C" {
    pub fn AMediaFormat_setFloat(
        arg1: *mut AMediaFormat,
        name: *const ::std::os::raw::c_char,
        value: f32,
    );
}
extern "C" {
    pub fn AMediaFormat_setString(
        arg1: *mut AMediaFormat,
        name: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn AMediaFormat_setBuffer(
        arg1: *mut AMediaFormat,
        name: *const ::std::os::raw::c_char,
        data: *const ::std::os::raw::c_void,
        size: usize,
    );
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_AAC_DRC_ATTENUATION_FACTOR: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_AAC_DRC_BOOST_FACTOR: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_AAC_DRC_HEAVY_COMPRESSION: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_AAC_DRC_TARGET_REFERENCE_LEVEL: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_AAC_ENCODED_TARGET_LEVEL: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_AAC_MAX_OUTPUT_CHANNEL_COUNT: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_AAC_PROFILE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_AAC_SBR_MODE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_AUDIO_SESSION_ID: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_BITRATE_MODE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_BIT_RATE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CAPTURE_RATE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CHANNEL_COUNT: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CHANNEL_MASK: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_COLOR_FORMAT: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_COLOR_RANGE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_COLOR_STANDARD: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_COLOR_TRANSFER: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_COMPLEXITY: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CSD: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CSD_0: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CSD_1: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CSD_2: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_DISPLAY_CROP: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_DISPLAY_HEIGHT: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_DISPLAY_WIDTH: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_DURATION: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_FLAC_COMPRESSION_LEVEL: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_FRAME_RATE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_GRID_COLUMNS: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_GRID_ROWS: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_HDR_STATIC_INFO: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_HEIGHT: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_INTRA_REFRESH_PERIOD: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_IS_ADTS: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_IS_AUTOSELECT: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_IS_DEFAULT: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_IS_FORCED_SUBTITLE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_I_FRAME_INTERVAL: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_LANGUAGE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_LATENCY: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_LEVEL: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_MAX_HEIGHT: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_MAX_INPUT_SIZE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_MAX_WIDTH: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_MIME: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_MPEG_USER_DATA: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_OPERATING_RATE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_PCM_ENCODING: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_PRIORITY: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_PROFILE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_PUSH_BLANK_BUFFERS_ON_STOP: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_REPEAT_PREVIOUS_FRAME_AFTER: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_ROTATION: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_SAMPLE_RATE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_SEI: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_SLICE_HEIGHT: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_STRIDE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_TEMPORAL_LAYER_ID: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_TEMPORAL_LAYERING: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_TILE_HEIGHT: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_TILE_WIDTH: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_TIME_US: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_TRACK_ID: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_TRACK_INDEX: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_WIDTH: *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn AMediaFormat_getDouble(
        arg1: *mut AMediaFormat,
        name: *const ::std::os::raw::c_char,
        out: *mut f64,
    ) -> bool;
}
extern "C" {
    pub fn AMediaFormat_getRect(
        arg1: *mut AMediaFormat,
        name: *const ::std::os::raw::c_char,
        left: *mut i32,
        top: *mut i32,
        right: *mut i32,
        bottom: *mut i32,
    ) -> bool;
}
extern "C" {
    pub fn AMediaFormat_setDouble(
        arg1: *mut AMediaFormat,
        name: *const ::std::os::raw::c_char,
        value: f64,
    );
}
extern "C" {
    pub fn AMediaFormat_setSize(
        arg1: *mut AMediaFormat,
        name: *const ::std::os::raw::c_char,
        value: usize,
    );
}
extern "C" {
    pub fn AMediaFormat_setRect(
        arg1: *mut AMediaFormat,
        name: *const ::std::os::raw::c_char,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    );
}
extern "C" {
    pub fn AMediaFormat_clear(arg1: *mut AMediaFormat);
}
extern "C" {
    pub fn AMediaFormat_copy(to: *mut AMediaFormat, from: *mut AMediaFormat) -> media_status_t;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_ALBUM: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_ALBUMART: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_ALBUMARTIST: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_ARTIST: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_AUDIO_PRESENTATION_INFO: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_AUDIO_PRESENTATION_PRESENTATION_ID:
        *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_AUDIO_PRESENTATION_PROGRAM_ID: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_AUTHOR: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_BITS_PER_SAMPLE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CDTRACKNUMBER: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_COMPILATION: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_COMPOSER: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CREATE_INPUT_SURFACE_SUSPENDED: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CRYPTO_DEFAULT_IV_SIZE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CRYPTO_ENCRYPTED_BYTE_BLOCK: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CRYPTO_ENCRYPTED_SIZES: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CRYPTO_IV: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CRYPTO_KEY: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CRYPTO_MODE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CRYPTO_PLAIN_SIZES: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CRYPTO_SKIP_BYTE_BLOCK: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CSD_AVC: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_CSD_HEVC: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_D263: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_DATE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_DISCNUMBER: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_ENCODER_DELAY: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_ENCODER_PADDING: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_ESDS: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_EXIF_OFFSET: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_EXIF_SIZE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_FRAME_COUNT: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_GENRE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_HAPTIC_CHANNEL_COUNT: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_ICC_PROFILE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_IS_SYNC_FRAME: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_LOCATION: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_LOOP: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_LYRICIST: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_MANUFACTURER: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_MAX_BIT_RATE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_MAX_FPS_TO_ENCODER: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_MAX_PTS_GAP_TO_ENCODER: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_MPEG2_STREAM_HEADER: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_PCM_BIG_ENDIAN: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_PSSH: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_SAR_HEIGHT: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_SAR_WIDTH: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_TARGET_TIME: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_TEMPORAL_LAYER_COUNT: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_TEXT_FORMAT_DATA: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_THUMBNAIL_CSD_HEVC: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_THUMBNAIL_HEIGHT: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_THUMBNAIL_TIME: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_THUMBNAIL_WIDTH: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_TITLE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_VALID_SAMPLES: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_YEAR: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_LOW_LATENCY: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_HDR10_PLUS_INFO: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_SLOW_MOTION_MARKERS: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_THUMBNAIL_CSD_AV1C: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_XMP_OFFSET: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_XMP_SIZE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_SAMPLE_FILE_OFFSET: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_LAST_SAMPLE_INDEX_IN_CHUNK: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_SAMPLE_TIME_BEFORE_APPEND: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_PICTURE_TYPE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_VIDEO_ENCODING_STATISTICS_LEVEL: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_VIDEO_QP_AVERAGE: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_VIDEO_QP_B_MAX: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_VIDEO_QP_B_MIN: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_VIDEO_QP_I_MAX: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_VIDEO_QP_I_MIN: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_VIDEO_QP_MAX: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_VIDEO_QP_MIN: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_VIDEO_QP_P_MAX: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_VIDEO_QP_P_MIN: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_MPEGH_COMPATIBLE_SETS: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_MPEGH_PROFILE_LEVEL_INDICATION: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIAFORMAT_KEY_MPEGH_REFERENCE_CHANNEL_LAYOUT: *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ANativeWindow {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AMediaCodec {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AMediaCodecBufferInfo {
    pub offset: i32,
    pub size: i32,
    pub presentationTimeUs: i64,
    pub flags: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AMediaCodecCryptoInfo {
    _unused: [u8; 0],
}
pub const AMEDIACODEC_BUFFER_FLAG_CODEC_CONFIG: _bindgen_ty_1 = 2;
pub const AMEDIACODEC_BUFFER_FLAG_END_OF_STREAM: _bindgen_ty_1 = 4;
pub const AMEDIACODEC_BUFFER_FLAG_PARTIAL_FRAME: _bindgen_ty_1 = 8;
pub const AMEDIACODEC_CONFIGURE_FLAG_ENCODE: _bindgen_ty_1 = 1;
pub const AMEDIACODEC_INFO_OUTPUT_BUFFERS_CHANGED: _bindgen_ty_1 = -3;
pub const AMEDIACODEC_INFO_OUTPUT_FORMAT_CHANGED: _bindgen_ty_1 = -2;
pub const AMEDIACODEC_INFO_TRY_AGAIN_LATER: _bindgen_ty_1 = -1;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
pub type AMediaCodecOnAsyncInputAvailable = ::std::option::Option<
    unsafe extern "C" fn(
        codec: *mut AMediaCodec,
        userdata: *mut ::std::os::raw::c_void,
        index: i32,
    ),
>;
pub type AMediaCodecOnAsyncOutputAvailable = ::std::option::Option<
    unsafe extern "C" fn(
        codec: *mut AMediaCodec,
        userdata: *mut ::std::os::raw::c_void,
        index: i32,
        bufferInfo: *mut AMediaCodecBufferInfo,
    ),
>;
pub type AMediaCodecOnAsyncFormatChanged = ::std::option::Option<
    unsafe extern "C" fn(
        codec: *mut AMediaCodec,
        userdata: *mut ::std::os::raw::c_void,
        format: *mut AMediaFormat,
    ),
>;
pub type AMediaCodecOnAsyncError = ::std::option::Option<
    unsafe extern "C" fn(
        codec: *mut AMediaCodec,
        userdata: *mut ::std::os::raw::c_void,
        error: media_status_t,
        actionCode: i32,
        detail: *const ::std::os::raw::c_char,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AMediaCodecOnAsyncNotifyCallback {
    pub onAsyncInputAvailable: AMediaCodecOnAsyncInputAvailable,
    pub onAsyncOutputAvailable: AMediaCodecOnAsyncOutputAvailable,
    pub onAsyncFormatChanged: AMediaCodecOnAsyncFormatChanged,
    pub onAsyncError: AMediaCodecOnAsyncError,
}
pub type AMediaCodecOnFrameRendered = ::std::option::Option<
    unsafe extern "C" fn(
        codec: *mut AMediaCodec,
        userdata: *mut ::std::os::raw::c_void,
        mediaTimeUs: i64,
        systemNano: i64,
    ),
>;
extern "C" {
    pub fn AMediaCodec_createCodecByName(name: *const ::std::os::raw::c_char) -> *mut AMediaCodec;
}
extern "C" {
    pub fn AMediaCodec_createDecoderByType(
        mime_type: *const ::std::os::raw::c_char,
    ) -> *mut AMediaCodec;
}
extern "C" {
    pub fn AMediaCodec_createEncoderByType(
        mime_type: *const ::std::os::raw::c_char,
    ) -> *mut AMediaCodec;
}
extern "C" {
    pub fn AMediaCodec_delete(arg1: *mut AMediaCodec) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_configure(
        arg1: *mut AMediaCodec,
        format: *const AMediaFormat,
        surface: *mut ANativeWindow,
        crypto: *mut AMediaCrypto,
        flags: u32,
    ) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_start(arg1: *mut AMediaCodec) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_stop(arg1: *mut AMediaCodec) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_flush(arg1: *mut AMediaCodec) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_getInputBuffer(
        arg1: *mut AMediaCodec,
        idx: usize,
        out_size: *mut usize,
    ) -> *mut u8;
}
extern "C" {
    pub fn AMediaCodec_getOutputBuffer(
        arg1: *mut AMediaCodec,
        idx: usize,
        out_size: *mut usize,
    ) -> *mut u8;
}
extern "C" {
    pub fn AMediaCodec_dequeueInputBuffer(arg1: *mut AMediaCodec, timeoutUs: i64) -> isize;
}
extern "C" {
    pub fn __assert(
        __file: *const ::std::os::raw::c_char,
        __line: ::std::os::raw::c_int,
        __msg: *const ::std::os::raw::c_char,
    ) -> !;
}
extern "C" {
    pub fn __assert2(
        __file: *const ::std::os::raw::c_char,
        __line: ::std::os::raw::c_int,
        __function: *const ::std::os::raw::c_char,
        __msg: *const ::std::os::raw::c_char,
    ) -> !;
}
extern "C" {
    pub fn AMediaCodec_queueInputBuffer(
        arg1: *mut AMediaCodec,
        idx: usize,
        offset: off_t,
        size: usize,
        time: u64,
        flags: u32,
    ) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_queueSecureInputBuffer(
        arg1: *mut AMediaCodec,
        idx: usize,
        offset: off_t,
        arg2: *mut AMediaCodecCryptoInfo,
        time: u64,
        flags: u32,
    ) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_dequeueOutputBuffer(
        arg1: *mut AMediaCodec,
        info: *mut AMediaCodecBufferInfo,
        timeoutUs: i64,
    ) -> isize;
}
extern "C" {
    pub fn AMediaCodec_getOutputFormat(arg1: *mut AMediaCodec) -> *mut AMediaFormat;
}
extern "C" {
    pub fn AMediaCodec_releaseOutputBuffer(
        arg1: *mut AMediaCodec,
        idx: usize,
        render: bool,
    ) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_setOutputSurface(
        arg1: *mut AMediaCodec,
        surface: *mut ANativeWindow,
    ) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_releaseOutputBufferAtTime(
        mData: *mut AMediaCodec,
        idx: usize,
        timestampNs: i64,
    ) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_createInputSurface(
        mData: *mut AMediaCodec,
        surface: *mut *mut ANativeWindow,
    ) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_createPersistentInputSurface(
        surface: *mut *mut ANativeWindow,
    ) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_setInputSurface(
        mData: *mut AMediaCodec,
        surface: *mut ANativeWindow,
    ) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_setParameters(
        mData: *mut AMediaCodec,
        params: *const AMediaFormat,
    ) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_signalEndOfInputStream(mData: *mut AMediaCodec) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_getBufferFormat(arg1: *mut AMediaCodec, index: usize) -> *mut AMediaFormat;
}
extern "C" {
    pub fn AMediaCodec_getName(
        arg1: *mut AMediaCodec,
        out_name: *mut *mut ::std::os::raw::c_char,
    ) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_releaseName(arg1: *mut AMediaCodec, name: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn AMediaCodec_setAsyncNotifyCallback(
        arg1: *mut AMediaCodec,
        callback: AMediaCodecOnAsyncNotifyCallback,
        userdata: *mut ::std::os::raw::c_void,
    ) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_setOnFrameRenderedCallback(
        arg1: *mut AMediaCodec,
        callback: AMediaCodecOnFrameRendered,
        userdata: *mut ::std::os::raw::c_void,
    ) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_releaseCrypto(arg1: *mut AMediaCodec) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodec_getInputFormat(arg1: *mut AMediaCodec) -> *mut AMediaFormat;
}
extern "C" {
    pub fn AMediaCodecActionCode_isRecoverable(actionCode: i32) -> bool;
}
extern "C" {
    pub fn AMediaCodecActionCode_isTransient(actionCode: i32) -> bool;
}
pub const cryptoinfo_mode_t_AMEDIACODECRYPTOINFO_MODE_CLEAR: cryptoinfo_mode_t = 0;
pub const cryptoinfo_mode_t_AMEDIACODECRYPTOINFO_MODE_AES_CTR: cryptoinfo_mode_t = 1;
pub const cryptoinfo_mode_t_AMEDIACODECRYPTOINFO_MODE_AES_WV: cryptoinfo_mode_t = 2;
pub const cryptoinfo_mode_t_AMEDIACODECRYPTOINFO_MODE_AES_CBC: cryptoinfo_mode_t = 3;
pub type cryptoinfo_mode_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cryptoinfo_pattern_t {
    pub encryptBlocks: i32,
    pub skipBlocks: i32,
}
extern "C" {
    pub fn AMediaCodecCryptoInfo_new(
        numsubsamples: ::std::os::raw::c_int,
        key: *mut u8,
        iv: *mut u8,
        mode: cryptoinfo_mode_t,
        clearbytes: *mut usize,
        encryptedbytes: *mut usize,
    ) -> *mut AMediaCodecCryptoInfo;
}
extern "C" {
    pub fn AMediaCodecCryptoInfo_delete(arg1: *mut AMediaCodecCryptoInfo) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodecCryptoInfo_setPattern(
        info: *mut AMediaCodecCryptoInfo,
        pattern: *mut cryptoinfo_pattern_t,
    );
}
extern "C" {
    pub fn AMediaCodecCryptoInfo_getNumSubSamples(arg1: *mut AMediaCodecCryptoInfo) -> usize;
}
extern "C" {
    pub fn AMediaCodecCryptoInfo_getKey(
        arg1: *mut AMediaCodecCryptoInfo,
        dst: *mut u8,
    ) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodecCryptoInfo_getIV(
        arg1: *mut AMediaCodecCryptoInfo,
        dst: *mut u8,
    ) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodecCryptoInfo_getMode(arg1: *mut AMediaCodecCryptoInfo) -> cryptoinfo_mode_t;
}
extern "C" {
    pub fn AMediaCodecCryptoInfo_getClearBytes(
        arg1: *mut AMediaCodecCryptoInfo,
        dst: *mut usize,
    ) -> media_status_t;
}
extern "C" {
    pub fn AMediaCodecCryptoInfo_getEncryptedBytes(
        arg1: *mut AMediaCodecCryptoInfo,
        dst: *mut usize,
    ) -> media_status_t;
}
extern "C" {
    pub static mut AMEDIACODEC_KEY_HDR10_PLUS_INFO: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIACODEC_KEY_LOW_LATENCY: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIACODEC_KEY_OFFSET_TIME: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIACODEC_KEY_REQUEST_SYNC_FRAME: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIACODEC_KEY_SUSPEND: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIACODEC_KEY_SUSPEND_TIME: *const ::std::os::raw::c_char;
}
extern "C" {
    pub static mut AMEDIACODEC_KEY_VIDEO_BITRATE: *const ::std::os::raw::c_char;
}
