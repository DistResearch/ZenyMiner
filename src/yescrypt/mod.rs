pub mod reference;
mod optimize;
mod simd;

const YESCRYPT_N: i32 = 2048;
const YESCRYPT_R: i32 = 8;
const YESCRYPT_P: i32 = 1;
const YESCRYPT_T: i32 = 0;

struct YescryptLocal {
    base: * c_void,
    aligned: * c_void,
    base_size: usize,
    aligned_size: usize,
}

struct YescryptShared {
    base: * c_void,
    aligned: * c_void,
    base_size: usize,
    aligned_size: usize,
    mask: u32,
}

enum YescryptInitSharedFlags {
    YescryptSharedDefaults = 0,
    YescryptSharedPreAllocated = 0x100,
}

enum YescryptFlags {
    //public
    YescryptWarm = 0,
    YescryptRW = 1,
    YescryptParallelSMix = 2,
    YescryptPwxForm = 4,
    //private
    __YescryptInitShared1 = 0x10000,
    __YescryptInitShared2 = 0x20000,
    __YescryptInitShared = 0x30000,
}

static YESCRYPT_KNOWN_FLAGS: u32 = YescryptFlags::YescryptRW | YescryptFlags::YescryptParallelSMix
    | YescryptFlags::YescryptPwxForm | YescryptFlags::__YescryptInitShared;