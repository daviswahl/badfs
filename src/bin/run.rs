extern crate fs;
fn main() {
    let dev = fs::Device::open("bar.txt", 1024).unwrap();

    let data = b"asdffdsdf";
    let written = dev.write_block(0, data);
}
