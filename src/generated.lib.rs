#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub const LIBSSH_VERSION_MAJOR: u32 = 0;
pub const LIBSSH_VERSION_MINOR: u32 = 9;
pub const LIBSSH_VERSION_MICRO: u32 = 5;
pub const _UNISTD_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 0;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 31;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __LONG_DOUBLE_USES_FLOAT128: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const _POSIX_VERSION: u32 = 200809;
pub const __POSIX2_THIS_VERSION: u32 = 200809;
pub const _POSIX2_VERSION: u32 = 200809;
pub const _POSIX2_C_VERSION: u32 = 200809;
pub const _POSIX2_C_BIND: u32 = 200809;
pub const _POSIX2_C_DEV: u32 = 200809;
pub const _POSIX2_SW_DEV: u32 = 200809;
pub const _POSIX2_LOCALEDEF: u32 = 200809;
pub const _XOPEN_VERSION: u32 = 700;
pub const _XOPEN_XCU_VERSION: u32 = 4;
pub const _XOPEN_XPG2: u32 = 1;
pub const _XOPEN_XPG3: u32 = 1;
pub const _XOPEN_XPG4: u32 = 1;
pub const _XOPEN_UNIX: u32 = 1;
pub const _XOPEN_ENH_I18N: u32 = 1;
pub const _XOPEN_LEGACY: u32 = 1;
pub const _BITS_POSIX_OPT_H: u32 = 1;
pub const _POSIX_JOB_CONTROL: u32 = 1;
pub const _POSIX_SAVED_IDS: u32 = 1;
pub const _POSIX_PRIORITY_SCHEDULING: u32 = 200809;
pub const _POSIX_SYNCHRONIZED_IO: u32 = 200809;
pub const _POSIX_FSYNC: u32 = 200809;
pub const _POSIX_MAPPED_FILES: u32 = 200809;
pub const _POSIX_MEMLOCK: u32 = 200809;
pub const _POSIX_MEMLOCK_RANGE: u32 = 200809;
pub const _POSIX_MEMORY_PROTECTION: u32 = 200809;
pub const _POSIX_CHOWN_RESTRICTED: u32 = 0;
pub const _POSIX_VDISABLE: u8 = 0u8;
pub const _POSIX_NO_TRUNC: u32 = 1;
pub const _XOPEN_REALTIME: u32 = 1;
pub const _XOPEN_REALTIME_THREADS: u32 = 1;
pub const _XOPEN_SHM: u32 = 1;
pub const _POSIX_THREADS: u32 = 200809;
pub const _POSIX_REENTRANT_FUNCTIONS: u32 = 1;
pub const _POSIX_THREAD_SAFE_FUNCTIONS: u32 = 200809;
pub const _POSIX_THREAD_PRIORITY_SCHEDULING: u32 = 200809;
pub const _POSIX_THREAD_ATTR_STACKSIZE: u32 = 200809;
pub const _POSIX_THREAD_ATTR_STACKADDR: u32 = 200809;
pub const _POSIX_THREAD_PRIO_INHERIT: u32 = 200809;
pub const _POSIX_THREAD_PRIO_PROTECT: u32 = 200809;
pub const _POSIX_THREAD_ROBUST_PRIO_INHERIT: u32 = 200809;
pub const _POSIX_THREAD_ROBUST_PRIO_PROTECT: i32 = -1;
pub const _POSIX_SEMAPHORES: u32 = 200809;
pub const _POSIX_REALTIME_SIGNALS: u32 = 200809;
pub const _POSIX_ASYNCHRONOUS_IO: u32 = 200809;
pub const _POSIX_ASYNC_IO: u32 = 1;
pub const _LFS_ASYNCHRONOUS_IO: u32 = 1;
pub const _POSIX_PRIORITIZED_IO: u32 = 200809;
pub const _LFS64_ASYNCHRONOUS_IO: u32 = 1;
pub const _LFS_LARGEFILE: u32 = 1;
pub const _LFS64_LARGEFILE: u32 = 1;
pub const _LFS64_STDIO: u32 = 1;
pub const _POSIX_SHARED_MEMORY_OBJECTS: u32 = 200809;
pub const _POSIX_CPUTIME: u32 = 0;
pub const _POSIX_THREAD_CPUTIME: u32 = 0;
pub const _POSIX_REGEXP: u32 = 1;
pub const _POSIX_READER_WRITER_LOCKS: u32 = 200809;
pub const _POSIX_SHELL: u32 = 1;
pub const _POSIX_TIMEOUTS: u32 = 200809;
pub const _POSIX_SPIN_LOCKS: u32 = 200809;
pub const _POSIX_SPAWN: u32 = 200809;
pub const _POSIX_TIMERS: u32 = 200809;
pub const _POSIX_BARRIERS: u32 = 200809;
pub const _POSIX_MESSAGE_PASSING: u32 = 200809;
pub const _POSIX_THREAD_PROCESS_SHARED: u32 = 200809;
pub const _POSIX_MONOTONIC_CLOCK: u32 = 0;
pub const _POSIX_CLOCK_SELECTION: u32 = 200809;
pub const _POSIX_ADVISORY_INFO: u32 = 200809;
pub const _POSIX_IPV6: u32 = 200809;
pub const _POSIX_RAW_SOCKETS: u32 = 200809;
pub const _POSIX2_CHAR_TERM: u32 = 200809;
pub const _POSIX_SPORADIC_SERVER: i32 = -1;
pub const _POSIX_THREAD_SPORADIC_SERVER: i32 = -1;
pub const _POSIX_TRACE: i32 = -1;
pub const _POSIX_TRACE_EVENT_FILTER: i32 = -1;
pub const _POSIX_TRACE_INHERIT: i32 = -1;
pub const _POSIX_TRACE_LOG: i32 = -1;
pub const _POSIX_TYPED_MEMORY_OBJECTS: i32 = -1;
pub const _POSIX_V7_LPBIG_OFFBIG: i32 = -1;
pub const _POSIX_V6_LPBIG_OFFBIG: i32 = -1;
pub const _XBS5_LPBIG_OFFBIG: i32 = -1;
pub const _POSIX_V7_LP64_OFF64: u32 = 1;
pub const _POSIX_V6_LP64_OFF64: u32 = 1;
pub const _XBS5_LP64_OFF64: u32 = 1;
pub const __ILP32_OFF32_CFLAGS: &'static [u8; 5usize] = b"-m32\0";
pub const __ILP32_OFF32_LDFLAGS: &'static [u8; 5usize] = b"-m32\0";
pub const __ILP32_OFFBIG_CFLAGS: &'static [u8; 48usize] =
    b"-m32 -D_LARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64\0";
