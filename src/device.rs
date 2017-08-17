use errors::*;
use libc::*;
use std::ffi;
use libc;
use errno;

use BLOCK_SIZE;

pub struct Device {
    n_blocks: usize,
    fd: i32,
}

impl Device {
    pub fn open(p: &str, n_bytes: i64) -> Result<Device> {
        let s = ffi::CString::new(p).chain_err(
            || "could not convert path to CSring",
        )?;
        let fd = unsafe {
            open(
                s.as_ref().as_ptr(),
                O_RDWR | O_CREAT,
                S_IRUSR as c_uint | S_IWUSR as c_uint,
            )
        };

        errno!(fd).map(|fd| {
            Device {
                fd: fd,
                n_blocks: (n_bytes * BLOCK_SIZE) as usize,
            }
        })
    }

    pub fn write_block(&self, block: i64, buf: &[u8]) -> Result<i64> {
        let offset = block * BLOCK_SIZE;
        let result =
            unsafe { pwrite(self.fd, buf.as_ptr() as *const c_void, buf.len(), offset) as i64 };
        errno!(result)
    }

    pub fn read_block(&self, block: i64, buf: &mut [u8]) -> Result<i64> {
        if buf.len() < block as usize {
            bail!("block size exceeds buf len");
        }
        let offset = block * BLOCK_SIZE;
        let result = unsafe {
            pread(
                self.fd,
                buf as *mut [u8] as *mut c_void,
                BLOCK_SIZE as usize,
                offset,
            ) as i64
        };
        errno!(result)
    }

    pub fn sync(&self) {
        unsafe { libc::sync() }
    }
}