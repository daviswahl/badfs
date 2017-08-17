// extern
use serde::{Deserialize, Deserializer, Serialize};
use rmp_serde::{Serializer, Deserializer as RmpDeser};
use typed_arena::Arena;

// std
use std::*;
use std::sync::{Arc, RwLock};
use std::result::Result as StdResult;

// crate
use errors::*;
use device::Device;
use BLOCK_SIZE;

pub struct FileSystem {
    device: Device,
    metadata: Arc<RwLock<Metadata>>,
    descriptors: Arc<RwLock<Vec<* const Inode>>>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Indirect {
    Inode(* mut Inode),
    Block(i64),
}

#[derive(Serialize, Deserialize, Debug)]
enum Inode {
    File(String, Vec<Indirect>),
    Directory(String, Vec<* mut Inode>),
    Root(Vec<* mut Inode>),
}

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    freelist: Vec<u8>,
    root: * mut Inode,
}

impl Inode {
    pub fn os_str(&self) -> &ffi::OsStr {
        use self::Inode::*;
        match *self {
            File(ref name, _) => ffi::OsStr::new(name),
            Directory(ref name, _) => ffi::OsStr::new(name),
            Root(_) => ffi::OsStr::new("/"),
        }
    }

    pub fn children(& mut self) -> Option<&mut Vec<& mut Inode>> {
        use self::Inode::*;
        match *self {
            File(..) => None,
            Directory(_, ref mut children) => Some(children),
            Root(ref mut children) => Some(children)
        }
    }

        pub fn add_child(& mut self, node: &mut Inode) -> Result<()> {
        use self::Inode::*;
        match *self {
            File(..) => bail!("tried to add child to file inode"),
            Directory(ref name, ref mut children) => unimplemented!(),

            Root(ref mut children) => {
                Ok(children.push(node))
            }
        }
    }
}

impl FileSystem {
    pub fn mount(device: Device) -> Result<FileSystem> {
        let mut buf: [u8; BLOCK_SIZE as usize] = [0; BLOCK_SIZE as usize];

        let bytes_read = device.read_block(0, &mut buf).chain_err(
            || "corrupt header",
        )?;

        let metadata = if bytes_read == 0 {
            let metadata = Metadata {
                freelist: vec![],
                root: vec![]
            };
            metadata
                .serialize(&mut Serializer::new(&mut buf.as_mut()))
                .chain_err(|| "corrupt header")?;
            device.write_block(0, &mut buf).chain_err(
                || "block write err",
            )?;
            metadata
        } else {
            let mut de = RmpDeser::new(&buf[..]);
            Deserialize::deserialize(&mut de).chain_err(
                || "corrupt header",
            )?
        };
        Ok(FileSystem {
            device: device,
            metadata: Arc::new(RwLock::new(metadata)),
            descriptors: Arc::new(RwLock::new(vec![])),
        })
    }

    pub fn open<'b>(&self, path: &'b path::Path) -> Result<i32> {
        match self.metadata.write() {
            Ok(metadata) => {
                let parent = find_parent_of(&mut metadata.root, path)?;
                let name = path.file_name()
                    .ok_or("bad filename")?
                    .to_owned()
                    .into_string()
                    .unwrap();
                let node = self.inodes.alloc(Inode::File(name, vec![]));
                parent.add_child(node);
                match self.descriptors.write() {
                    Ok(mut descriptors) => {
                        descriptors.push(node as *const Inode);
                        Ok((descriptors.len() as i32 - 1).clone())
                    }
                    Err(_) => Err("foo".into())
                }
            }
            Err(e) => Err("foo".into())
        }
    }

    pub fn write(&mut self, fd: i32, buf: &[u8]) -> Result<()> {
        unimplemented!();
    }

    pub fn read(&self, fd: i32, buf: &mut [u8]) -> Result<()> {
        unimplemented!()
    }
}

fn find_parent_of<'a, 'b>(root: &'a mut Vec<& mut Inode>, path: &'b path::Path) -> Result<&'a mut Inode> {
    unimplemented!()
}