pub const __ILP32_OFFBIG_LDFLAGS: &'static [u8; 5usize] = b"-m32\0";
pub const __LP64_OFF64_CFLAGS: &'static [u8; 5usize] = b"-m64\0";
pub const __LP64_OFF64_LDFLAGS: &'static [u8; 5usize] = b"-m64\0";
pub const STDIN_FILENO: u32 = 0;
pub const STDOUT_FILENO: u32 = 1;
pub const STDERR_FILENO: u32 = 2;
pub const _BITS_TYPES_H: u32 = 1;
pub const __TIMESIZE: u32 = 64;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const R_OK: u32 = 4;
pub const W_OK: u32 = 2;
pub const X_OK: u32 = 1;
pub const F_OK: u32 = 0;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const L_SET: u32 = 0;
pub const L_INCR: u32 = 1;
pub const L_XTND: u32 = 2;
pub const _GETOPT_POSIX_H: u32 = 1;
pub const _GETOPT_CORE_H: u32 = 1;
pub const F_ULOCK: u32 = 0;
pub const F_LOCK: u32 = 1;
pub const F_TLOCK: u32 = 2;
pub const F_TEST: u32 = 3;
pub const _INTTYPES_H: u32 = 1;
pub const _STDINT_H: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const ____gwchar_t_defined: u32 = 1;
pub const __PRI64_PREFIX: &'static [u8; 2usize] = b"l\0";
pub const __PRIPTR_PREFIX: &'static [u8; 2usize] = b"l\0";
pub const PRId8: &'static [u8; 2usize] = b"d\0";
pub const PRId16: &'static [u8; 2usize] = b"d\0";
pub const PRId32: &'static [u8; 2usize] = b"d\0";
pub const PRId64: &'static [u8; 3usize] = b"ld\0";
pub const PRIdLEAST8: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST16: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST32: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST64: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST8: &'static [u8; 2usize] = b"d\0";
pub const PRIdFAST16: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST32: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST64: &'static [u8; 3usize] = b"ld\0";
pub const PRIi8: &'static [u8; 2usize] = b"i\0";
pub const PRIi16: &'static [u8; 2usize] = b"i\0";
pub const PRIi32: &'static [u8; 2usize] = b"i\0";
pub const PRIi64: &'static [u8; 3usize] = b"li\0";
pub const PRIiLEAST8: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST16: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST32: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST64: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST8: &'static [u8; 2usize] = b"i\0";
pub const PRIiFAST16: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST32: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST64: &'static [u8; 3usize] = b"li\0";
pub const PRIo8: &'static [u8; 2usize] = b"o\0";
pub const PRIo16: &'static [u8; 2usize] = b"o\0";
pub const PRIo32: &'static [u8; 2usize] = b"o\0";
pub const PRIo64: &'static [u8; 3usize] = b"lo\0";
pub const PRIoLEAST8: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST16: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST32: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST64: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST8: &'static [u8; 2usize] = b"o\0";
pub const PRIoFAST16: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST32: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST64: &'static [u8; 3usize] = b"lo\0";
pub const PRIu8: &'static [u8; 2usize] = b"u\0";
pub const PRIu16: &'static [u8; 2usize] = b"u\0";
pub const PRIu32: &'static [u8; 2usize] = b"u\0";
pub const PRIu64: &'static [u8; 3usize] = b"lu\0";
pub const PRIuLEAST8: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST16: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST32: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST64: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST8: &'static [u8; 2usize] = b"u\0";
pub const PRIuFAST16: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST32: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST64: &'static [u8; 3usize] = b"lu\0";
pub const PRIx8: &'static [u8; 2usize] = b"x\0";
pub const PRIx16: &'static [u8; 2usize] = b"x\0";
pub const PRIx32: &'static [u8; 2usize] = b"x\0";
pub const PRIx64: &'static [u8; 3usize] = b"lx\0";
pub const PRIxLEAST8: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST16: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST32: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST64: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST8: &'static [u8; 2usize] = b"x\0";
pub const PRIxFAST16: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST32: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST64: &'static [u8; 3usize] = b"lx\0";
pub const PRIX8: &'static [u8; 2usize] = b"X\0";
pub const PRIX16: &'static [u8; 2usize] = b"X\0";
pub const PRIX32: &'static [u8; 2usize] = b"X\0";
pub const PRIX64: &'static [u8; 3usize] = b"lX\0";
pub const PRIXLEAST8: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST16: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST32: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST64: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST8: &'static [u8; 2usize] = b"X\0";
pub const PRIXFAST16: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST32: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST64: &'static [u8; 3usize] = b"lX\0";
pub const PRIdMAX: &'static [u8; 3usize] = b"ld\0";
pub const PRIiMAX: &'static [u8; 3usize] = b"li\0";
pub const PRIoMAX: &'static [u8; 3usize] = b"lo\0";
pub const PRIuMAX: &'static [u8; 3usize] = b"lu\0";
pub const PRIxMAX: &'static [u8; 3usize] = b"lx\0";
pub const PRIXMAX: &'static [u8; 3usize] = b"lX\0";
pub const PRIdPTR: &'static [u8; 3usize] = b"ld\0";
pub const PRIiPTR: &'static [u8; 3usize] = b"li\0";
pub const PRIoPTR: &'static [u8; 3usize] = b"lo\0";
pub const PRIuPTR: &'static [u8; 3usize] = b"lu\0";
pub const PRIxPTR: &'static [u8; 3usize] = b"lx\0";
pub const PRIXPTR: &'static [u8; 3usize] = b"lX\0";
pub const SCNd8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNd16: &'static [u8; 3usize] = b"hd\0";
pub const SCNd32: &'static [u8; 2usize] = b"d\0";
pub const SCNd64: &'static [u8; 3usize] = b"ld\0";
pub const SCNdLEAST8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNdLEAST16: &'static [u8; 3usize] = b"hd\0";
pub const SCNdLEAST32: &'static [u8; 2usize] = b"d\0";
pub const SCNdLEAST64: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNdFAST16: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST32: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST64: &'static [u8; 3usize] = b"ld\0";
pub const SCNi8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNi16: &'static [u8; 3usize] = b"hi\0";
pub const SCNi32: &'static [u8; 2usize] = b"i\0";
pub const SCNi64: &'static [u8; 3usize] = b"li\0";
pub const SCNiLEAST8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNiLEAST16: &'static [u8; 3usize] = b"hi\0";
pub const SCNiLEAST32: &'static [u8; 2usize] = b"i\0";
pub const SCNiLEAST64: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNiFAST16: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST32: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST64: &'static [u8; 3usize] = b"li\0";
pub const SCNu8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNu16: &'static [u8; 3usize] = b"hu\0";
pub const SCNu32: &'static [u8; 2usize] = b"u\0";
pub const SCNu64: &'static [u8; 3usize] = b"lu\0";
pub const SCNuLEAST8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNuLEAST16: &'static [u8; 3usize] = b"hu\0";
pub const SCNuLEAST32: &'static [u8; 2usize] = b"u\0";
pub const SCNuLEAST64: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNuFAST16: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST32: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST64: &'static [u8; 3usize] = b"lu\0";
pub const SCNo8: &'static [u8; 4usize] = b"hho\0";
pub const SCNo16: &'static [u8; 3usize] = b"ho\0";
pub const SCNo32: &'static [u8; 2usize] = b"o\0";
pub const SCNo64: &'static [u8; 3usize] = b"lo\0";
pub const SCNoLEAST8: &'static [u8; 4usize] = b"hho\0";
pub const SCNoLEAST16: &'static [u8; 3usize] = b"ho\0";
pub const SCNoLEAST32: &'static [u8; 2usize] = b"o\0";
pub const SCNoLEAST64: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST8: &'static [u8; 4usize] = b"hho\0";
pub const SCNoFAST16: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST32: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST64: &'static [u8; 3usize] = b"lo\0";
pub const SCNx8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNx16: &'static [u8; 3usize] = b"hx\0";
pub const SCNx32: &'static [u8; 2usize] = b"x\0";
pub const SCNx64: &'static [u8; 3usize] = b"lx\0";
pub const SCNxLEAST8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNxLEAST16: &'static [u8; 3usize] = b"hx\0";
pub const SCNxLEAST32: &'static [u8; 2usize] = b"x\0";
pub const SCNxLEAST64: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNxFAST16: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST32: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST64: &'static [u8; 3usize] = b"lx\0";
pub const SCNdMAX: &'static [u8; 3usize] = b"ld\0";
pub const SCNiMAX: &'static [u8; 3usize] = b"li\0";
pub const SCNoMAX: &'static [u8; 3usize] = b"lo\0";
pub const SCNuMAX: &'static [u8; 3usize] = b"lu\0";
pub const SCNxMAX: &'static [u8; 3usize] = b"lx\0";
pub const SCNdPTR: &'static [u8; 3usize] = b"ld\0";
pub const SCNiPTR: &'static [u8; 3usize] = b"li\0";
pub const SCNoPTR: &'static [u8; 3usize] = b"lo\0";
pub const SCNuPTR: &'static [u8; 3usize] = b"lu\0";
pub const SCNxPTR: &'static [u8; 3usize] = b"lx\0";
pub const _SYS_TYPES_H: u32 = 1;
pub const __clock_t_defined: u32 = 1;
pub const __clockid_t_defined: u32 = 1;
pub const __time_t_defined: u32 = 1;
pub const __timer_t_defined: u32 = 1;
pub const __BIT_TYPES_DEFINED__: u32 = 1;
pub const _ENDIAN_H: u32 = 1;
pub const _BITS_ENDIAN_H: u32 = 1;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __PDP_ENDIAN: u32 = 3412;
pub const _BITS_ENDIANNESS_H: u32 = 1;
pub const __BYTE_ORDER: u32 = 1234;
pub const __FLOAT_WORD_ORDER: u32 = 1234;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const BYTE_ORDER: u32 = 1234;
pub const _BITS_BYTESWAP_H: u32 = 1;
pub const _BITS_UINTN_IDENTITY_H: u32 = 1;
pub const _SYS_SELECT_H: u32 = 1;
pub const __FD_ZERO_STOS: &'static [u8; 6usize] = b"stosq\0";
pub const __sigset_t_defined: u32 = 1;
pub const __timeval_defined: u32 = 1;
pub const _STRUCT_TIMESPEC: u32 = 1;
pub const FD_SETSIZE: u32 = 1024;
pub const _BITS_PTHREADTYPES_COMMON_H: u32 = 1;
pub const _THREAD_SHARED_TYPES_H: u32 = 1;
pub const _BITS_PTHREADTYPES_ARCH_H: u32 = 1;
pub const __SIZEOF_PTHREAD_MUTEX_T: u32 = 40;
pub const __SIZEOF_PTHREAD_ATTR_T: u32 = 56;
pub const __SIZEOF_PTHREAD_RWLOCK_T: u32 = 56;
pub const __SIZEOF_PTHREAD_BARRIER_T: u32 = 32;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_COND_T: u32 = 48;
pub const __SIZEOF_PTHREAD_CONDATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: u32 = 8;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: u32 = 4;
pub const _THREAD_MUTEX_INTERNAL_H: u32 = 1;
pub const __PTHREAD_MUTEX_HAVE_PREV: u32 = 1;
pub const __have_pthread_attr_t: u32 = 1;
pub const _NETDB_H: u32 = 1;
pub const _NETINET_IN_H: u32 = 1;
pub const _SYS_SOCKET_H: u32 = 1;
pub const __iovec_defined: u32 = 1;
pub const PF_UNSPEC: u32 = 0;
pub const PF_LOCAL: u32 = 1;
pub const PF_UNIX: u32 = 1;
pub const PF_FILE: u32 = 1;
pub const PF_INET: u32 = 2;
pub const PF_AX25: u32 = 3;
pub const PF_IPX: u32 = 4;
pub const PF_APPLETALK: u32 = 5;
pub const PF_NETROM: u32 = 6;
pub const PF_BRIDGE: u32 = 7;
pub const PF_ATMPVC: u32 = 8;
pub const PF_X25: u32 = 9;
pub const PF_INET6: u32 = 10;
pub const PF_ROSE: u32 = 11;
pub const PF_DECnet: u32 = 12;
pub const PF_NETBEUI: u32 = 13;
pub const PF_SECURITY: u32 = 14;
pub const PF_KEY: u32 = 15;
pub const PF_NETLINK: u32 = 16;
pub const PF_ROUTE: u32 = 16;
pub const PF_PACKET: u32 = 17;
pub const PF_ASH: u32 = 18;
pub const PF_ECONET: u32 = 19;
pub const PF_ATMSVC: u32 = 20;
pub const PF_RDS: u32 = 21;
pub const PF_SNA: u32 = 22;
pub const PF_IRDA: u32 = 23;
pub const PF_PPPOX: u32 = 24;
pub const PF_WANPIPE: u32 = 25;
pub const PF_LLC: u32 = 26;
pub const PF_IB: u32 = 27;
pub const PF_MPLS: u32 = 28;
pub const PF_CAN: u32 = 29;
pub const PF_TIPC: u32 = 30;
pub const PF_BLUETOOTH: u32 = 31;
pub const PF_IUCV: u32 = 32;
pub const PF_RXRPC: u32 = 33;
pub const PF_ISDN: u32 = 34;
pub const PF_PHONET: u32 = 35;
pub const PF_IEEE802154: u32 = 36;
pub const PF_CAIF: u32 = 37;
pub const PF_ALG: u32 = 38;
pub const PF_NFC: u32 = 39;
pub const PF_VSOCK: u32 = 40;
pub const PF_KCM: u32 = 41;
pub const PF_QIPCRTR: u32 = 42;
pub const PF_SMC: u32 = 43;
pub const PF_XDP: u32 = 44;
pub const PF_MAX: u32 = 45;
pub const AF_UNSPEC: u32 = 0;
pub const AF_LOCAL: u32 = 1;
pub const AF_UNIX: u32 = 1;
pub const AF_FILE: u32 = 1;
pub const AF_INET: u32 = 2;
pub const AF_AX25: u32 = 3;
pub const AF_IPX: u32 = 4;
pub const AF_APPLETALK: u32 = 5;
pub const AF_NETROM: u32 = 6;
pub const AF_BRIDGE: u32 = 7;
pub const AF_ATMPVC: u32 = 8;
pub const AF_X25: u32 = 9;
pub const AF_INET6: u32 = 10;
pub const AF_ROSE: u32 = 11;
pub const AF_DECnet: u32 = 12;
pub const AF_NETBEUI: u32 = 13;
pub const AF_SECURITY: u32 = 14;
pub const AF_KEY: u32 = 15;
pub const AF_NETLINK: u32 = 16;
pub const AF_ROUTE: u32 = 16;
pub const AF_PACKET: u32 = 17;
pub const AF_ASH: u32 = 18;
pub const AF_ECONET: u32 = 19;
pub const AF_ATMSVC: u32 = 20;
pub const AF_RDS: u32 = 21;
pub const AF_SNA: u32 = 22;
pub const AF_IRDA: u32 = 23;
pub const AF_PPPOX: u32 = 24;
pub const AF_WANPIPE: u32 = 25;
pub const AF_LLC: u32 = 26;
pub const AF_IB: u32 = 27;
pub const AF_MPLS: u32 = 28;
pub const AF_CAN: u32 = 29;
pub const AF_TIPC: u32 = 30;
pub const AF_BLUETOOTH: u32 = 31;
pub const AF_IUCV: u32 = 32;
pub const AF_RXRPC: u32 = 33;
pub const AF_ISDN: u32 = 34;
pub const AF_PHONET: u32 = 35;
pub const AF_IEEE802154: u32 = 36;
pub const AF_CAIF: u32 = 37;
pub const AF_ALG: u32 = 38;
pub const AF_NFC: u32 = 39;
pub const AF_VSOCK: u32 = 40;
pub const AF_KCM: u32 = 41;
pub const AF_QIPCRTR: u32 = 42;
pub const AF_SMC: u32 = 43;
pub const AF_XDP: u32 = 44;
pub const AF_MAX: u32 = 45;
pub const SOL_RAW: u32 = 255;
pub const SOL_DECNET: u32 = 261;
pub const SOL_X25: u32 = 262;
pub const SOL_PACKET: u32 = 263;
pub const SOL_ATM: u32 = 264;
pub const SOL_AAL: u32 = 265;
pub const SOL_IRDA: u32 = 266;
pub const SOL_NETBEUI: u32 = 267;
pub const SOL_LLC: u32 = 268;
pub const SOL_DCCP: u32 = 269;
pub const SOL_NETLINK: u32 = 270;
pub const SOL_TIPC: u32 = 271;
pub const SOL_RXRPC: u32 = 272;
pub const SOL_PPPOL2TP: u32 = 273;
pub const SOL_BLUETOOTH: u32 = 274;
pub const SOL_PNPIPE: u32 = 275;
pub const SOL_RDS: u32 = 276;
pub const SOL_IUCV: u32 = 277;
pub const SOL_CAIF: u32 = 278;
pub const SOL_ALG: u32 = 279;
pub const SOL_NFC: u32 = 280;
pub const SOL_KCM: u32 = 281;
pub const SOL_TLS: u32 = 282;
pub const SOL_XDP: u32 = 283;
pub const SOMAXCONN: u32 = 4096;
pub const _BITS_SOCKADDR_H: u32 = 1;
pub const _SS_SIZE: u32 = 128;
pub const __BITS_PER_LONG: u32 = 64;
pub const FIOSETOWN: u32 = 35073;
pub const SIOCSPGRP: u32 = 35074;
pub const FIOGETOWN: u32 = 35075;
pub const SIOCGPGRP: u32 = 35076;
pub const SIOCATMARK: u32 = 35077;
pub const SIOCGSTAMP: u32 = 35078;
pub const SIOCGSTAMPNS: u32 = 35079;
pub const SOL_SOCKET: u32 = 1;
pub const SO_DEBUG: u32 = 1;
pub const SO_REUSEADDR: u32 = 2;
pub const SO_TYPE: u32 = 3;
pub const SO_ERROR: u32 = 4;
pub const SO_DONTROUTE: u32 = 5;
pub const SO_BROADCAST: u32 = 6;
pub const SO_SNDBUF: u32 = 7;
pub const SO_RCVBUF: u32 = 8;
pub const SO_SNDBUFFORCE: u32 = 32;
pub const SO_RCVBUFFORCE: u32 = 33;
pub const SO_KEEPALIVE: u32 = 9;
pub const SO_OOBINLINE: u32 = 10;
pub const SO_NO_CHECK: u32 = 11;
pub const SO_PRIORITY: u32 = 12;
pub const SO_LINGER: u32 = 13;
pub const SO_BSDCOMPAT: u32 = 14;
pub const SO_REUSEPORT: u32 = 15;
pub const SO_PASSCRED: u32 = 16;
pub const SO_PEERCRED: u32 = 17;
pub const SO_RCVLOWAT: u32 = 18;
pub const SO_SNDLOWAT: u32 = 19;
pub const SO_RCVTIMEO_OLD: u32 = 20;
pub const SO_SNDTIMEO_OLD: u32 = 21;
pub const SO_SECURITY_AUTHENTICATION: u32 = 22;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: u32 = 23;
pub const SO_SECURITY_ENCRYPTION_NETWORK: u32 = 24;
pub const SO_BINDTODEVICE: u32 = 25;
pub const SO_ATTACH_FILTER: u32 = 26;
pub const SO_DETACH_FILTER: u32 = 27;
pub const SO_GET_FILTER: u32 = 26;
pub const SO_PEERNAME: u32 = 28;
pub const SO_ACCEPTCONN: u32 = 30;
pub const SO_PEERSEC: u32 = 31;
pub const SO_PASSSEC: u32 = 34;
pub const SO_MARK: u32 = 36;
pub const SO_PROTOCOL: u32 = 38;
pub const SO_DOMAIN: u32 = 39;
pub const SO_RXQ_OVFL: u32 = 40;
pub const SO_WIFI_STATUS: u32 = 41;
pub const SCM_WIFI_STATUS: u32 = 41;
pub const SO_PEEK_OFF: u32 = 42;
pub const SO_NOFCS: u32 = 43;
pub const SO_LOCK_FILTER: u32 = 44;
pub const SO_SELECT_ERR_QUEUE: u32 = 45;
pub const SO_BUSY_POLL: u32 = 46;
pub const SO_MAX_PACING_RATE: u32 = 47;
pub const SO_BPF_EXTENSIONS: u32 = 48;
pub const SO_INCOMING_CPU: u32 = 49;
pub const SO_ATTACH_BPF: u32 = 50;
pub const SO_DETACH_BPF: u32 = 27;
pub const SO_ATTACH_REUSEPORT_CBPF: u32 = 51;
pub const SO_ATTACH_REUSEPORT_EBPF: u32 = 52;
pub const SO_CNX_ADVICE: u32 = 53;
pub const SCM_TIMESTAMPING_OPT_STATS: u32 = 54;
pub const SO_MEMINFO: u32 = 55;
pub const SO_INCOMING_NAPI_ID: u32 = 56;
pub const SO_COOKIE: u32 = 57;
pub const SCM_TIMESTAMPING_PKTINFO: u32 = 58;
pub const SO_PEERGROUPS: u32 = 59;
pub const SO_ZEROCOPY: u32 = 60;
pub const SO_TXTIME: u32 = 61;
pub const SCM_TXTIME: u32 = 61;
pub const SO_BINDTOIFINDEX: u32 = 62;
pub const SO_TIMESTAMP_OLD: u32 = 29;
pub const SO_TIMESTAMPNS_OLD: u32 = 35;
pub const SO_TIMESTAMPING_OLD: u32 = 37;
pub const SO_TIMESTAMP_NEW: u32 = 63;
pub const SO_TIMESTAMPNS_NEW: u32 = 64;
pub const SO_TIMESTAMPING_NEW: u32 = 65;
pub const SO_RCVTIMEO_NEW: u32 = 66;
pub const SO_SNDTIMEO_NEW: u32 = 67;
pub const SO_DETACH_REUSEPORT_BPF: u32 = 68;
pub const SO_TIMESTAMP: u32 = 29;
pub const SO_TIMESTAMPNS: u32 = 35;
pub const SO_TIMESTAMPING: u32 = 37;
pub const SO_RCVTIMEO: u32 = 20;
pub const SO_SNDTIMEO: u32 = 21;
pub const SCM_TIMESTAMP: u32 = 29;
pub const SCM_TIMESTAMPNS: u32 = 35;
pub const SCM_TIMESTAMPING: u32 = 37;
pub const __osockaddr_defined: u32 = 1;
pub const __USE_KERNEL_IPV6_DEFS: u32 = 0;
pub const IP_OPTIONS: u32 = 4;
pub const IP_HDRINCL: u32 = 3;
pub const IP_TOS: u32 = 1;
pub const IP_TTL: u32 = 2;
pub const IP_RECVOPTS: u32 = 6;
pub const IP_RETOPTS: u32 = 7;
pub const IP_MULTICAST_IF: u32 = 32;
pub const IP_MULTICAST_TTL: u32 = 33;
pub const IP_MULTICAST_LOOP: u32 = 34;
pub const IP_ADD_MEMBERSHIP: u32 = 35;
pub const IP_DROP_MEMBERSHIP: u32 = 36;
pub const IP_UNBLOCK_SOURCE: u32 = 37;
pub const IP_BLOCK_SOURCE: u32 = 38;
pub const IP_ADD_SOURCE_MEMBERSHIP: u32 = 39;
pub const IP_DROP_SOURCE_MEMBERSHIP: u32 = 40;
pub const IP_MSFILTER: u32 = 41;
pub const MCAST_JOIN_GROUP: u32 = 42;
pub const MCAST_BLOCK_SOURCE: u32 = 43;
pub const MCAST_UNBLOCK_SOURCE: u32 = 44;
pub const MCAST_LEAVE_GROUP: u32 = 45;
pub const MCAST_JOIN_SOURCE_GROUP: u32 = 46;
pub const MCAST_LEAVE_SOURCE_GROUP: u32 = 47;
pub const MCAST_MSFILTER: u32 = 48;
pub const IP_MULTICAST_ALL: u32 = 49;
pub const IP_UNICAST_IF: u32 = 50;
pub const MCAST_EXCLUDE: u32 = 0;
pub const MCAST_INCLUDE: u32 = 1;
pub const IP_ROUTER_ALERT: u32 = 5;
pub const IP_PKTINFO: u32 = 8;
pub const IP_PKTOPTIONS: u32 = 9;
pub const IP_PMTUDISC: u32 = 10;
pub const IP_MTU_DISCOVER: u32 = 10;
pub const IP_RECVERR: u32 = 11;
pub const IP_RECVTTL: u32 = 12;
pub const IP_RECVTOS: u32 = 13;
pub const IP_MTU: u32 = 14;
pub const IP_FREEBIND: u32 = 15;
pub const IP_IPSEC_POLICY: u32 = 16;
pub const IP_XFRM_POLICY: u32 = 17;
pub const IP_PASSSEC: u32 = 18;
pub const IP_TRANSPARENT: u32 = 19;
pub const IP_ORIGDSTADDR: u32 = 20;
pub const IP_RECVORIGDSTADDR: u32 = 20;
pub const IP_MINTTL: u32 = 21;
pub const IP_NODEFRAG: u32 = 22;
pub const IP_CHECKSUM: u32 = 23;
pub const IP_BIND_ADDRESS_NO_PORT: u32 = 24;
pub const IP_RECVFRAGSIZE: u32 = 25;
pub const IP_PMTUDISC_DONT: u32 = 0;
pub const IP_PMTUDISC_WANT: u32 = 1;
pub const IP_PMTUDISC_DO: u32 = 2;
pub const IP_PMTUDISC_PROBE: u32 = 3;
pub const IP_PMTUDISC_INTERFACE: u32 = 4;
pub const IP_PMTUDISC_OMIT: u32 = 5;
pub const SOL_IP: u32 = 0;
pub const IP_DEFAULT_MULTICAST_TTL: u32 = 1;
pub const IP_DEFAULT_MULTICAST_LOOP: u32 = 1;
pub const IP_MAX_MEMBERSHIPS: u32 = 20;
pub const IPV6_ADDRFORM: u32 = 1;
pub const IPV6_2292PKTINFO: u32 = 2;
pub const IPV6_2292HOPOPTS: u32 = 3;
pub const IPV6_2292DSTOPTS: u32 = 4;
pub const IPV6_2292RTHDR: u32 = 5;
pub const IPV6_2292PKTOPTIONS: u32 = 6;
pub const IPV6_CHECKSUM: u32 = 7;
pub const IPV6_2292HOPLIMIT: u32 = 8;
pub const IPV6_NEXTHOP: u32 = 9;
pub const IPV6_AUTHHDR: u32 = 10;
pub const IPV6_UNICAST_HOPS: u32 = 16;
pub const IPV6_MULTICAST_IF: u32 = 17;
pub const IPV6_MULTICAST_HOPS: u32 = 18;
pub const IPV6_MULTICAST_LOOP: u32 = 19;
pub const IPV6_JOIN_GROUP: u32 = 20;
pub const IPV6_LEAVE_GROUP: u32 = 21;
pub const IPV6_ROUTER_ALERT: u32 = 22;
pub const IPV6_MTU_DISCOVER: u32 = 23;
pub const IPV6_MTU: u32 = 24;
pub const IPV6_RECVERR: u32 = 25;
pub const IPV6_V6ONLY: u32 = 26;
pub const IPV6_JOIN_ANYCAST: u32 = 27;
pub const IPV6_LEAVE_ANYCAST: u32 = 28;
pub const IPV6_MULTICAST_ALL: u32 = 29;
pub const IPV6_ROUTER_ALERT_ISOLATE: u32 = 30;
pub const IPV6_IPSEC_POLICY: u32 = 34;
pub const IPV6_XFRM_POLICY: u32 = 35;
pub const IPV6_HDRINCL: u32 = 36;
pub const IPV6_RECVPKTINFO: u32 = 49;
pub const IPV6_PKTINFO: u32 = 50;
pub const IPV6_RECVHOPLIMIT: u32 = 51;
pub const IPV6_HOPLIMIT: u32 = 52;
pub const IPV6_RECVHOPOPTS: u32 = 53;
pub const IPV6_HOPOPTS: u32 = 54;
pub const IPV6_RTHDRDSTOPTS: u32 = 55;
pub const IPV6_RECVRTHDR: u32 = 56;
pub const IPV6_RTHDR: u32 = 57;
pub const IPV6_RECVDSTOPTS: u32 = 58;
pub const IPV6_DSTOPTS: u32 = 59;
pub const IPV6_RECVPATHMTU: u32 = 60;
pub const IPV6_PATHMTU: u32 = 61;
pub const IPV6_DONTFRAG: u32 = 62;
pub const IPV6_RECVTCLASS: u32 = 66;
pub const IPV6_TCLASS: u32 = 67;
pub const IPV6_AUTOFLOWLABEL: u32 = 70;
pub const IPV6_ADDR_PREFERENCES: u32 = 72;
pub const IPV6_MINHOPCOUNT: u32 = 73;
pub const IPV6_ORIGDSTADDR: u32 = 74;
pub const IPV6_RECVORIGDSTADDR: u32 = 74;
pub const IPV6_TRANSPARENT: u32 = 75;
pub const IPV6_UNICAST_IF: u32 = 76;
pub const IPV6_RECVFRAGSIZE: u32 = 77;
pub const IPV6_FREEBIND: u32 = 78;
pub const IPV6_ADD_MEMBERSHIP: u32 = 20;
pub const IPV6_DROP_MEMBERSHIP: u32 = 21;
pub const IPV6_RXHOPOPTS: u32 = 54;
pub const IPV6_RXDSTOPTS: u32 = 59;
pub const IPV6_PMTUDISC_DONT: u32 = 0;
pub const IPV6_PMTUDISC_WANT: u32 = 1;
pub const IPV6_PMTUDISC_DO: u32 = 2;
pub const IPV6_PMTUDISC_PROBE: u32 = 3;
pub const IPV6_PMTUDISC_INTERFACE: u32 = 4;
pub const IPV6_PMTUDISC_OMIT: u32 = 5;
pub const SOL_IPV6: u32 = 41;
pub const SOL_ICMPV6: u32 = 58;
pub const IPV6_RTHDR_LOOSE: u32 = 0;
pub const IPV6_RTHDR_STRICT: u32 = 1;
pub const IPV6_RTHDR_TYPE_0: u32 = 0;
pub const IN_CLASSA_NET: u32 = 4278190080;
pub const IN_CLASSA_NSHIFT: u32 = 24;
pub const IN_CLASSA_HOST: u32 = 16777215;
pub const IN_CLASSA_MAX: u32 = 128;
pub const IN_CLASSB_NET: u32 = 4294901760;
pub const IN_CLASSB_NSHIFT: u32 = 16;
pub const IN_CLASSB_HOST: u32 = 65535;
pub const IN_CLASSB_MAX: u32 = 65536;
pub const IN_CLASSC_NET: u32 = 4294967040;
pub const IN_CLASSC_NSHIFT: u32 = 8;
pub const IN_CLASSC_HOST: u32 = 255;
pub const IN_LOOPBACKNET: u32 = 127;
pub const INET_ADDRSTRLEN: u32 = 16;
pub const INET6_ADDRSTRLEN: u32 = 46;
pub const _RPC_NETDB_H: u32 = 1;
pub const _PATH_HEQUIV: &'static [u8; 17usize] = b"/etc/hosts.equiv\0";
pub const _PATH_HOSTS: &'static [u8; 11usize] = b"/etc/hosts\0";
pub const _PATH_NETWORKS: &'static [u8; 14usize] = b"/etc/networks\0";
pub const _PATH_NSSWITCH_CONF: &'static [u8; 19usize] = b"/etc/nsswitch.conf\0";
pub const _PATH_PROTOCOLS: &'static [u8; 15usize] = b"/etc/protocols\0";
pub const _PATH_SERVICES: &'static [u8; 14usize] = b"/etc/services\0";
pub const HOST_NOT_FOUND: u32 = 1;
pub const TRY_AGAIN: u32 = 2;
pub const NO_RECOVERY: u32 = 3;
pub const NO_DATA: u32 = 4;
pub const NETDB_INTERNAL: i32 = -1;
pub const NETDB_SUCCESS: u32 = 0;
pub const NO_ADDRESS: u32 = 4;
pub const IPPORT_RESERVED: u32 = 1024;
pub const AI_PASSIVE: u32 = 1;
pub const AI_CANONNAME: u32 = 2;
pub const AI_NUMERICHOST: u32 = 4;
pub const AI_V4MAPPED: u32 = 8;
pub const AI_ALL: u32 = 16;
pub const AI_ADDRCONFIG: u32 = 32;
pub const AI_NUMERICSERV: u32 = 1024;
pub const EAI_BADFLAGS: i32 = -1;
pub const EAI_NONAME: i32 = -2;
pub const EAI_AGAIN: i32 = -3;
pub const EAI_FAIL: i32 = -4;
pub const EAI_FAMILY: i32 = -6;
pub const EAI_SOCKTYPE: i32 = -7;
pub const EAI_SERVICE: i32 = -8;
pub const EAI_MEMORY: i32 = -10;
pub const EAI_SYSTEM: i32 = -11;
pub const EAI_OVERFLOW: i32 = -12;
pub const NI_MAXHOST: u32 = 1025;
pub const NI_MAXSERV: u32 = 32;
pub const NI_NUMERICHOST: u32 = 1;
pub const NI_NUMERICSERV: u32 = 2;
pub const NI_NOFQDN: u32 = 4;
pub const NI_NAMEREQD: u32 = 8;
pub const NI_DGRAM: u32 = 16;
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__fsid_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type ssize_t = __ssize_t;
pub type size_t = ::std::os::raw::c_ulonglong;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type useconds_t = __useconds_t;
pub type pid_t = __pid_t;
pub type socklen_t = __socklen_t;
extern "C" {
    pub fn access(
        __name: *const ::std::os::raw::c_char,
        __type: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faccessat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __type: ::std::os::raw::c_int,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lseek(
        __fd: ::std::os::raw::c_int,
        __offset: __off_t,
        __whence: ::std::os::raw::c_int,
    ) -> __off_t;
}
extern "C" {
    pub fn close(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn read(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
}
extern "C" {
    pub fn write(
        __fd: ::std::os::raw::c_int,
        __buf: *const ::std::os::raw::c_void,
        __n: size_t,
    ) -> ssize_t;
}
extern "C" {
    pub fn pread(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_void,
        __nbytes: size_t,
        __offset: __off_t,
    ) -> ssize_t;
}
extern "C" {
    pub fn pwrite(
        __fd: ::std::os::raw::c_int,
        __buf: *const ::std::os::raw::c_void,
        __n: size_t,
        __offset: __off_t,
    ) -> ssize_t;
}
extern "C" {
    pub fn pipe(__pipedes: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn alarm(__seconds: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sleep(__seconds: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn ualarm(__value: __useconds_t, __interval: __useconds_t) -> __useconds_t;
}
extern "C" {
    pub fn usleep(__useconds: __useconds_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pause() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chown(
        __file: *const ::std::os::raw::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchown(
        __fd: ::std::os::raw::c_int,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lchown(
        __file: *const ::std::os::raw::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchownat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __owner: __uid_t,
        __group: __gid_t,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chdir(__path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchdir(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getcwd(
        __buf: *mut ::std::os::raw::c_char,
        __size: size_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getwd(__buf: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn dup(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dup2(__fd: ::std::os::raw::c_int, __fd2: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub static mut __environ: *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn execve(
        __path: *const ::std::os::raw::c_char,
        __argv: *const *mut ::std::os::raw::c_char,
        __envp: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fexecve(
        __fd: ::std::os::raw::c_int,
        __argv: *const *mut ::std::os::raw::c_char,
        __envp: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execv(
        __path: *const ::std::os::raw::c_char,
        __argv: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execle(
        __path: *const ::std::os::raw::c_char,
        __arg: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execl(
        __path: *const ::std::os::raw::c_char,
        __arg: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execvp(
        __file: *const ::std::os::raw::c_char,
        __argv: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn execlp(
        __file: *const ::std::os::raw::c_char,
        __arg: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nice(__inc: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _exit(__status: ::std::os::raw::c_int);
}
pub const _PC_LINK_MAX: ::std::os::raw::c_uint = 0;
pub const _PC_MAX_CANON: ::std::os::raw::c_uint = 1;
pub const _PC_MAX_INPUT: ::std::os::raw::c_uint = 2;
pub const _PC_NAME_MAX: ::std::os::raw::c_uint = 3;
pub const _PC_PATH_MAX: ::std::os::raw::c_uint = 4;
pub const _PC_PIPE_BUF: ::std::os::raw::c_uint = 5;
pub const _PC_CHOWN_RESTRICTED: ::std::os::raw::c_uint = 6;
pub const _PC_NO_TRUNC: ::std::os::raw::c_uint = 7;
pub const _PC_VDISABLE: ::std::os::raw::c_uint = 8;
pub const _PC_SYNC_IO: ::std::os::raw::c_uint = 9;
pub const _PC_ASYNC_IO: ::std::os::raw::c_uint = 10;
pub const _PC_PRIO_IO: ::std::os::raw::c_uint = 11;
pub const _PC_SOCK_MAXBUF: ::std::os::raw::c_uint = 12;
pub const _PC_FILESIZEBITS: ::std::os::raw::c_uint = 13;
pub const _PC_REC_INCR_XFER_SIZE: ::std::os::raw::c_uint = 14;
pub const _PC_REC_MAX_XFER_SIZE: ::std::os::raw::c_uint = 15;
pub const _PC_REC_MIN_XFER_SIZE: ::std::os::raw::c_uint = 16;
pub const _PC_REC_XFER_ALIGN: ::std::os::raw::c_uint = 17;
pub const _PC_ALLOC_SIZE_MIN: ::std::os::raw::c_uint = 18;
pub const _PC_SYMLINK_MAX: ::std::os::raw::c_uint = 19;
pub const _PC_2_SYMLINKS: ::std::os::raw::c_uint = 20;
pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
pub const _SC_ARG_MAX: ::std::os::raw::c_uint = 0;
pub const _SC_CHILD_MAX: ::std::os::raw::c_uint = 1;
pub const _SC_CLK_TCK: ::std::os::raw::c_uint = 2;
pub const _SC_NGROUPS_MAX: ::std::os::raw::c_uint = 3;
pub const _SC_OPEN_MAX: ::std::os::raw::c_uint = 4;
pub const _SC_STREAM_MAX: ::std::os::raw::c_uint = 5;
pub const _SC_TZNAME_MAX: ::std::os::raw::c_uint = 6;
pub const _SC_JOB_CONTROL: ::std::os::raw::c_uint = 7;
pub const _SC_SAVED_IDS: ::std::os::raw::c_uint = 8;
pub const _SC_REALTIME_SIGNALS: ::std::os::raw::c_uint = 9;
pub const _SC_PRIORITY_SCHEDULING: ::std::os::raw::c_uint = 10;
pub const _SC_TIMERS: ::std::os::raw::c_uint = 11;
pub const _SC_ASYNCHRONOUS_IO: ::std::os::raw::c_uint = 12;
pub const _SC_PRIORITIZED_IO: ::std::os::raw::c_uint = 13;
pub const _SC_SYNCHRONIZED_IO: ::std::os::raw::c_uint = 14;
pub const _SC_FSYNC: ::std::os::raw::c_uint = 15;
pub const _SC_MAPPED_FILES: ::std::os::raw::c_uint = 16;
pub const _SC_MEMLOCK: ::std::os::raw::c_uint = 17;
pub const _SC_MEMLOCK_RANGE: ::std::os::raw::c_uint = 18;
pub const _SC_MEMORY_PROTECTION: ::std::os::raw::c_uint = 19;
pub const _SC_MESSAGE_PASSING: ::std::os::raw::c_uint = 20;
pub const _SC_SEMAPHORES: ::std::os::raw::c_uint = 21;
pub const _SC_SHARED_MEMORY_OBJECTS: ::std::os::raw::c_uint = 22;
pub const _SC_AIO_LISTIO_MAX: ::std::os::raw::c_uint = 23;
pub const _SC_AIO_MAX: ::std::os::raw::c_uint = 24;
pub const _SC_AIO_PRIO_DELTA_MAX: ::std::os::raw::c_uint = 25;
pub const _SC_DELAYTIMER_MAX: ::std::os::raw::c_uint = 26;
pub const _SC_MQ_OPEN_MAX: ::std::os::raw::c_uint = 27;
pub const _SC_MQ_PRIO_MAX: ::std::os::raw::c_uint = 28;
pub const _SC_VERSION: ::std::os::raw::c_uint = 29;
pub const _SC_PAGESIZE: ::std::os::raw::c_uint = 30;
pub const _SC_RTSIG_MAX: ::std::os::raw::c_uint = 31;
pub const _SC_SEM_NSEMS_MAX: ::std::os::raw::c_uint = 32;
pub const _SC_SEM_VALUE_MAX: ::std::os::raw::c_uint = 33;
pub const _SC_SIGQUEUE_MAX: ::std::os::raw::c_uint = 34;
pub const _SC_TIMER_MAX: ::std::os::raw::c_uint = 35;
pub const _SC_BC_BASE_MAX: ::std::os::raw::c_uint = 36;
pub const _SC_BC_DIM_MAX: ::std::os::raw::c_uint = 37;
pub const _SC_BC_SCALE_MAX: ::std::os::raw::c_uint = 38;
pub const _SC_BC_STRING_MAX: ::std::os::raw::c_uint = 39;
pub const _SC_COLL_WEIGHTS_MAX: ::std::os::raw::c_uint = 40;
pub const _SC_EQUIV_CLASS_MAX: ::std::os::raw::c_uint = 41;
pub const _SC_EXPR_NEST_MAX: ::std::os::raw::c_uint = 42;
pub const _SC_LINE_MAX: ::std::os::raw::c_uint = 43;
pub const _SC_RE_DUP_MAX: ::std::os::raw::c_uint = 44;
pub const _SC_CHARCLASS_NAME_MAX: ::std::os::raw::c_uint = 45;
pub const _SC_2_VERSION: ::std::os::raw::c_uint = 46;
pub const _SC_2_C_BIND: ::std::os::raw::c_uint = 47;
pub const _SC_2_C_DEV: ::std::os::raw::c_uint = 48;
pub const _SC_2_FORT_DEV: ::std::os::raw::c_uint = 49;
pub const _SC_2_FORT_RUN: ::std::os::raw::c_uint = 50;
pub const _SC_2_SW_DEV: ::std::os::raw::c_uint = 51;
pub const _SC_2_LOCALEDEF: ::std::os::raw::c_uint = 52;
pub const _SC_PII: ::std::os::raw::c_uint = 53;
pub const _SC_PII_XTI: ::std::os::raw::c_uint = 54;
pub const _SC_PII_SOCKET: ::std::os::raw::c_uint = 55;
pub const _SC_PII_INTERNET: ::std::os::raw::c_uint = 56;
pub const _SC_PII_OSI: ::std::os::raw::c_uint = 57;
pub const _SC_POLL: ::std::os::raw::c_uint = 58;
pub const _SC_SELECT: ::std::os::raw::c_uint = 59;
pub const _SC_UIO_MAXIOV: ::std::os::raw::c_uint = 60;
pub const _SC_IOV_MAX: ::std::os::raw::c_uint = 60;
pub const _SC_PII_INTERNET_STREAM: ::std::os::raw::c_uint = 61;
pub const _SC_PII_INTERNET_DGRAM: ::std::os::raw::c_uint = 62;
pub const _SC_PII_OSI_COTS: ::std::os::raw::c_uint = 63;
pub const _SC_PII_OSI_CLTS: ::std::os::raw::c_uint = 64;
pub const _SC_PII_OSI_M: ::std::os::raw::c_uint = 65;
pub const _SC_T_IOV_MAX: ::std::os::raw::c_uint = 66;
pub const _SC_THREADS: ::std::os::raw::c_uint = 67;
pub const _SC_THREAD_SAFE_FUNCTIONS: ::std::os::raw::c_uint = 68;
pub const _SC_GETGR_R_SIZE_MAX: ::std::os::raw::c_uint = 69;
pub const _SC_GETPW_R_SIZE_MAX: ::std::os::raw::c_uint = 70;
pub const _SC_LOGIN_NAME_MAX: ::std::os::raw::c_uint = 71;
pub const _SC_TTY_NAME_MAX: ::std::os::raw::c_uint = 72;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: ::std::os::raw::c_uint = 73;
pub const _SC_THREAD_KEYS_MAX: ::std::os::raw::c_uint = 74;
pub const _SC_THREAD_STACK_MIN: ::std::os::raw::c_uint = 75;
pub const _SC_THREAD_THREADS_MAX: ::std::os::raw::c_uint = 76;
pub const _SC_THREAD_ATTR_STACKADDR: ::std::os::raw::c_uint = 77;
pub const _SC_THREAD_ATTR_STACKSIZE: ::std::os::raw::c_uint = 78;
pub const _SC_THREAD_PRIORITY_SCHEDULING: ::std::os::raw::c_uint = 79;
pub const _SC_THREAD_PRIO_INHERIT: ::std::os::raw::c_uint = 80;
pub const _SC_THREAD_PRIO_PROTECT: ::std::os::raw::c_uint = 81;
pub const _SC_THREAD_PROCESS_SHARED: ::std::os::raw::c_uint = 82;
pub const _SC_NPROCESSORS_CONF: ::std::os::raw::c_uint = 83;
pub const _SC_NPROCESSORS_ONLN: ::std::os::raw::c_uint = 84;
pub const _SC_PHYS_PAGES: ::std::os::raw::c_uint = 85;
pub const _SC_AVPHYS_PAGES: ::std::os::raw::c_uint = 86;
pub const _SC_ATEXIT_MAX: ::std::os::raw::c_uint = 87;
pub const _SC_PASS_MAX: ::std::os::raw::c_uint = 88;
pub const _SC_XOPEN_VERSION: ::std::os::raw::c_uint = 89;
pub const _SC_XOPEN_XCU_VERSION: ::std::os::raw::c_uint = 90;
pub const _SC_XOPEN_UNIX: ::std::os::raw::c_uint = 91;
pub const _SC_XOPEN_CRYPT: ::std::os::raw::c_uint = 92;
pub const _SC_XOPEN_ENH_I18N: ::std::os::raw::c_uint = 93;
pub const _SC_XOPEN_SHM: ::std::os::raw::c_uint = 94;
pub const _SC_2_CHAR_TERM: ::std::os::raw::c_uint = 95;
pub const _SC_2_C_VERSION: ::std::os::raw::c_uint = 96;
pub const _SC_2_UPE: ::std::os::raw::c_uint = 97;
pub const _SC_XOPEN_XPG2: ::std::os::raw::c_uint = 98;
pub const _SC_XOPEN_XPG3: ::std::os::raw::c_uint = 99;
pub const _SC_XOPEN_XPG4: ::std::os::raw::c_uint = 100;
pub const _SC_CHAR_BIT: ::std::os::raw::c_uint = 101;
pub const _SC_CHAR_MAX: ::std::os::raw::c_uint = 102;
pub const _SC_CHAR_MIN: ::std::os::raw::c_uint = 103;
pub const _SC_INT_MAX: ::std::os::raw::c_uint = 104;
pub const _SC_INT_MIN: ::std::os::raw::c_uint = 105;
pub const _SC_LONG_BIT: ::std::os::raw::c_uint = 106;
pub const _SC_WORD_BIT: ::std::os::raw::c_uint = 107;
pub const _SC_MB_LEN_MAX: ::std::os::raw::c_uint = 108;
pub const _SC_NZERO: ::std::os::raw::c_uint = 109;
pub const _SC_SSIZE_MAX: ::std::os::raw::c_uint = 110;
pub const _SC_SCHAR_MAX: ::std::os::raw::c_uint = 111;
pub const _SC_SCHAR_MIN: ::std::os::raw::c_uint = 112;
pub const _SC_SHRT_MAX: ::std::os::raw::c_uint = 113;
pub const _SC_SHRT_MIN: ::std::os::raw::c_uint = 114;
pub const _SC_UCHAR_MAX: ::std::os::raw::c_uint = 115;
pub const _SC_UINT_MAX: ::std::os::raw::c_uint = 116;
pub const _SC_ULONG_MAX: ::std::os::raw::c_uint = 117;
pub const _SC_USHRT_MAX: ::std::os::raw::c_uint = 118;
pub const _SC_NL_ARGMAX: ::std::os::raw::c_uint = 119;
pub const _SC_NL_LANGMAX: ::std::os::raw::c_uint = 120;
pub const _SC_NL_MSGMAX: ::std::os::raw::c_uint = 121;
pub const _SC_NL_NMAX: ::std::os::raw::c_uint = 122;
pub const _SC_NL_SETMAX: ::std::os::raw::c_uint = 123;
pub const _SC_NL_TEXTMAX: ::std::os::raw::c_uint = 124;
pub const _SC_XBS5_ILP32_OFF32: ::std::os::raw::c_uint = 125;
pub const _SC_XBS5_ILP32_OFFBIG: ::std::os::raw::c_uint = 126;
pub const _SC_XBS5_LP64_OFF64: ::std::os::raw::c_uint = 127;
pub const _SC_XBS5_LPBIG_OFFBIG: ::std::os::raw::c_uint = 128;
pub const _SC_XOPEN_LEGACY: ::std::os::raw::c_uint = 129;
pub const _SC_XOPEN_REALTIME: ::std::os::raw::c_uint = 130;
pub const _SC_XOPEN_REALTIME_THREADS: ::std::os::raw::c_uint = 131;
pub const _SC_ADVISORY_INFO: ::std::os::raw::c_uint = 132;
pub const _SC_BARRIERS: ::std::os::raw::c_uint = 133;
pub const _SC_BASE: ::std::os::raw::c_uint = 134;
pub const _SC_C_LANG_SUPPORT: ::std::os::raw::c_uint = 135;
pub const _SC_C_LANG_SUPPORT_R: ::std::os::raw::c_uint = 136;
pub const _SC_CLOCK_SELECTION: ::std::os::raw::c_uint = 137;
pub const _SC_CPUTIME: ::std::os::raw::c_uint = 138;
pub const _SC_THREAD_CPUTIME: ::std::os::raw::c_uint = 139;
pub const _SC_DEVICE_IO: ::std::os::raw::c_uint = 140;
pub const _SC_DEVICE_SPECIFIC: ::std::os::raw::c_uint = 141;
pub const _SC_DEVICE_SPECIFIC_R: ::std::os::raw::c_uint = 142;
pub const _SC_FD_MGMT: ::std::os::raw::c_uint = 143;
pub const _SC_FIFO: ::std::os::raw::c_uint = 144;
pub const _SC_PIPE: ::std::os::raw::c_uint = 145;
pub const _SC_FILE_ATTRIBUTES: ::std::os::raw::c_uint = 146;
pub const _SC_FILE_LOCKING: ::std::os::raw::c_uint = 147;
pub const _SC_FILE_SYSTEM: ::std::os::raw::c_uint = 148;
pub const _SC_MONOTONIC_CLOCK: ::std::os::raw::c_uint = 149;
pub const _SC_MULTI_PROCESS: ::std::os::raw::c_uint = 150;
pub const _SC_SINGLE_PROCESS: ::std::os::raw::c_uint = 151;
pub const _SC_NETWORKING: ::std::os::raw::c_uint = 152;
pub const _SC_READER_WRITER_LOCKS: ::std::os::raw::c_uint = 153;
pub const _SC_SPIN_LOCKS: ::std::os::raw::c_uint = 154;
pub const _SC_REGEXP: ::std::os::raw::c_uint = 155;
pub const _SC_REGEX_VERSION: ::std::os::raw::c_uint = 156;
pub const _SC_SHELL: ::std::os::raw::c_uint = 157;
pub const _SC_SIGNALS: ::std::os::raw::c_uint = 158;
pub const _SC_SPAWN: ::std::os::raw::c_uint = 159;
pub const _SC_SPORADIC_SERVER: ::std::os::raw::c_uint = 160;
pub const _SC_THREAD_SPORADIC_SERVER: ::std::os::raw::c_uint = 161;
pub const _SC_SYSTEM_DATABASE: ::std::os::raw::c_uint = 162;
pub const _SC_SYSTEM_DATABASE_R: ::std::os::raw::c_uint = 163;
pub const _SC_TIMEOUTS: ::std::os::raw::c_uint = 164;
pub const _SC_TYPED_MEMORY_OBJECTS: ::std::os::raw::c_uint = 165;
pub const _SC_USER_GROUPS: ::std::os::raw::c_uint = 166;
pub const _SC_USER_GROUPS_R: ::std::os::raw::c_uint = 167;
pub const _SC_2_PBS: ::std::os::raw::c_uint = 168;
pub const _SC_2_PBS_ACCOUNTING: ::std::os::raw::c_uint = 169;
pub const _SC_2_PBS_LOCATE: ::std::os::raw::c_uint = 170;
pub const _SC_2_PBS_MESSAGE: ::std::os::raw::c_uint = 171;
pub const _SC_2_PBS_TRACK: ::std::os::raw::c_uint = 172;
pub const _SC_SYMLOOP_MAX: ::std::os::raw::c_uint = 173;
pub const _SC_STREAMS: ::std::os::raw::c_uint = 174;
pub const _SC_2_PBS_CHECKPOINT: ::std::os::raw::c_uint = 175;
pub const _SC_V6_ILP32_OFF32: ::std::os::raw::c_uint = 176;
pub const _SC_V6_ILP32_OFFBIG: ::std::os::raw::c_uint = 177;
pub const _SC_V6_LP64_OFF64: ::std::os::raw::c_uint = 178;
pub const _SC_V6_LPBIG_OFFBIG: ::std::os::raw::c_uint = 179;
pub const _SC_HOST_NAME_MAX: ::std::os::raw::c_uint = 180;
pub const _SC_TRACE: ::std::os::raw::c_uint = 181;
pub const _SC_TRACE_EVENT_FILTER: ::std::os::raw::c_uint = 182;
pub const _SC_TRACE_INHERIT: ::std::os::raw::c_uint = 183;
pub const _SC_TRACE_LOG: ::std::os::raw::c_uint = 184;
pub const _SC_LEVEL1_ICACHE_SIZE: ::std::os::raw::c_uint = 185;
pub const _SC_LEVEL1_ICACHE_ASSOC: ::std::os::raw::c_uint = 186;
pub const _SC_LEVEL1_ICACHE_LINESIZE: ::std::os::raw::c_uint = 187;
pub const _SC_LEVEL1_DCACHE_SIZE: ::std::os::raw::c_uint = 188;
pub const _SC_LEVEL1_DCACHE_ASSOC: ::std::os::raw::c_uint = 189;
pub const _SC_LEVEL1_DCACHE_LINESIZE: ::std::os::raw::c_uint = 190;
pub const _SC_LEVEL2_CACHE_SIZE: ::std::os::raw::c_uint = 191;
pub const _SC_LEVEL2_CACHE_ASSOC: ::std::os::raw::c_uint = 192;
pub const _SC_LEVEL2_CACHE_LINESIZE: ::std::os::raw::c_uint = 193;
pub const _SC_LEVEL3_CACHE_SIZE: ::std::os::raw::c_uint = 194;
pub const _SC_LEVEL3_CACHE_ASSOC: ::std::os::raw::c_uint = 195;
pub const _SC_LEVEL3_CACHE_LINESIZE: ::std::os::raw::c_uint = 196;
pub const _SC_LEVEL4_CACHE_SIZE: ::std::os::raw::c_uint = 197;
pub const _SC_LEVEL4_CACHE_ASSOC: ::std::os::raw::c_uint = 198;
pub const _SC_LEVEL4_CACHE_LINESIZE: ::std::os::raw::c_uint = 199;
pub const _SC_IPV6: ::std::os::raw::c_uint = 235;
pub const _SC_RAW_SOCKETS: ::std::os::raw::c_uint = 236;
pub const _SC_V7_ILP32_OFF32: ::std::os::raw::c_uint = 237;
pub const _SC_V7_ILP32_OFFBIG: ::std::os::raw::c_uint = 238;
pub const _SC_V7_LP64_OFF64: ::std::os::raw::c_uint = 239;
pub const _SC_V7_LPBIG_OFFBIG: ::std::os::raw::c_uint = 240;
pub const _SC_SS_REPL_MAX: ::std::os::raw::c_uint = 241;
pub const _SC_TRACE_EVENT_NAME_MAX: ::std::os::raw::c_uint = 242;
pub const _SC_TRACE_NAME_MAX: ::std::os::raw::c_uint = 243;
pub const _SC_TRACE_SYS_MAX: ::std::os::raw::c_uint = 244;
pub const _SC_TRACE_USER_EVENT_MAX: ::std::os::raw::c_uint = 245;
pub const _SC_XOPEN_STREAMS: ::std::os::raw::c_uint = 246;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: ::std::os::raw::c_uint = 247;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: ::std::os::raw::c_uint = 248;
pub type _bindgen_ty_2 = ::std::os::raw::c_uint;
pub const _CS_PATH: ::std::os::raw::c_uint = 0;
pub const _CS_V6_WIDTH_RESTRICTED_ENVS: ::std::os::raw::c_uint = 1;
pub const _CS_GNU_LIBC_VERSION: ::std::os::raw::c_uint = 2;
pub const _CS_GNU_LIBPTHREAD_VERSION: ::std::os::raw::c_uint = 3;
pub const _CS_V5_WIDTH_RESTRICTED_ENVS: ::std::os::raw::c_uint = 4;
pub const _CS_V7_WIDTH_RESTRICTED_ENVS: ::std::os::raw::c_uint = 5;
pub const _CS_LFS_CFLAGS: ::std::os::raw::c_uint = 1000;
pub const _CS_LFS_LDFLAGS: ::std::os::raw::c_uint = 1001;
pub const _CS_LFS_LIBS: ::std::os::raw::c_uint = 1002;
pub const _CS_LFS_LINTFLAGS: ::std::os::raw::c_uint = 1003;
pub const _CS_LFS64_CFLAGS: ::std::os::raw::c_uint = 1004;
pub const _CS_LFS64_LDFLAGS: ::std::os::raw::c_uint = 1005;
pub const _CS_LFS64_LIBS: ::std::os::raw::c_uint = 1006;
pub const _CS_LFS64_LINTFLAGS: ::std::os::raw::c_uint = 1007;
pub const _CS_XBS5_ILP32_OFF32_CFLAGS: ::std::os::raw::c_uint = 1100;
pub const _CS_XBS5_ILP32_OFF32_LDFLAGS: ::std::os::raw::c_uint = 1101;
pub const _CS_XBS5_ILP32_OFF32_LIBS: ::std::os::raw::c_uint = 1102;
pub const _CS_XBS5_ILP32_OFF32_LINTFLAGS: ::std::os::raw::c_uint = 1103;
pub const _CS_XBS5_ILP32_OFFBIG_CFLAGS: ::std::os::raw::c_uint = 1104;
pub const _CS_XBS5_ILP32_OFFBIG_LDFLAGS: ::std::os::raw::c_uint = 1105;
pub const _CS_XBS5_ILP32_OFFBIG_LIBS: ::std::os::raw::c_uint = 1106;
pub const _CS_XBS5_ILP32_OFFBIG_LINTFLAGS: ::std::os::raw::c_uint = 1107;
pub const _CS_XBS5_LP64_OFF64_CFLAGS: ::std::os::raw::c_uint = 1108;
pub const _CS_XBS5_LP64_OFF64_LDFLAGS: ::std::os::raw::c_uint = 1109;
pub const _CS_XBS5_LP64_OFF64_LIBS: ::std::os::raw::c_uint = 1110;
pub const _CS_XBS5_LP64_OFF64_LINTFLAGS: ::std::os::raw::c_uint = 1111;
pub const _CS_XBS5_LPBIG_OFFBIG_CFLAGS: ::std::os::raw::c_uint = 1112;
pub const _CS_XBS5_LPBIG_OFFBIG_LDFLAGS: ::std::os::raw::c_uint = 1113;
pub const _CS_XBS5_LPBIG_OFFBIG_LIBS: ::std::os::raw::c_uint = 1114;
pub const _CS_XBS5_LPBIG_OFFBIG_LINTFLAGS: ::std::os::raw::c_uint = 1115;
pub const _CS_POSIX_V6_ILP32_OFF32_CFLAGS: ::std::os::raw::c_uint = 1116;
pub const _CS_POSIX_V6_ILP32_OFF32_LDFLAGS: ::std::os::raw::c_uint = 1117;
pub const _CS_POSIX_V6_ILP32_OFF32_LIBS: ::std::os::raw::c_uint = 1118;
pub const _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS: ::std::os::raw::c_uint = 1119;
pub const _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS: ::std::os::raw::c_uint = 1120;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS: ::std::os::raw::c_uint = 1121;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LIBS: ::std::os::raw::c_uint = 1122;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS: ::std::os::raw::c_uint = 1123;
pub const _CS_POSIX_V6_LP64_OFF64_CFLAGS: ::std::os::raw::c_uint = 1124;
pub const _CS_POSIX_V6_LP64_OFF64_LDFLAGS: ::std::os::raw::c_uint = 1125;
pub const _CS_POSIX_V6_LP64_OFF64_LIBS: ::std::os::raw::c_uint = 1126;
pub const _CS_POSIX_V6_LP64_OFF64_LINTFLAGS: ::std::os::raw::c_uint = 1127;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS: ::std::os::raw::c_uint = 1128;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS: ::std::os::raw::c_uint = 1129;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LIBS: ::std::os::raw::c_uint = 1130;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS: ::std::os::raw::c_uint = 1131;
pub const _CS_POSIX_V7_ILP32_OFF32_CFLAGS: ::std::os::raw::c_uint = 1132;
pub const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS: ::std::os::raw::c_uint = 1133;
pub const _CS_POSIX_V7_ILP32_OFF32_LIBS: ::std::os::raw::c_uint = 1134;
pub const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS: ::std::os::raw::c_uint = 1135;
pub const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS: ::std::os::raw::c_uint = 1136;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS: ::std::os::raw::c_uint = 1137;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LIBS: ::std::os::raw::c_uint = 1138;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS: ::std::os::raw::c_uint = 1139;
pub const _CS_POSIX_V7_LP64_OFF64_CFLAGS: ::std::os::raw::c_uint = 1140;
pub const _CS_POSIX_V7_LP64_OFF64_LDFLAGS: ::std::os::raw::c_uint = 1141;
pub const _CS_POSIX_V7_LP64_OFF64_LIBS: ::std::os::raw::c_uint = 1142;
pub const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS: ::std::os::raw::c_uint = 1143;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS: ::std::os::raw::c_uint = 1144;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS: ::std::os::raw::c_uint = 1145;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS: ::std::os::raw::c_uint = 1146;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS: ::std::os::raw::c_uint = 1147;
pub const _CS_V6_ENV: ::std::os::raw::c_uint = 1148;
pub const _CS_V7_ENV: ::std::os::raw::c_uint = 1149;
pub type _bindgen_ty_3 = ::std::os::raw::c_uint;
extern "C" {
    pub fn pathconf(
        __path: *const ::std::os::raw::c_char,
        __name: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn fpathconf(
        __fd: ::std::os::raw::c_int,
        __name: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn sysconf(__name: ::std::os::raw::c_int) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn confstr(
        __name: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: size_t,
    ) -> size_t;
}
extern "C" {
    pub fn getpid() -> __pid_t;
}
extern "C" {
    pub fn getppid() -> __pid_t;
}
extern "C" {
    pub fn getpgrp() -> __pid_t;
}
extern "C" {
    pub fn __getpgid(__pid: __pid_t) -> __pid_t;
}
extern "C" {
    pub fn getpgid(__pid: __pid_t) -> __pid_t;
}
extern "C" {
    pub fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setpgrp() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setsid() -> __pid_t;
}
extern "C" {
    pub fn getsid(__pid: __pid_t) -> __pid_t;
}
extern "C" {
    pub fn getuid() -> __uid_t;
}
extern "C" {
    pub fn geteuid() -> __uid_t;
}
extern "C" {
    pub fn getgid() -> __gid_t;
}
extern "C" {
    pub fn getegid() -> __gid_t;
}
extern "C" {
    pub fn getgroups(__size: ::std::os::raw::c_int, __list: *mut __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setuid(__uid: __uid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setreuid(__ruid: __uid_t, __euid: __uid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn seteuid(__uid: __uid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setgid(__gid: __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setregid(__rgid: __gid_t, __egid: __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setegid(__gid: __gid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fork() -> __pid_t;
}
extern "C" {
    pub fn vfork() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ttyname(__fd: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ttyname_r(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isatty(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ttyslot() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn link(
        __from: *const ::std::os::raw::c_char,
        __to: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn linkat(
        __fromfd: ::std::os::raw::c_int,
        __from: *const ::std::os::raw::c_char,
        __tofd: ::std::os::raw::c_int,
        __to: *const ::std::os::raw::c_char,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn symlink(
        __from: *const ::std::os::raw::c_char,
        __to: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn readlink(
        __path: *const ::std::os::raw::c_char,
        __buf: *mut ::std::os::raw::c_char,
        __len: size_t,
    ) -> ssize_t;
}
extern "C" {
    pub fn symlinkat(
        __from: *const ::std::os::raw::c_char,
        __tofd: ::std::os::raw::c_int,
        __to: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn readlinkat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __buf: *mut ::std::os::raw::c_char,
        __len: size_t,
    ) -> ssize_t;
}
extern "C" {
    pub fn unlink(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unlinkat(
        __fd: ::std::os::raw::c_int,
        __name: *const ::std::os::raw::c_char,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rmdir(__path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tcgetpgrp(__fd: ::std::os::raw::c_int) -> __pid_t;
}
extern "C" {
    pub fn tcsetpgrp(__fd: ::std::os::raw::c_int, __pgrp_id: __pid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getlogin() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getlogin_r(
        __name: *mut ::std::os::raw::c_char,
        __name_len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setlogin(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub static mut optarg: *mut ::std::os::raw::c_char;
}
extern "C" {
    pub static mut optind: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut opterr: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut optopt: ::std::os::raw::c_int;
}
extern "C" {
    pub fn getopt(
        ___argc: ::std::os::raw::c_int,
        ___argv: *const *mut ::std::os::raw::c_char,
        __shortopts: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gethostname(__name: *mut ::std::os::raw::c_char, __len: size_t)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sethostname(
        __name: *const ::std::os::raw::c_char,
        __len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sethostid(__id: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getdomainname(
        __name: *mut ::std::os::raw::c_char,
        __len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setdomainname(
        __name: *const ::std::os::raw::c_char,
        __len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vhangup() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn revoke(__file: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn profil(
        __sample_buffer: *mut ::std::os::raw::c_ushort,
        __size: size_t,
        __offset: size_t,
        __scale: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn acct(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getusershell() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn endusershell();
}
extern "C" {
    pub fn setusershell();
}
extern "C" {
    pub fn daemon(
        __nochdir: ::std::os::raw::c_int,
        __noclose: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chroot(__path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpass(__prompt: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fsync(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gethostid() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn sync();
}
extern "C" {
    pub fn getpagesize() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getdtablesize() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn truncate(
        __file: *const ::std::os::raw::c_char,
        __length: __off_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftruncate(__fd: ::std::os::raw::c_int, __length: __off_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn brk(__addr: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sbrk(__delta: isize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn syscall(__sysno: ::std::os::raw::c_long, ...) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn lockf(
        __fd: ::std::os::raw::c_int,
        __cmd: ::std::os::raw::c_int,
        __len: __off_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fdatasync(__fildes: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypt(
        __key: *const ::std::os::raw::c_char,
        __salt: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getentropy(
        __buffer: *mut ::std::os::raw::c_void,
        __length: size_t,
    ) -> ::std::os::raw::c_int;
}
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type __gwchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct imaxdiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_imaxdiv_t() {
    assert_eq!(
        ::std::mem::size_of::<imaxdiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(imaxdiv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<imaxdiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(imaxdiv_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<imaxdiv_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(imaxdiv_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<imaxdiv_t>())).rem as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(imaxdiv_t),
            "::",
            stringify!(rem)
        )
    );
}
extern "C" {
    pub fn imaxabs(__n: intmax_t) -> intmax_t;
}
extern "C" {
    pub fn imaxdiv(__numer: intmax_t, __denom: intmax_t) -> imaxdiv_t;
}
extern "C" {
    pub fn strtoimax(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> intmax_t;
}
extern "C" {
    pub fn strtoumax(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> uintmax_t;
}
extern "C" {
    pub fn wcstoimax(
        __nptr: *const __gwchar_t,
        __endptr: *mut *mut __gwchar_t,
        __base: ::std::os::raw::c_int,
    ) -> intmax_t;
}
extern "C" {
    pub fn wcstoumax(
        __nptr: *const __gwchar_t,
        __endptr: *mut *mut __gwchar_t,
        __base: ::std::os::raw::c_int,
    ) -> uintmax_t;
}
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type nlink_t = __nlink_t;
pub type id_t = __id_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
pub type ulong = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint = ::std::os::raw::c_uint;
pub type u_int8_t = __uint8_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
pub type register_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___sigset_t() {
    assert_eq!(
        ::std::mem::size_of::<__sigset_t>(),
        128usize,
        concat!("Size of: ", stringify!(__sigset_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__sigset_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__sigset_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sigset_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigset_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[test]
fn bindgen_test_layout_timeval() {
    assert_eq!(
        ::std::mem::size_of::<timeval>(),
        16usize,
        concat!("Size of: ", stringify!(timeval))
    );
    assert_eq!(
        ::std::mem::align_of::<timeval>(),
        8usize,
        concat!("Alignment of ", stringify!(timeval))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timeval>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timeval>())).tv_usec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_usec)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[test]
fn bindgen_test_layout_timespec() {
    assert_eq!(
        ::std::mem::size_of::<timespec>(),
        16usize,
        concat!("Size of: ", stringify!(timespec))
    );
    assert_eq!(
        ::std::mem::align_of::<timespec>(),
        8usize,
        concat!("Alignment of ", stringify!(timespec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timespec>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timespec>())).tv_nsec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_nsec)
        )
    );
}
pub type suseconds_t = __suseconds_t;
#[test]
fn bindgen_test_layout_fd_set() {
    assert_eq!(
        ::std::mem::size_of::<fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(fd_set))
    );
    assert_eq!(
        ::std::mem::align_of::<fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(fd_set))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fd_set>())).__fds_bits as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fd_set),
            "::",
            stringify!(__fds_bits)
        )
    );
}
pub type fd_mask = __fd_mask;
extern "C" {
    pub fn select(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pselect(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::std::os::raw::c_int;
}
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[test]
fn bindgen_test_layout___pthread_internal_list() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_internal_list>(),
        16usize,
        concat!("Size of: ", stringify!(__pthread_internal_list))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_internal_list>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_internal_list))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_internal_list>())).__prev as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_internal_list),
            "::",
            stringify!(__prev)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_internal_list>())).__next as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_internal_list),
            "::",
            stringify!(__next)
        )
    );
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_slist {
    pub __next: *mut __pthread_internal_slist,
}
#[test]
fn bindgen_test_layout___pthread_internal_slist() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_internal_slist>(),
        8usize,
        concat!("Size of: ", stringify!(__pthread_internal_slist))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_internal_slist>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_internal_slist))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_internal_slist>())).__next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_internal_slist),
            "::",
            stringify!(__next)
        )
    );
}
pub type __pthread_slist_t = __pthread_internal_slist;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: ::std::os::raw::c_int,
    pub __count: ::std::os::raw::c_uint,
    pub __owner: ::std::os::raw::c_int,
    pub __nusers: ::std::os::raw::c_uint,
    pub __kind: ::std::os::raw::c_int,
    pub __spins: ::std::os::raw::c_short,
    pub __elision: ::std::os::raw::c_short,
    pub __list: __pthread_list_t,
}
#[test]
fn bindgen_test_layout___pthread_mutex_s() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_mutex_s>(),
        40usize,
        concat!("Size of: ", stringify!(__pthread_mutex_s))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_mutex_s>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_mutex_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__lock as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__lock)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__count as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__owner as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__owner)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__nusers as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__nusers)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__kind as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__kind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__spins as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__spins)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__elision as *const _ as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__elision)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__list as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__list)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::std::os::raw::c_uint,
    pub __writers: ::std::os::raw::c_uint,
    pub __wrphase_futex: ::std::os::raw::c_uint,
    pub __writers_futex: ::std::os::raw::c_uint,
    pub __pad3: ::std::os::raw::c_uint,
    pub __pad4: ::std::os::raw::c_uint,
    pub __cur_writer: ::std::os::raw::c_int,
    pub __shared: ::std::os::raw::c_int,
    pub __rwelision: ::std::os::raw::c_schar,
    pub __pad1: [::std::os::raw::c_uchar; 7usize],
    pub __pad2: ::std::os::raw::c_ulong,
    pub __flags: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_rwlock_arch_t() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_rwlock_arch_t>(),
        56usize,
        concat!("Size of: ", stringify!(__pthread_rwlock_arch_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_rwlock_arch_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_rwlock_arch_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__readers as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__readers)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__writers as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__writers)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__wrphase_futex as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__wrphase_futex)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__writers_futex as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__writers_futex)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__pad3 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__pad4 as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__cur_writer as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__cur_writer)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__shared as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__shared)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__rwelision as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__rwelision)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__pad1 as *const _ as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__pad2 as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__flags as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__flags)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __bindgen_anon_1: __pthread_cond_s__bindgen_ty_1,
    pub __bindgen_anon_2: __pthread_cond_s__bindgen_ty_2,
    pub __g_refs: [::std::os::raw::c_uint; 2usize],
    pub __g_size: [::std::os::raw::c_uint; 2usize],
    pub __g1_orig_size: ::std::os::raw::c_uint,
    pub __wrefs: ::std::os::raw::c_uint,
    pub __g_signals: [::std::os::raw::c_uint; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_1 {
    pub __wseq: ::std::os::raw::c_ulonglong,
    pub __wseq32: __pthread_cond_s__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>())).__low
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(__low)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>())).__high
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(__high)
        )
    );
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(__pthread_cond_s__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_1>())).__wseq as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1),
            "::",
            stringify!(__wseq)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_1>())).__wseq32 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1),
            "::",
            stringify!(__wseq32)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_2 {
    pub __g1_start: ::std::os::raw::c_ulonglong,
    pub __g1_start32: __pthread_cond_s__bindgen_ty_2__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_2__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>())).__low
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(__low)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>())).__high
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(__high)
        )
    );
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s__bindgen_ty_2>(),
        8usize,
        concat!("Size of: ", stringify!(__pthread_cond_s__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s__bindgen_ty_2>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_2>())).__g1_start as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2),
            "::",
            stringify!(__g1_start)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_2>())).__g1_start32 as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2),
            "::",
            stringify!(__g1_start32)
        )
    );
}
#[test]
fn bindgen_test_layout___pthread_cond_s() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s>(),
        48usize,
        concat!("Size of: ", stringify!(__pthread_cond_s))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_cond_s>())).__g_refs as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_refs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_cond_s>())).__g_size as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_cond_s>())).__g1_orig_size as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g1_orig_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_cond_s>())).__wrefs as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__wrefs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_cond_s>())).__g_signals as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_signals)
        )
    );
}
pub type pthread_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_pthread_mutexattr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_mutexattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_mutexattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_mutexattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_mutexattr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_mutexattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutexattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_mutexattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutexattr_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_pthread_condattr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_condattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_condattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_condattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_condattr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_condattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_condattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_condattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_condattr_t),
            "::",
            stringify!(__align)
        )
    );
}
pub type pthread_key_t = ::std::os::raw::c_uint;
pub type pthread_once_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[test]
fn bindgen_test_layout_pthread_attr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_attr_t>(),
        56usize,
        concat!("Size of: ", stringify!(pthread_attr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_attr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_attr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_attr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_attr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_attr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_attr_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::std::os::raw::c_char; 40usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 5usize],
}
#[test]
fn bindgen_test_layout_pthread_mutex_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_mutex_t>(),
        40usize,
        concat!("Size of: ", stringify!(pthread_mutex_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_mutex_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_mutex_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_mutex_t>())).__data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_mutex_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_mutex_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::std::os::raw::c_char; 48usize],
    pub __align: ::std::os::raw::c_longlong,
    _bindgen_union_align: [u64; 6usize],
}
#[test]
fn bindgen_test_layout_pthread_cond_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_cond_t>(),
        48usize,
        concat!("Size of: ", stringify!(pthread_cond_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_cond_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_cond_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_cond_t>())).__data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_cond_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_cond_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[test]
