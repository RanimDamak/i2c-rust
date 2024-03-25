//! Scull module in Rust

use core::result::Result::Ok;
use kernel::io_buffer::{IoBufferReader, IoBufferWriter};
use kernel::prelude::*;
use kernel::sync::{Arc, ArcBorrow};
use kernel::{file, miscdev};

module! {
    type: Scull,
    name: "scull",
    license: "GPL",
}
struct Device {
    number: usize,
    contents: Vec<u8>,
}

struct Scull {
    _dev: Pin<Box<miscdev::Registration<Scull>>>,
}

#[vtable]
impl file::Operations for Scull {
    type OpenData = Arc<Device>;
    type Data = Arc<Device>;

    fn open(context: &Self::OpenData,
         _file: &file::File) -> Result<Self::Data> {
        pr_info!("File from device {} was opened\n", context.number);
        Ok(context.clone())
    }

    fn read(
        _this: ArcBorrow<'_, Device>,
        _file: &file::File,
        _writer: &mut impl IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("File was read\n");
        Ok(0)
    }

    /// Handles ioctls defined with the `_IOW` macro, that is, with an input buffer provided as
    /// argument.
    fn write(
        _this: ArcBorrow<'_, Device>,
        _file: &file::File,
        reader: &mut impl IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("File was written\n");
        Ok(reader.len())
    }
}

impl kernel::Module for Scull {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello from new Rust model\n");

        let dev = Arc::try_new(Device {
            number: 0,
            contents: Vec::new(),
        })?;
        let reg = miscdev::Registration::new_pinned(fmt!("scull"), dev)?;
        Ok(Scull { _dev: reg })
    }
}
