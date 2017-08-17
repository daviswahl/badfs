#![recursion_limit = "1024"]

extern crate libc;
extern crate errno;
extern crate typed_arena;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate rmp_serde;

// std
use std::*;
// local





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

#[macro_use]
mod macros;
mod device;
pub mod file_system;

const BLOCK_SIZE: i64 = 4096;

#[cfg(test)]
mod tests {
    use super::*;

    use device::Device;
    use file_system::FileSystem;
    use std::io::Read;
    #[test]
    fn test_mount() {
        fs::remove_file(path::Path::new("testing/foo.txt")).unwrap();
        let dev = Device::open("testing/foo.txt", 1024 * 100).unwrap();
        let mut fsys = FileSystem::mount(dev).unwrap();
        let fd;
        {
            fd = fsys.open(path::Path::new("/foo")).unwrap().clone();
        }

        let mut input_buf = vec![];
        {
            fs::File::open(path::Path::new("testing/image.gif"))
                .unwrap()
                .read(&mut input_buf).unwrap();
        }
        {
            fsys.write(fd, &input_buf).unwrap();
        }

        let mut output_buf = vec![];
        fsys.read(fd, &mut output_buf);
        assert_eq!(input_buf, output_buf);
    }
    #[test]
    fn it_works() {
        let dev = Device::open("testing/bar.txt", 1024 * 100).unwrap();

        let data = b"asdffdsdf";
        let written = dev.write_block(0, data);

        dev.sync();
        let mut buf: [u8; BLOCK_SIZE as usize] = [0; BLOCK_SIZE as usize];
        dev.read_block(0, &mut buf);

        //assert_eq!(written, read);
        assert_eq!(buf.to_owned().split_at(written.unwrap() as usize).0, data);
    }
}
