//! Native FFI bindings to libfuse.
//!
//! This is a small set of bindings that are required to mount/unmount FUSE filesystems and
//! open/close a fd to the FUSE kernel driver.

#![warn(missing_debug_implementations)]
#![allow(missing_docs)]

use log::error;
use std::fs::{File, OpenOptions};
use std::io::{self, Error, ErrorKind};
use std::os::unix::io::AsRawFd;
use std::sync::Arc;

#[derive(Debug)]
pub struct Cuse {
    file: Arc<File>,
}

impl Cuse {
    pub fn new() -> io::Result<Self> {
        let file = cuse_open()?;
        Ok(Self {
            file: Arc::new(file),
        })
    }

    pub fn file(&self) -> Arc<File> {
        self.file.clone()
    }
}

impl Drop for Cuse {
    fn drop(&mut self) {}
}

#[allow(dead_code)]
fn cuse_open() -> Result<File, Error> {
    let cuse_device_name = "/dev/cuse";

    let file = match OpenOptions::new()
        .read(true)
        .write(true)
        .open(cuse_device_name)
    {
        Ok(file) => file,
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                error!("{} not found. Try 'modprobe fuse'", cuse_device_name);
            }
            return Err(error);
        }
    };
    assert!(
        file.as_raw_fd() > 2,
        "Conflict with stdin/stdout/stderr. fd={}",
        file.as_raw_fd()
    );

    Ok(file)
}
