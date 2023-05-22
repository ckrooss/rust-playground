use crate::flash_device::FlashDevice;
use anyhow::{bail, Result};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::prelude::FileExt;

pub struct FileDevice {
    file: File,
    size: usize,
}

impl FileDevice {
    pub fn new(size: usize, path: &str) -> Result<FileDevice> {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        let buf = vec![0xff; size];
        file.write_all(&buf)?;
        Ok(FileDevice { file, size })
    }
}

impl FlashDevice for FileDevice {
    fn read(&self, offset: usize, size: usize) -> Result<Vec<u8>> {
        if size == 0 {
            bail!("Invalid read-size 0");
        }
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(size, 0);
        self.file.read_exact_at(buf.as_mut_slice(), offset as u64)?;
        Ok(buf)
    }

    fn write(&mut self, offset: usize, data: &[u8]) -> Result<()> {
        if offset + data.len() > self.size {
            bail!("Request to write data past device boundary");
        }
        self.file.write_all_at(data, offset as u64)?;
        Ok(())
    }

    fn erase(&mut self, offset: usize, size: usize) -> Result<()> {
        if offset + size > self.size {
            bail!("Request to erase past device boundary");
        }
        let buf = vec![0xff; size];
        self.file.write_all_at(&buf, offset as u64)?;
        Ok(())
    }

    fn erase_device(&mut self) -> Result<()> {
        let buf: Vec<u8> = vec![0xff; self.size];
        self.file.write_all_at(buf.as_slice(), 0)?;
        Ok(())
    }
}