fn bindgen_test_layout_pthread_rwlock_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_rwlock_t>(),
        56usize,
        concat!("Size of: ", stringify!(pthread_rwlock_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_rwlock_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_rwlock_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_rwlock_t>())).__data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_rwlock_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_rwlock_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [::std::os::raw::c_char; 8usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_pthread_rwlockattr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_rwlockattr_t>(),
        8usize,
        concat!("Size of: ", stringify!(pthread_rwlockattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_rwlockattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_rwlockattr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_rwlockattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlockattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_rwlockattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlockattr_t),
            "::",
            stringify!(__align)
        )
    );
}
pub type pthread_spinlock_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [::std::os::raw::c_char; 32usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 4usize],
}
#[test]
fn bindgen_test_layout_pthread_barrier_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_barrier_t>(),
        32usize,
        concat!("Size of: ", stringify!(pthread_barrier_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_barrier_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_barrier_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_barrier_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrier_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_barrier_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrier_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_pthread_barrierattr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_barrierattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_barrierattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_barrierattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_barrierattr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_barrierattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrierattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_barrierattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrierattr_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iovec {
    pub iov_base: *mut ::std::os::raw::c_void,
    pub iov_len: size_t,
}
#[test]
fn bindgen_test_layout_iovec() {
    assert_eq!(
        ::std::mem::size_of::<iovec>(),
        16usize,
        concat!("Size of: ", stringify!(iovec))
    );
    assert_eq!(
        ::std::mem::align_of::<iovec>(),
        8usize,
        concat!("Alignment of ", stringify!(iovec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<iovec>())).iov_base as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(iovec),
            "::",
            stringify!(iov_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<iovec>())).iov_len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(iovec),
            "::",
            stringify!(iov_len)
        )
    );
}
pub const __socket_type_SOCK_STREAM: __socket_type = 1;
pub const __socket_type_SOCK_DGRAM: __socket_type = 2;
pub const __socket_type_SOCK_RAW: __socket_type = 3;
pub const __socket_type_SOCK_RDM: __socket_type = 4;
pub const __socket_type_SOCK_SEQPACKET: __socket_type = 5;
pub const __socket_type_SOCK_DCCP: __socket_type = 6;
pub const __socket_type_SOCK_PACKET: __socket_type = 10;
pub const __socket_type_SOCK_CLOEXEC: __socket_type = 524288;
pub const __socket_type_SOCK_NONBLOCK: __socket_type = 2048;
pub type __socket_type = ::std::os::raw::c_uint;
pub type sa_family_t = ::std::os::raw::c_ushort;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::std::os::raw::c_char; 14usize],
}
#[test]
fn bindgen_test_layout_sockaddr() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr>(),
        16usize,
        concat!("Size of: ", stringify!(sockaddr))
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr>(),
        2usize,
        concat!("Alignment of ", stringify!(sockaddr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr>())).sa_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr),
            "::",
            stringify!(sa_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr>())).sa_data as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr),
            "::",
            stringify!(sa_data)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [::std::os::raw::c_char; 118usize],
    pub __ss_align: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout_sockaddr_storage() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr_storage>(),
        128usize,
        concat!("Size of: ", stringify!(sockaddr_storage))
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr_storage>(),
        8usize,
        concat!("Alignment of ", stringify!(sockaddr_storage))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_storage>())).ss_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_storage),
            "::",
            stringify!(ss_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_storage>())).__ss_padding as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_storage),
            "::",
            stringify!(__ss_padding)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_storage>())).__ss_align as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_storage),
            "::",
            stringify!(__ss_align)
        )
    );
}
pub const MSG_OOB: ::std::os::raw::c_uint = 1;
pub const MSG_PEEK: ::std::os::raw::c_uint = 2;
pub const MSG_DONTROUTE: ::std::os::raw::c_uint = 4;
pub const MSG_CTRUNC: ::std::os::raw::c_uint = 8;
pub const MSG_PROXY: ::std::os::raw::c_uint = 16;
pub const MSG_TRUNC: ::std::os::raw::c_uint = 32;
pub const MSG_DONTWAIT: ::std::os::raw::c_uint = 64;
pub const MSG_EOR: ::std::os::raw::c_uint = 128;
pub const MSG_WAITALL: ::std::os::raw::c_uint = 256;
pub const MSG_FIN: ::std::os::raw::c_uint = 512;
pub const MSG_SYN: ::std::os::raw::c_uint = 1024;
pub const MSG_CONFIRM: ::std::os::raw::c_uint = 2048;
pub const MSG_RST: ::std::os::raw::c_uint = 4096;
pub const MSG_ERRQUEUE: ::std::os::raw::c_uint = 8192;
pub const MSG_NOSIGNAL: ::std::os::raw::c_uint = 16384;
pub const MSG_MORE: ::std::os::raw::c_uint = 32768;
pub const MSG_WAITFORONE: ::std::os::raw::c_uint = 65536;
pub const MSG_BATCH: ::std::os::raw::c_uint = 262144;
pub const MSG_ZEROCOPY: ::std::os::raw::c_uint = 67108864;
pub const MSG_FASTOPEN: ::std::os::raw::c_uint = 536870912;
pub const MSG_CMSG_CLOEXEC: ::std::os::raw::c_uint = 1073741824;
pub type _bindgen_ty_4 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct msghdr {
    pub msg_name: *mut ::std::os::raw::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut ::std::os::raw::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_msghdr() {
    assert_eq!(
        ::std::mem::size_of::<msghdr>(),
        56usize,
        concat!("Size of: ", stringify!(msghdr))
    );
    assert_eq!(
        ::std::mem::align_of::<msghdr>(),
        8usize,
        concat!("Alignment of ", stringify!(msghdr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msghdr>())).msg_name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(msghdr),
            "::",
            stringify!(msg_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msghdr>())).msg_namelen as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(msghdr),
            "::",
            stringify!(msg_namelen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msghdr>())).msg_iov as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(msghdr),
            "::",
            stringify!(msg_iov)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msghdr>())).msg_iovlen as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(msghdr),
            "::",
            stringify!(msg_iovlen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msghdr>())).msg_control as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(msghdr),
            "::",
            stringify!(msg_control)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msghdr>())).msg_controllen as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(msghdr),
            "::",
            stringify!(msg_controllen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msghdr>())).msg_flags as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(msghdr),
            "::",
            stringify!(msg_flags)
        )
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: ::std::os::raw::c_int,
    pub cmsg_type: ::std::os::raw::c_int,
    pub __cmsg_data: __IncompleteArrayField<::std::os::raw::c_uchar>,
}
#[test]
fn bindgen_test_layout_cmsghdr() {
    assert_eq!(
        ::std::mem::size_of::<cmsghdr>(),
        16usize,
        concat!("Size of: ", stringify!(cmsghdr))
    );
    assert_eq!(
        ::std::mem::align_of::<cmsghdr>(),
        8usize,
        concat!("Alignment of ", stringify!(cmsghdr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cmsghdr>())).cmsg_len as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(cmsghdr),
            "::",
            stringify!(cmsg_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cmsghdr>())).cmsg_level as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(cmsghdr),
            "::",
            stringify!(cmsg_level)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cmsghdr>())).cmsg_type as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(cmsghdr),
            "::",
            stringify!(cmsg_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<cmsghdr>())).__cmsg_data as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(cmsghdr),
            "::",
            stringify!(__cmsg_data)
        )
    );
}
extern "C" {
    pub fn __cmsg_nxthdr(__mhdr: *mut msghdr, __cmsg: *mut cmsghdr) -> *mut cmsghdr;
}
pub const SCM_RIGHTS: ::std::os::raw::c_uint = 1;
pub type _bindgen_ty_5 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fd_set {
    pub fds_bits: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___kernel_fd_set() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_fd_set>())).fds_bits as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_fd_set),
            "::",
            stringify!(fds_bits)
        )
    );
}
pub type __kernel_sighandler_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
pub type __kernel_key_t = ::std::os::raw::c_int;
pub type __kernel_mqd_t = ::std::os::raw::c_int;
pub type __kernel_old_uid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_gid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_dev_t = ::std::os::raw::c_ulong;
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
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fsid_t {
    pub val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___kernel_fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_fsid_t>())).val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_fsid_t),
            "::",
            stringify!(val)
        )
    );
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct linger {
    pub l_onoff: ::std::os::raw::c_int,
    pub l_linger: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_linger() {
    assert_eq!(
        ::std::mem::size_of::<linger>(),
        8usize,
        concat!("Size of: ", stringify!(linger))
    );
    assert_eq!(
        ::std::mem::align_of::<linger>(),
        4usize,
        concat!("Alignment of ", stringify!(linger))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<linger>())).l_onoff as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(linger),
            "::",
            stringify!(l_onoff)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<linger>())).l_linger as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(linger),
            "::",
            stringify!(l_linger)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osockaddr {
    pub sa_family: ::std::os::raw::c_ushort,
    pub sa_data: [::std::os::raw::c_uchar; 14usize],
}
#[test]
fn bindgen_test_layout_osockaddr() {
    assert_eq!(
        ::std::mem::size_of::<osockaddr>(),
        16usize,
        concat!("Size of: ", stringify!(osockaddr))
    );
    assert_eq!(
        ::std::mem::align_of::<osockaddr>(),
        2usize,
        concat!("Alignment of ", stringify!(osockaddr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<osockaddr>())).sa_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(osockaddr),
            "::",
            stringify!(sa_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<osockaddr>())).sa_data as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(osockaddr),
            "::",
            stringify!(sa_data)
        )
    );
}
pub const SHUT_RD: ::std::os::raw::c_uint = 0;
pub const SHUT_WR: ::std::os::raw::c_uint = 1;
pub const SHUT_RDWR: ::std::os::raw::c_uint = 2;
pub type _bindgen_ty_6 = ::std::os::raw::c_uint;
extern "C" {
    pub fn socket(
        __domain: ::std::os::raw::c_int,
        __type: ::std::os::raw::c_int,
        __protocol: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn socketpair(
        __domain: ::std::os::raw::c_int,
        __type: ::std::os::raw::c_int,
        __protocol: ::std::os::raw::c_int,
        __fds: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bind(
        __fd: ::std::os::raw::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getsockname(
        __fd: ::std::os::raw::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn connect(
        __fd: ::std::os::raw::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getpeername(
        __fd: ::std::os::raw::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn send(
        __fd: ::std::os::raw::c_int,
        __buf: *const ::std::os::raw::c_void,
        __n: size_t,
        __flags: ::std::os::raw::c_int,
    ) -> ssize_t;
}
extern "C" {
    pub fn recv(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_void,
        __n: size_t,
        __flags: ::std::os::raw::c_int,
    ) -> ssize_t;
}
extern "C" {
    pub fn sendto(
        __fd: ::std::os::raw::c_int,
        __buf: *const ::std::os::raw::c_void,
        __n: size_t,
        __flags: ::std::os::raw::c_int,
        __addr: *const sockaddr,
        __addr_len: socklen_t,
    ) -> ssize_t;
}
extern "C" {
    pub fn recvfrom(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_void,
        __n: size_t,
        __flags: ::std::os::raw::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
}
extern "C" {
    pub fn sendmsg(
        __fd: ::std::os::raw::c_int,
        __message: *const msghdr,
        __flags: ::std::os::raw::c_int,
    ) -> ssize_t;
}
extern "C" {
    pub fn recvmsg(
        __fd: ::std::os::raw::c_int,
        __message: *mut msghdr,
        __flags: ::std::os::raw::c_int,
    ) -> ssize_t;
}
extern "C" {
    pub fn getsockopt(
        __fd: ::std::os::raw::c_int,
        __level: ::std::os::raw::c_int,
        __optname: ::std::os::raw::c_int,
        __optval: *mut ::std::os::raw::c_void,
        __optlen: *mut socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setsockopt(
        __fd: ::std::os::raw::c_int,
        __level: ::std::os::raw::c_int,
        __optname: ::std::os::raw::c_int,
        __optval: *const ::std::os::raw::c_void,
        __optlen: socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn listen(__fd: ::std::os::raw::c_int, __n: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn accept(
        __fd: ::std::os::raw::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn shutdown(
        __fd: ::std::os::raw::c_int,
        __how: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sockatmark(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn isfdtype(
        __fd: ::std::os::raw::c_int,
        __fdtype: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type in_addr_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
#[test]
fn bindgen_test_layout_in_addr() {
    assert_eq!(
        ::std::mem::size_of::<in_addr>(),
        4usize,
        concat!("Size of: ", stringify!(in_addr))
    );
    assert_eq!(
        ::std::mem::align_of::<in_addr>(),
        4usize,
        concat!("Alignment of ", stringify!(in_addr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<in_addr>())).s_addr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in_addr),
            "::",
            stringify!(s_addr)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_opts {
    pub ip_dst: in_addr,
    pub ip_opts: [::std::os::raw::c_char; 40usize],
}
#[test]
fn bindgen_test_layout_ip_opts() {
    assert_eq!(
        ::std::mem::size_of::<ip_opts>(),
        44usize,
        concat!("Size of: ", stringify!(ip_opts))
    );
    assert_eq!(
        ::std::mem::align_of::<ip_opts>(),
        4usize,
        concat!("Alignment of ", stringify!(ip_opts))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_opts>())).ip_dst as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_opts),
            "::",
            stringify!(ip_dst)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_opts>())).ip_opts as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_opts),
            "::",
            stringify!(ip_opts)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ip_mreqn {
    pub imr_multiaddr: in_addr,
    pub imr_address: in_addr,
    pub imr_ifindex: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ip_mreqn() {
    assert_eq!(
        ::std::mem::size_of::<ip_mreqn>(),
        12usize,
        concat!("Size of: ", stringify!(ip_mreqn))
    );
    assert_eq!(
        ::std::mem::align_of::<ip_mreqn>(),
        4usize,
        concat!("Alignment of ", stringify!(ip_mreqn))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_mreqn>())).imr_multiaddr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_mreqn),
            "::",
            stringify!(imr_multiaddr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_mreqn>())).imr_address as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_mreqn),
            "::",
            stringify!(imr_address)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_mreqn>())).imr_ifindex as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_mreqn),
            "::",
            stringify!(imr_ifindex)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct in_pktinfo {
    pub ipi_ifindex: ::std::os::raw::c_int,
    pub ipi_spec_dst: in_addr,
    pub ipi_addr: in_addr,
}
#[test]
fn bindgen_test_layout_in_pktinfo() {
    assert_eq!(
        ::std::mem::size_of::<in_pktinfo>(),
        12usize,
        concat!("Size of: ", stringify!(in_pktinfo))
    );
    assert_eq!(
        ::std::mem::align_of::<in_pktinfo>(),
        4usize,
        concat!("Alignment of ", stringify!(in_pktinfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<in_pktinfo>())).ipi_ifindex as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in_pktinfo),
            "::",
            stringify!(ipi_ifindex)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<in_pktinfo>())).ipi_spec_dst as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(in_pktinfo),
            "::",
            stringify!(ipi_spec_dst)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<in_pktinfo>())).ipi_addr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(in_pktinfo),
            "::",
            stringify!(ipi_addr)
        )
    );
}
pub const IPPROTO_IP: ::std::os::raw::c_uint = 0;
pub const IPPROTO_ICMP: ::std::os::raw::c_uint = 1;
pub const IPPROTO_IGMP: ::std::os::raw::c_uint = 2;
pub const IPPROTO_IPIP: ::std::os::raw::c_uint = 4;
pub const IPPROTO_TCP: ::std::os::raw::c_uint = 6;
pub const IPPROTO_EGP: ::std::os::raw::c_uint = 8;
pub const IPPROTO_PUP: ::std::os::raw::c_uint = 12;
pub const IPPROTO_UDP: ::std::os::raw::c_uint = 17;
pub const IPPROTO_IDP: ::std::os::raw::c_uint = 22;
pub const IPPROTO_TP: ::std::os::raw::c_uint = 29;
pub const IPPROTO_DCCP: ::std::os::raw::c_uint = 33;
pub const IPPROTO_IPV6: ::std::os::raw::c_uint = 41;
pub const IPPROTO_RSVP: ::std::os::raw::c_uint = 46;
pub const IPPROTO_GRE: ::std::os::raw::c_uint = 47;
pub const IPPROTO_ESP: ::std::os::raw::c_uint = 50;
pub const IPPROTO_AH: ::std::os::raw::c_uint = 51;
pub const IPPROTO_MTP: ::std::os::raw::c_uint = 92;
pub const IPPROTO_BEETPH: ::std::os::raw::c_uint = 94;
pub const IPPROTO_ENCAP: ::std::os::raw::c_uint = 98;
pub const IPPROTO_PIM: ::std::os::raw::c_uint = 103;
pub const IPPROTO_COMP: ::std::os::raw::c_uint = 108;
pub const IPPROTO_SCTP: ::std::os::raw::c_uint = 132;
pub const IPPROTO_UDPLITE: ::std::os::raw::c_uint = 136;
pub const IPPROTO_MPLS: ::std::os::raw::c_uint = 137;
pub const IPPROTO_RAW: ::std::os::raw::c_uint = 255;
pub const IPPROTO_MAX: ::std::os::raw::c_uint = 256;
pub type _bindgen_ty_7 = ::std::os::raw::c_uint;
pub const IPPROTO_HOPOPTS: ::std::os::raw::c_uint = 0;
pub const IPPROTO_ROUTING: ::std::os::raw::c_uint = 43;
pub const IPPROTO_FRAGMENT: ::std::os::raw::c_uint = 44;
pub const IPPROTO_ICMPV6: ::std::os::raw::c_uint = 58;
pub const IPPROTO_NONE: ::std::os::raw::c_uint = 59;
pub const IPPROTO_DSTOPTS: ::std::os::raw::c_uint = 60;
pub const IPPROTO_MH: ::std::os::raw::c_uint = 135;
pub type _bindgen_ty_8 = ::std::os::raw::c_uint;
pub type in_port_t = u16;
pub const IPPORT_ECHO: ::std::os::raw::c_uint = 7;
pub const IPPORT_DISCARD: ::std::os::raw::c_uint = 9;
pub const IPPORT_SYSTAT: ::std::os::raw::c_uint = 11;
pub const IPPORT_DAYTIME: ::std::os::raw::c_uint = 13;
pub const IPPORT_NETSTAT: ::std::os::raw::c_uint = 15;
pub const IPPORT_FTP: ::std::os::raw::c_uint = 21;
pub const IPPORT_TELNET: ::std::os::raw::c_uint = 23;
pub const IPPORT_SMTP: ::std::os::raw::c_uint = 25;
pub const IPPORT_TIMESERVER: ::std::os::raw::c_uint = 37;
pub const IPPORT_NAMESERVER: ::std::os::raw::c_uint = 42;
pub const IPPORT_WHOIS: ::std::os::raw::c_uint = 43;
pub const IPPORT_MTP: ::std::os::raw::c_uint = 57;
pub const IPPORT_TFTP: ::std::os::raw::c_uint = 69;
pub const IPPORT_RJE: ::std::os::raw::c_uint = 77;
pub const IPPORT_FINGER: ::std::os::raw::c_uint = 79;
pub const IPPORT_TTYLINK: ::std::os::raw::c_uint = 87;
pub const IPPORT_SUPDUP: ::std::os::raw::c_uint = 95;
pub const IPPORT_EXECSERVER: ::std::os::raw::c_uint = 512;
pub const IPPORT_LOGINSERVER: ::std::os::raw::c_uint = 513;
pub const IPPORT_CMDSERVER: ::std::os::raw::c_uint = 514;
pub const IPPORT_EFSSERVER: ::std::os::raw::c_uint = 520;
pub const IPPORT_BIFFUDP: ::std::os::raw::c_uint = 512;
pub const IPPORT_WHOSERVER: ::std::os::raw::c_uint = 513;
pub const IPPORT_ROUTESERVER: ::std::os::raw::c_uint = 520;
pub const IPPORT_USERRESERVED: ::std::os::raw::c_uint = 5000;
pub type _bindgen_ty_9 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub __in6_u: in6_addr__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union in6_addr__bindgen_ty_1 {
    pub __u6_addr8: [u8; 16usize],
    pub __u6_addr16: [u16; 8usize],
    pub __u6_addr32: [u32; 4usize],
    _bindgen_union_align: [u32; 4usize],
}
#[test]
fn bindgen_test_layout_in6_addr__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<in6_addr__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(in6_addr__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<in6_addr__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(in6_addr__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<in6_addr__bindgen_ty_1>())).__u6_addr8 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in6_addr__bindgen_ty_1),
            "::",
            stringify!(__u6_addr8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<in6_addr__bindgen_ty_1>())).__u6_addr16 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in6_addr__bindgen_ty_1),
            "::",
            stringify!(__u6_addr16)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<in6_addr__bindgen_ty_1>())).__u6_addr32 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in6_addr__bindgen_ty_1),
            "::",
            stringify!(__u6_addr32)
        )
    );
}
#[test]
fn bindgen_test_layout_in6_addr() {
    assert_eq!(
        ::std::mem::size_of::<in6_addr>(),
        16usize,
        concat!("Size of: ", stringify!(in6_addr))
    );
    assert_eq!(
        ::std::mem::align_of::<in6_addr>(),
        4usize,
        concat!("Alignment of ", stringify!(in6_addr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<in6_addr>())).__in6_u as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in6_addr),
            "::",
            stringify!(__in6_u)
        )
    );
}
extern "C" {
    pub static in6addr_any: in6_addr;
}
extern "C" {
    pub static in6addr_loopback: in6_addr;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_sockaddr_in() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr_in>(),
        16usize,
        concat!("Size of: ", stringify!(sockaddr_in))
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr_in>(),
        4usize,
        concat!("Alignment of ", stringify!(sockaddr_in))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in>())).sin_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in),
            "::",
            stringify!(sin_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in>())).sin_port as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in),
            "::",
            stringify!(sin_port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in>())).sin_addr as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in),
            "::",
            stringify!(sin_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in>())).sin_zero as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in),
            "::",
            stringify!(sin_zero)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}
