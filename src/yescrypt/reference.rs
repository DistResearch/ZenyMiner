use std::os::raw::c_void;
use std::sync::Mutex;

fn yescrypt_bitzeny(password: u8, password_len: u32, salt: u8,
                    salt_len: u32, buf: u8, buf_len: u32) -> i32 {
    let initialized = Mutex::new(0);
    let mut shared: Mutex<YescryptShared>;
    let mut local: Mutex<YescryptLocal>;
    let mut retval: i32;
    if !initialized {
        //todo
    }
}