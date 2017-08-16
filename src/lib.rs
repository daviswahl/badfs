#![recursion_limit = "1024"]

#[macro_use]
extern crate libc;
extern crate errno;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate rmp_serde;

use libc::*;
use std::ffi;
use std::sync::RwLock;
use std::*;
use std::collections::HashMap;
use std::io::Write;

use serde::Serialize;
use serde::Deserialize;


// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

// This only gives access within this module. Make this `pub use errors::*;`
// instead if the types must be accessible from other modules (e.g., within
// a `links` section).
use errors::*;


const BLOCK_SIZE: i64 = 4096;
pub struct Device {
    n_blocks: usize,
    fd: i32,
}

macro_rules! errno {
    ($s:expr) => { if $s == -1 { errno!() } else { Ok($s) }};
    () => { Err(format!("errno: {}", errno::errno()).into()) }
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

#[derive(Serialize, Deserialize, Debug)]
enum Indirect {
    Inode(Inode),
    Block(i64),
}

#[derive(Serialize, Deserialize, Debug)]
enum Inode {
    File { blocklist: Vec<Indirect> },
    Directory(HashMap<String, Inode>),
}

struct FileSystem {
    device: Device,
    metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    freelist: Vec<u8>,
    inodes: Vec<Inode>,
}

impl FileSystem {
    fn mount(device: Device) -> Result<FileSystem> {
        let mut buf: [u8; BLOCK_SIZE as usize] = [0; BLOCK_SIZE as usize];

        let bytes_read = device.read_block(0, &mut buf).chain_err(
            || "corrupt header",
        )?;
        let metadata = if bytes_read == 0 {
            let metadata = Metadata {
                freelist: vec![],
                inodes: vec![],
            };
            metadata
                .serialize(&mut rmp_serde::Serializer::new(&mut buf.as_mut()))
                .chain_err(|| "corrupt header")?;
            device.write_block(0, &mut buf);
            metadata
        } else {
            let mut de = rmp_serde::Deserializer::new(&buf[..]);
            Deserialize::deserialize(&mut de).chain_err(
                || "corrupt header",
            )?
        };
        Ok(FileSystem {
            device: device,
            metadata: metadata,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mount() {
        let dev = Device::open("foo.txt", 1024 * 100).unwrap();
        let fsys = FileSystem::mount(dev).unwrap();
    }
    #[test]
    fn it_works() {
        let dev = Device::open("bar.txt", 1024 * 100).unwrap();

        let data = b"asdffdsdf";
        let written = dev.write_block(0, data);

        dev.sync();
        let mut buf: [u8; BLOCK_SIZE as usize] = [0; BLOCK_SIZE as usize];
        let read = dev.read_block(0, &mut buf);

        //assert_eq!(written, read);
        assert_eq!(buf.to_owned().split_at(written.unwrap() as usize).0, data);
    }
}
