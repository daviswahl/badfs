extern crate libc;
use std::ffi;
use std::path::PathBuf;

pub fn open_disk(p: PathBuf, n_bytes: usize) -> usize {
    unimplemented!()
}

pub fn write_block(disk: usize, block: usize, buf: &[u8]) -> usize {
    unimplemented!()
}

pub fn read_block(disk: usize, block: usize, buf: &mut [u8]) -> usize {
    unimplemented!()
}

pub fn sync() {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let dev = open_disk("foo.txt".into(), 1024);

        let data = b"asdffdsdf";
        let written = write_block(dev, 0, data);

        let mut buf = vec![];
        let read =  read_block(dev, 0, buf.as_mut());

        assert_eq!(written, read);
        assert_eq!(buf, data);


    }
}
