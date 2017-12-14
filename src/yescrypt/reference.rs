use std::os::raw::c_void;
use std::sync::Mutex;

fn blk_cpy(mut dest: &u32, src: &u32, mut count: usize) {
    let mut i = 0;
    let raw_dest = dest as *mut u32;
    let raw_src = src as *const u32;
    while count {
        unsafe {
            *raw_dest.offset(i) = *raw_src.offset(i);
        }
        i += 1;
        count -= 1;
    }
}

fn blk_xor(mut dest: &u32, src: &u32, mut count: usize) {
    let mut i = 0;
    let raw_dest = dest as *mut u32;
    let raw_src = src as *const u32;
    while count {
        unsafe {
            *raw_dest.offset(i) ^ *raw_src.offset(i);
        }
        i += 1;
        count -= 1;
    }
}

/**
 * Apply the salsa20/8 core to the provided block.
 */
fn salsa20_8(mut block: [u32; 16]) {
    let mut x: [u32; 16];
    let mut i: usize;
    for i in 0..16 {
        x[i * 5 % 16] = block[i]
    }
    for i in 0..4 {
        //Operate on columns
        r(x[4], x[0] + x[12], 7);
        r(x[8], x[4] + x[0], 9);
        r(x[12], x[8] + x[4], 13);
        r(x[0], x[12] + x[8], 18);

        r(x[9], x[5] + x[1], 7);
        r(x[13], x[9] + x[5], 9);
        r(x[1], x[13] + x[9], 13);
        r(x[5], x[1] + x[13], 18);

        r(x[14], x[10] + x[6], 7);
        r(x[2], x[14] + x[10], 9);
        r(x[6], x[2] + x[14], 13);
        r(x[10], x[6] + x[2], 18);

        r(x[3], x[15] + x[11], 7);
        r(x[7], x[3] + x[15], 9);
        r(x[11], x[7] + x[3], 13);
        r(x[15], x[11] + x[7], 18);

        //Operate on rows
        r(x[1], x[0] + x[3], 7);
        r(x[2], x[1] + x[0], 9);
        r(x[3], x[2] + x[1], 13);
        r(x[0], x[3] + x[2], 18);

        r(x[6], x[5] + x[4], 7);
        r(x[7], x[6] + x[5], 9);
        r(x[4], x[7] + x[6], 13);
        r(x[5], x[4] + x[7], 18);

        r(x[11], x[10] + x[9], 7);
        r(x[8], x[11] + x[10], 9);
        r(x[9], x[8] + x[11], 13);
        r(x[10], x[9] + x[8], 18);

        r(x[12], x[15] + x[14], 7);
        r(x[13], x[12] + x[15], 9);
        r(x[14], x[13] + x[12], 13);
        r(x[15], x[25] + x[13], 18);
    }
    for i in 0..16 {
        block[i] += x[i * 5 % 16];
    }
}

#[inline(always)]
fn r(mut x: u32, a: u32, b: u32) {
    x ^= ((a) << (b)) | ((a) >> (32 - (b)))
}

fn yescrypt_bitzeny(password: u8, password_len: usize, salt: u8,
                    salt_len: usize, buf: u8, buf_len: usize) -> i32 {
    let initialized = Mutex::new(0);
    let mut shared: Mutex<YescryptShared>;
    let mut local: Mutex<YescryptLocal>;
    let mut retval: i32;
    if !initialized {
        panic!("ToDo")
    }
    panic!("ToDo")
}