#[test]
fn bindgen_test_layout_sockaddr_in6() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr_in6>(),
        28usize,
        concat!("Size of: ", stringify!(sockaddr_in6))
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr_in6>(),
        4usize,
        concat!("Alignment of ", stringify!(sockaddr_in6))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in6>())).sin6_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in6),
            "::",
            stringify!(sin6_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in6>())).sin6_port as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in6),
            "::",
            stringify!(sin6_port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in6>())).sin6_flowinfo as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in6),
            "::",
            stringify!(sin6_flowinfo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in6>())).sin6_addr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in6),
            "::",
            stringify!(sin6_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in6>())).sin6_scope_id as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in6),
            "::",
            stringify!(sin6_scope_id)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}
#[test]
fn bindgen_test_layout_ip_mreq() {
    assert_eq!(
        ::std::mem::size_of::<ip_mreq>(),
        8usize,
        concat!("Size of: ", stringify!(ip_mreq))
    );
    assert_eq!(
        ::std::mem::align_of::<ip_mreq>(),
        4usize,
        concat!("Alignment of ", stringify!(ip_mreq))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_mreq>())).imr_multiaddr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_mreq),
            "::",
            stringify!(imr_multiaddr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_mreq>())).imr_interface as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_mreq),
            "::",
            stringify!(imr_interface)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ip_mreq_source {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
    pub imr_sourceaddr: in_addr,
}
#[test]
fn bindgen_test_layout_ip_mreq_source() {
    assert_eq!(
        ::std::mem::size_of::<ip_mreq_source>(),
        12usize,
        concat!("Size of: ", stringify!(ip_mreq_source))
    );
    assert_eq!(
        ::std::mem::align_of::<ip_mreq_source>(),
        4usize,
        concat!("Alignment of ", stringify!(ip_mreq_source))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_mreq_source>())).imr_multiaddr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_mreq_source),
            "::",
            stringify!(imr_multiaddr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_mreq_source>())).imr_interface as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_mreq_source),
            "::",
            stringify!(imr_interface)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_mreq_source>())).imr_sourceaddr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_mreq_source),
            "::",
            stringify!(imr_sourceaddr)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_ipv6_mreq() {
    assert_eq!(
        ::std::mem::size_of::<ipv6_mreq>(),
        20usize,
        concat!("Size of: ", stringify!(ipv6_mreq))
    );
    assert_eq!(
        ::std::mem::align_of::<ipv6_mreq>(),
        4usize,
        concat!("Alignment of ", stringify!(ipv6_mreq))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ipv6_mreq>())).ipv6mr_multiaddr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ipv6_mreq),
            "::",
            stringify!(ipv6mr_multiaddr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ipv6_mreq>())).ipv6mr_interface as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ipv6_mreq),
            "::",
            stringify!(ipv6mr_interface)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_req {
    pub gr_interface: u32,
    pub gr_group: sockaddr_storage,
}
#[test]
fn bindgen_test_layout_group_req() {
    assert_eq!(
        ::std::mem::size_of::<group_req>(),
        136usize,
        concat!("Size of: ", stringify!(group_req))
    );
    assert_eq!(
        ::std::mem::align_of::<group_req>(),
        8usize,
        concat!("Alignment of ", stringify!(group_req))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_req>())).gr_interface as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(group_req),
            "::",
            stringify!(gr_interface)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_req>())).gr_group as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(group_req),
            "::",
            stringify!(gr_group)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_source_req {
    pub gsr_interface: u32,
    pub gsr_group: sockaddr_storage,
    pub gsr_source: sockaddr_storage,
}
#[test]
fn bindgen_test_layout_group_source_req() {
    assert_eq!(
        ::std::mem::size_of::<group_source_req>(),
        264usize,
        concat!("Size of: ", stringify!(group_source_req))
    );
    assert_eq!(
        ::std::mem::align_of::<group_source_req>(),
        8usize,
        concat!("Alignment of ", stringify!(group_source_req))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_source_req>())).gsr_interface as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(group_source_req),
            "::",
            stringify!(gsr_interface)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_source_req>())).gsr_group as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(group_source_req),
            "::",
            stringify!(gsr_group)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_source_req>())).gsr_source as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(group_source_req),
            "::",
            stringify!(gsr_source)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ip_msfilter {
    pub imsf_multiaddr: in_addr,
    pub imsf_interface: in_addr,
    pub imsf_fmode: u32,
    pub imsf_numsrc: u32,
    pub imsf_slist: [in_addr; 1usize],
}
#[test]
fn bindgen_test_layout_ip_msfilter() {
    assert_eq!(
        ::std::mem::size_of::<ip_msfilter>(),
        20usize,
        concat!("Size of: ", stringify!(ip_msfilter))
    );
    assert_eq!(
        ::std::mem::align_of::<ip_msfilter>(),
        4usize,
        concat!("Alignment of ", stringify!(ip_msfilter))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_msfilter>())).imsf_multiaddr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_msfilter),
            "::",
            stringify!(imsf_multiaddr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_msfilter>())).imsf_interface as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_msfilter),
            "::",
            stringify!(imsf_interface)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_msfilter>())).imsf_fmode as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_msfilter),
            "::",
            stringify!(imsf_fmode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_msfilter>())).imsf_numsrc as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_msfilter),
            "::",
            stringify!(imsf_numsrc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_msfilter>())).imsf_slist as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_msfilter),
            "::",
            stringify!(imsf_slist)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_filter {
    pub gf_interface: u32,
    pub gf_group: sockaddr_storage,
    pub gf_fmode: u32,
    pub gf_numsrc: u32,
    pub gf_slist: [sockaddr_storage; 1usize],
}
#[test]
fn bindgen_test_layout_group_filter() {
    assert_eq!(
        ::std::mem::size_of::<group_filter>(),
        272usize,
        concat!("Size of: ", stringify!(group_filter))
    );
    assert_eq!(
        ::std::mem::align_of::<group_filter>(),
        8usize,
        concat!("Alignment of ", stringify!(group_filter))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_filter>())).gf_interface as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(group_filter),
            "::",
            stringify!(gf_interface)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_filter>())).gf_group as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(group_filter),
            "::",
            stringify!(gf_group)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_filter>())).gf_fmode as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(group_filter),
            "::",
            stringify!(gf_fmode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_filter>())).gf_numsrc as *const _ as usize },
        140usize,
        concat!(
            "Offset of field: ",
            stringify!(group_filter),
            "::",
            stringify!(gf_numsrc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<group_filter>())).gf_slist as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(group_filter),
            "::",
            stringify!(gf_slist)
        )
    );
}
extern "C" {
    pub fn ntohl(__netlong: u32) -> u32;
}
extern "C" {
    pub fn ntohs(__netshort: u16) -> u16;
}
extern "C" {
    pub fn htonl(__hostlong: u32) -> u32;
}
extern "C" {
    pub fn htons(__hostshort: u16) -> u16;
}
extern "C" {
    pub fn bindresvport(
        __sockfd: ::std::os::raw::c_int,
        __sock_in: *mut sockaddr_in,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bindresvport6(
        __sockfd: ::std::os::raw::c_int,
        __sock_in: *mut sockaddr_in6,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rpcent {
    pub r_name: *mut ::std::os::raw::c_char,
    pub r_aliases: *mut *mut ::std::os::raw::c_char,
    pub r_number: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_rpcent() {
    assert_eq!(
        ::std::mem::size_of::<rpcent>(),
        24usize,
        concat!("Size of: ", stringify!(rpcent))
    );
    assert_eq!(
        ::std::mem::align_of::<rpcent>(),
        8usize,
        concat!("Alignment of ", stringify!(rpcent))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpcent>())).r_name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rpcent),
            "::",
            stringify!(r_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpcent>())).r_aliases as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rpcent),
            "::",
            stringify!(r_aliases)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpcent>())).r_number as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rpcent),
            "::",
            stringify!(r_number)
        )
    );
}
extern "C" {
    pub fn setrpcent(__stayopen: ::std::os::raw::c_int);
}
extern "C" {
    pub fn endrpcent();
}
extern "C" {
    pub fn getrpcbyname(__name: *const ::std::os::raw::c_char) -> *mut rpcent;
}
extern "C" {
    pub fn getrpcbynumber(__number: ::std::os::raw::c_int) -> *mut rpcent;
}
extern "C" {
    pub fn getrpcent() -> *mut rpcent;
}
extern "C" {
    pub fn getrpcbyname_r(
        __name: *const ::std::os::raw::c_char,
        __result_buf: *mut rpcent,
        __buffer: *mut ::std::os::raw::c_char,
        __buflen: size_t,
        __result: *mut *mut rpcent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getrpcbynumber_r(
        __number: ::std::os::raw::c_int,
        __result_buf: *mut rpcent,
        __buffer: *mut ::std::os::raw::c_char,
        __buflen: size_t,
        __result: *mut *mut rpcent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getrpcent_r(
        __result_buf: *mut rpcent,
        __buffer: *mut ::std::os::raw::c_char,
        __buflen: size_t,
        __result: *mut *mut rpcent,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct netent {
    pub n_name: *mut ::std::os::raw::c_char,
    pub n_aliases: *mut *mut ::std::os::raw::c_char,
    pub n_addrtype: ::std::os::raw::c_int,
    pub n_net: u32,
}
#[test]
fn bindgen_test_layout_netent() {
    assert_eq!(
        ::std::mem::size_of::<netent>(),
        24usize,
        concat!("Size of: ", stringify!(netent))
    );
    assert_eq!(
        ::std::mem::align_of::<netent>(),
        8usize,
        concat!("Alignment of ", stringify!(netent))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<netent>())).n_name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(netent),
            "::",
            stringify!(n_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<netent>())).n_aliases as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(netent),
            "::",
            stringify!(n_aliases)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<netent>())).n_addrtype as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(netent),
            "::",
            stringify!(n_addrtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<netent>())).n_net as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(netent),
            "::",
            stringify!(n_net)
        )
    );
}
extern "C" {
    pub fn __h_errno_location() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn herror(__str: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn hstrerror(__err_num: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hostent {
    pub h_name: *mut ::std::os::raw::c_char,
    pub h_aliases: *mut *mut ::std::os::raw::c_char,
    pub h_addrtype: ::std::os::raw::c_int,
    pub h_length: ::std::os::raw::c_int,
    pub h_addr_list: *mut *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_hostent() {
    assert_eq!(
        ::std::mem::size_of::<hostent>(),
        32usize,
        concat!("Size of: ", stringify!(hostent))
    );
    assert_eq!(
        ::std::mem::align_of::<hostent>(),
        8usize,
        concat!("Alignment of ", stringify!(hostent))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hostent>())).h_name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(hostent),
            "::",
            stringify!(h_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hostent>())).h_aliases as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(hostent),
            "::",
            stringify!(h_aliases)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hostent>())).h_addrtype as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(hostent),
            "::",
            stringify!(h_addrtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hostent>())).h_length as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(hostent),
            "::",
            stringify!(h_length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hostent>())).h_addr_list as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(hostent),
            "::",
            stringify!(h_addr_list)
        )
    );
}
extern "C" {
    pub fn sethostent(__stay_open: ::std::os::raw::c_int);
}
extern "C" {
    pub fn endhostent();
}
extern "C" {
    pub fn gethostent() -> *mut hostent;
}
extern "C" {
    pub fn gethostbyaddr(
        __addr: *const ::std::os::raw::c_void,
        __len: __socklen_t,
        __type: ::std::os::raw::c_int,
    ) -> *mut hostent;
}
extern "C" {
    pub fn gethostbyname(__name: *const ::std::os::raw::c_char) -> *mut hostent;
}
extern "C" {
    pub fn gethostbyname2(
        __name: *const ::std::os::raw::c_char,
        __af: ::std::os::raw::c_int,
    ) -> *mut hostent;
}
extern "C" {
    pub fn gethostent_r(
        __result_buf: *mut hostent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
        __result: *mut *mut hostent,
        __h_errnop: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gethostbyaddr_r(
        __addr: *const ::std::os::raw::c_void,
        __len: __socklen_t,
        __type: ::std::os::raw::c_int,
        __result_buf: *mut hostent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
        __result: *mut *mut hostent,
        __h_errnop: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gethostbyname_r(
        __name: *const ::std::os::raw::c_char,
        __result_buf: *mut hostent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
        __result: *mut *mut hostent,
        __h_errnop: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gethostbyname2_r(
        __name: *const ::std::os::raw::c_char,
        __af: ::std::os::raw::c_int,
        __result_buf: *mut hostent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
        __result: *mut *mut hostent,
        __h_errnop: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setnetent(__stay_open: ::std::os::raw::c_int);
}
extern "C" {
    pub fn endnetent();
}
extern "C" {
    pub fn getnetent() -> *mut netent;
}
extern "C" {
    pub fn getnetbyaddr(__net: u32, __type: ::std::os::raw::c_int) -> *mut netent;
}
extern "C" {
    pub fn getnetbyname(__name: *const ::std::os::raw::c_char) -> *mut netent;
}
extern "C" {
    pub fn getnetent_r(
        __result_buf: *mut netent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
        __result: *mut *mut netent,
        __h_errnop: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getnetbyaddr_r(
        __net: u32,
        __type: ::std::os::raw::c_int,
        __result_buf: *mut netent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
        __result: *mut *mut netent,
        __h_errnop: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getnetbyname_r(
        __name: *const ::std::os::raw::c_char,
        __result_buf: *mut netent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
        __result: *mut *mut netent,
        __h_errnop: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct servent {
    pub s_name: *mut ::std::os::raw::c_char,
    pub s_aliases: *mut *mut ::std::os::raw::c_char,
    pub s_port: ::std::os::raw::c_int,
    pub s_proto: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_servent() {
    assert_eq!(
        ::std::mem::size_of::<servent>(),
        32usize,
        concat!("Size of: ", stringify!(servent))
    );
    assert_eq!(
        ::std::mem::align_of::<servent>(),
        8usize,
        concat!("Alignment of ", stringify!(servent))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<servent>())).s_name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(servent),
            "::",
            stringify!(s_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<servent>())).s_aliases as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(servent),
            "::",
            stringify!(s_aliases)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<servent>())).s_port as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(servent),
            "::",
            stringify!(s_port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<servent>())).s_proto as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(servent),
            "::",
            stringify!(s_proto)
        )
    );
}
extern "C" {
    pub fn setservent(__stay_open: ::std::os::raw::c_int);
}
extern "C" {
    pub fn endservent();
}
extern "C" {
    pub fn getservent() -> *mut servent;
}
extern "C" {
    pub fn getservbyname(
        __name: *const ::std::os::raw::c_char,
        __proto: *const ::std::os::raw::c_char,
    ) -> *mut servent;
}
extern "C" {
    pub fn getservbyport(
        __port: ::std::os::raw::c_int,
        __proto: *const ::std::os::raw::c_char,
    ) -> *mut servent;
}
extern "C" {
    pub fn getservent_r(
        __result_buf: *mut servent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
        __result: *mut *mut servent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getservbyname_r(
        __name: *const ::std::os::raw::c_char,
        __proto: *const ::std::os::raw::c_char,
        __result_buf: *mut servent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
        __result: *mut *mut servent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getservbyport_r(
        __port: ::std::os::raw::c_int,
        __proto: *const ::std::os::raw::c_char,
        __result_buf: *mut servent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
        __result: *mut *mut servent,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct protoent {
    pub p_name: *mut ::std::os::raw::c_char,
    pub p_aliases: *mut *mut ::std::os::raw::c_char,
    pub p_proto: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_protoent() {
    assert_eq!(
        ::std::mem::size_of::<protoent>(),
        24usize,
        concat!("Size of: ", stringify!(protoent))
    );
    assert_eq!(
        ::std::mem::align_of::<protoent>(),
        8usize,
        concat!("Alignment of ", stringify!(protoent))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<protoent>())).p_name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(protoent),
            "::",
            stringify!(p_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<protoent>())).p_aliases as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(protoent),
            "::",
            stringify!(p_aliases)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<protoent>())).p_proto as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(protoent),
            "::",
            stringify!(p_proto)
        )
    );
}
extern "C" {
    pub fn setprotoent(__stay_open: ::std::os::raw::c_int);
}
extern "C" {
    pub fn endprotoent();
}
extern "C" {
    pub fn getprotoent() -> *mut protoent;
}
extern "C" {
    pub fn getprotobyname(__name: *const ::std::os::raw::c_char) -> *mut protoent;
}
extern "C" {
    pub fn getprotobynumber(__proto: ::std::os::raw::c_int) -> *mut protoent;
}
extern "C" {
    pub fn getprotoent_r(
        __result_buf: *mut protoent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
        __result: *mut *mut protoent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getprotobyname_r(
        __name: *const ::std::os::raw::c_char,
        __result_buf: *mut protoent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
        __result: *mut *mut protoent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getprotobynumber_r(
        __proto: ::std::os::raw::c_int,
        __result_buf: *mut protoent,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
        __result: *mut *mut protoent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setnetgrent(__netgroup: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn endnetgrent();
}
extern "C" {
    pub fn getnetgrent(
        __hostp: *mut *mut ::std::os::raw::c_char,
        __userp: *mut *mut ::std::os::raw::c_char,
        __domainp: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn innetgr(
        __netgroup: *const ::std::os::raw::c_char,
        __host: *const ::std::os::raw::c_char,
        __user: *const ::std::os::raw::c_char,
        __domain: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getnetgrent_r(
        __hostp: *mut *mut ::std::os::raw::c_char,
        __userp: *mut *mut ::std::os::raw::c_char,
        __domainp: *mut *mut ::std::os::raw::c_char,
        __buffer: *mut ::std::os::raw::c_char,
        __buflen: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rcmd(
        __ahost: *mut *mut ::std::os::raw::c_char,
        __rport: ::std::os::raw::c_ushort,
        __locuser: *const ::std::os::raw::c_char,
        __remuser: *const ::std::os::raw::c_char,
        __cmd: *const ::std::os::raw::c_char,
        __fd2p: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rcmd_af(
        __ahost: *mut *mut ::std::os::raw::c_char,
        __rport: ::std::os::raw::c_ushort,
        __locuser: *const ::std::os::raw::c_char,
        __remuser: *const ::std::os::raw::c_char,
        __cmd: *const ::std::os::raw::c_char,
        __fd2p: *mut ::std::os::raw::c_int,
        __af: sa_family_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rexec(
        __ahost: *mut *mut ::std::os::raw::c_char,
        __rport: ::std::os::raw::c_int,
        __name: *const ::std::os::raw::c_char,
        __pass: *const ::std::os::raw::c_char,
        __cmd: *const ::std::os::raw::c_char,
        __fd2p: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rexec_af(
        __ahost: *mut *mut ::std::os::raw::c_char,
        __rport: ::std::os::raw::c_int,
        __name: *const ::std::os::raw::c_char,
        __pass: *const ::std::os::raw::c_char,
        __cmd: *const ::std::os::raw::c_char,
        __fd2p: *mut ::std::os::raw::c_int,
        __af: sa_family_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ruserok(
        __rhost: *const ::std::os::raw::c_char,
        __suser: ::std::os::raw::c_int,
        __remuser: *const ::std::os::raw::c_char,
        __locuser: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ruserok_af(
        __rhost: *const ::std::os::raw::c_char,
        __suser: ::std::os::raw::c_int,
        __remuser: *const ::std::os::raw::c_char,
        __locuser: *const ::std::os::raw::c_char,
        __af: sa_family_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iruserok(
        __raddr: u32,
        __suser: ::std::os::raw::c_int,
        __remuser: *const ::std::os::raw::c_char,
        __locuser: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iruserok_af(
        __raddr: *const ::std::os::raw::c_void,
        __suser: ::std::os::raw::c_int,
        __remuser: *const ::std::os::raw::c_char,
        __locuser: *const ::std::os::raw::c_char,
        __af: sa_family_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rresvport(__alport: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rresvport_af(
        __alport: *mut ::std::os::raw::c_int,
        __af: sa_family_t,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct addrinfo {
    pub ai_flags: ::std::os::raw::c_int,
    pub ai_family: ::std::os::raw::c_int,
    pub ai_socktype: ::std::os::raw::c_int,
    pub ai_protocol: ::std::os::raw::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut ::std::os::raw::c_char,
    pub ai_next: *mut addrinfo,
}
#[test]
fn bindgen_test_layout_addrinfo() {
    assert_eq!(
        ::std::mem::size_of::<addrinfo>(),
        48usize,
        concat!("Size of: ", stringify!(addrinfo))
    );
    assert_eq!(
        ::std::mem::align_of::<addrinfo>(),
        8usize,
        concat!("Alignment of ", stringify!(addrinfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<addrinfo>())).ai_flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(addrinfo),
            "::",
            stringify!(ai_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<addrinfo>())).ai_family as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(addrinfo),
            "::",
            stringify!(ai_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<addrinfo>())).ai_socktype as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(addrinfo),
            "::",
            stringify!(ai_socktype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<addrinfo>())).ai_protocol as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(addrinfo),
            "::",
            stringify!(ai_protocol)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<addrinfo>())).ai_addrlen as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(addrinfo),
            "::",
            stringify!(ai_addrlen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<addrinfo>())).ai_addr as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(addrinfo),
            "::",
            stringify!(ai_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<addrinfo>())).ai_canonname as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(addrinfo),
            "::",
            stringify!(ai_canonname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<addrinfo>())).ai_next as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(addrinfo),
            "::",
            stringify!(ai_next)
        )
    );
}
extern "C" {
    pub fn getaddrinfo(
        __name: *const ::std::os::raw::c_char,
        __service: *const ::std::os::raw::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn freeaddrinfo(__ai: *mut addrinfo);
}
extern "C" {
    pub fn gai_strerror(__ecode: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn getnameinfo(
        __sa: *const sockaddr,
        __salen: socklen_t,
        __host: *mut ::std::os::raw::c_char,
        __hostlen: socklen_t,
        __serv: *mut ::std::os::raw::c_char,
        __servlen: socklen_t,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[test]
fn bindgen_test_layout_ssh_counter_struct() {
    assert_eq!(
        ::std::mem::size_of::<ssh_counter_struct>(),
        32usize,
        concat!("Size of: ", stringify!(ssh_counter_struct))
    );
    assert_eq!(
        ::std::mem::align_of::<ssh_counter_struct>(),
        8usize,
        concat!("Alignment of ", stringify!(ssh_counter_struct))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ssh_counter_struct>())).in_bytes as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ssh_counter_struct),
            "::",
            stringify!(in_bytes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ssh_counter_struct>())).out_bytes as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ssh_counter_struct),
            "::",
            stringify!(out_bytes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ssh_counter_struct>())).in_packets as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ssh_counter_struct),
            "::",
            stringify!(in_packets)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ssh_counter_struct>())).out_packets as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ssh_counter_struct),
            "::",
            stringify!(out_packets)
        )
    );
}
#[test]
fn bindgen_test_layout_ssh_knownhosts_entry() {
    assert_eq!(
        ::std::mem::size_of::<ssh_knownhosts_entry>(),
        32usize,
        concat!("Size of: ", stringify!(ssh_knownhosts_entry))
    );
    assert_eq!(
        ::std::mem::align_of::<ssh_knownhosts_entry>(),
        8usize,
        concat!("Alignment of ", stringify!(ssh_knownhosts_entry))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ssh_knownhosts_entry>())).hostname as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ssh_knownhosts_entry),
            "::",
            stringify!(hostname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ssh_knownhosts_entry>())).unparsed as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ssh_knownhosts_entry),
            "::",
            stringify!(unparsed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ssh_knownhosts_entry>())).publickey as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ssh_knownhosts_entry),
            "::",
            stringify!(publickey)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ssh_knownhosts_entry>())).comment as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ssh_knownhosts_entry),
            "::",
            stringify!(comment)
        )
    );
}
#[doc = " No logging at all"]
pub const SSH_LOG_NOLOG: ::std::os::raw::c_uint = 0;
#[doc = " Only warnings"]
pub const SSH_LOG_WARNING: ::std::os::raw::c_uint = 1;
#[doc = " High level protocol information"]
pub const SSH_LOG_PROTOCOL: ::std::os::raw::c_uint = 2;
#[doc = " Lower level protocol infomations, packet level"]
pub const SSH_LOG_PACKET: ::std::os::raw::c_uint = 3;
#[doc = " Every function path"]
pub const SSH_LOG_FUNCTIONS: ::std::os::raw::c_uint = 4;
#[doc = " @addtogroup libssh_log"]
#[doc = ""]
#[doc = " @{"]
pub type _bindgen_ty_10 = ::std::os::raw::c_uint;
pub type _bindgen_ty_11 = ::std::os::raw::c_uint;
extern "C" {
    pub fn buffer_free(buffer: ssh_buffer);
}
extern "C" {
    pub fn buffer_get(buffer: ssh_buffer) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn buffer_get_len(buffer: ssh_buffer) -> u32;
}
extern "C" {
    pub fn buffer_new() -> ssh_buffer;
}
extern "C" {
    pub fn channel_accept_x11(
        channel: ssh_channel,
        timeout_ms: ::std::os::raw::c_int,
    ) -> ssh_channel;
}
extern "C" {
    pub fn channel_change_pty_size(
        channel: ssh_channel,
        cols: ::std::os::raw::c_int,
        rows: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_forward_accept(
        session: ssh_session,
        timeout_ms: ::std::os::raw::c_int,
    ) -> ssh_channel;
}
extern "C" {
    pub fn channel_close(channel: ssh_channel) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_forward_cancel(
        session: ssh_session,
        address: *const ::std::os::raw::c_char,
        port: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_forward_listen(
        session: ssh_session,
        address: *const ::std::os::raw::c_char,
        port: ::std::os::raw::c_int,
        bound_port: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_free(channel: ssh_channel);
}
extern "C" {
    pub fn channel_get_exit_status(channel: ssh_channel) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_get_session(channel: ssh_channel) -> ssh_session;
}
extern "C" {
    pub fn channel_is_closed(channel: ssh_channel) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_is_eof(channel: ssh_channel) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_is_open(channel: ssh_channel) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_new(session: ssh_session) -> ssh_channel;
}
extern "C" {
    pub fn channel_open_forward(
        channel: ssh_channel,
        remotehost: *const ::std::os::raw::c_char,
        remoteport: ::std::os::raw::c_int,
        sourcehost: *const ::std::os::raw::c_char,
        localport: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_open_session(channel: ssh_channel) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_poll(
        channel: ssh_channel,
        is_stderr: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_read(
        channel: ssh_channel,
        dest: *mut ::std::os::raw::c_void,
        count: u32,
        is_stderr: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_read_buffer(
        channel: ssh_channel,
        buffer: ssh_buffer,
        count: u32,
        is_stderr: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_read_nonblocking(
        channel: ssh_channel,
        dest: *mut ::std::os::raw::c_void,
        count: u32,
        is_stderr: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_request_env(
        channel: ssh_channel,
        name: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_request_exec(
        channel: ssh_channel,
        cmd: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_request_pty(channel: ssh_channel) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_request_pty_size(
        channel: ssh_channel,
        term: *const ::std::os::raw::c_char,
        cols: ::std::os::raw::c_int,
        rows: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_request_shell(channel: ssh_channel) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_request_send_signal(
        channel: ssh_channel,
        signum: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_request_sftp(channel: ssh_channel) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_request_subsystem(
        channel: ssh_channel,
        subsystem: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_request_x11(
        channel: ssh_channel,
        single_connection: ::std::os::raw::c_int,
        protocol: *const ::std::os::raw::c_char,
        cookie: *const ::std::os::raw::c_char,
        screen_number: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_send_eof(channel: ssh_channel) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_select(
        readchans: *mut ssh_channel,
        writechans: *mut ssh_channel,
        exceptchans: *mut ssh_channel,
        timeout: *mut timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn channel_set_blocking(channel: ssh_channel, blocking: ::std::os::raw::c_int);
}
extern "C" {
    pub fn channel_write(
        channel: ssh_channel,
        data: *const ::std::os::raw::c_void,
        len: u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn privatekey_free(prv: ssh_private_key);
}
extern "C" {
    pub fn privatekey_from_file(
        session: ssh_session,
        filename: *const ::std::os::raw::c_char,
        type_: ::std::os::raw::c_int,
        passphrase: *const ::std::os::raw::c_char,
    ) -> ssh_private_key;
}
extern "C" {
    pub fn publickey_free(key: ssh_public_key);
}
extern "C" {
    pub fn ssh_publickey_to_file(
        session: ssh_session,
        file: *const ::std::os::raw::c_char,
        pubkey: ssh_string,
        type_: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn publickey_from_file(
        session: ssh_session,
        filename: *const ::std::os::raw::c_char,
        type_: *mut ::std::os::raw::c_int,
    ) -> ssh_string;
}
extern "C" {
    pub fn publickey_from_privatekey(prv: ssh_private_key) -> ssh_public_key;
}
extern "C" {
    pub fn publickey_to_string(key: ssh_public_key) -> ssh_string;
}
extern "C" {
    pub fn ssh_try_publickey_from_file(
        session: ssh_session,
        keyfile: *const ::std::os::raw::c_char,
        publickey: *mut ssh_string,
        type_: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ssh_privatekey_type(privatekey: ssh_private_key) -> ssh_keytypes_e;
}
extern "C" {
    pub fn ssh_get_pubkey(session: ssh_session) -> ssh_string;
}
extern "C" {
    pub fn ssh_message_retrieve(session: ssh_session, packettype: u32) -> ssh_message;
}
extern "C" {
    pub fn ssh_message_auth_publickey(msg: ssh_message) -> ssh_public_key;
}
extern "C" {
    pub fn string_burn(str_: ssh_string);
}
extern "C" {
    pub fn string_copy(str_: ssh_string) -> ssh_string;
}
extern "C" {
    pub fn string_data(str_: ssh_string) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn string_fill(
        str_: ssh_string,
        data: *const ::std::os::raw::c_void,
        len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn string_free(str_: ssh_string);
}
extern "C" {
    pub fn string_from_char(what: *const ::std::os::raw::c_char) -> ssh_string;
}
extern "C" {
    pub fn string_len(str_: ssh_string) -> size_t;
}
extern "C" {
    pub fn string_new(size: size_t) -> ssh_string;
}
extern "C" {
    pub fn string_to_char(str_: ssh_string) -> *mut ::std::os::raw::c_char;
}
