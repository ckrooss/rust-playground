use crate::flash_device::FlashDevice;
use anyhow::{bail, Result};

pub struct NandDevice {
    data: Vec<u8>,
}

impl NandDevice {
    pub fn new(size: usize) -> NandDevice {
        NandDevice {
            data: vec![0xff; size],
        }
    }
}

impl FlashDevice for NandDevice {
    /// Read a single u8 block from the NAND device
    fn read(&self, offset: usize, size: usize) -> Result<Vec<u8>> {
        if size == 0 {
            bail!("Read block size is 0");
        }

        if offset + size > self.data.len() {
            bail!("Read out of bounds");
        }

        Ok(self.data[offset..offset + size].to_vec())
    }

    fn write(&mut self, offset: usize, data: &[u8]) -> Result<()> {
        if offset + data.len() > self.data.len() {
            bail!("Cannot write past device boundary");
        }

        for (i, val) in data.iter().enumerate() {
            if self.data[offset + i] != 0xff {
                bail!("Write to non-erased block");
            }
            self.data[offset + i] &= val;
        }

        Ok(())
    }

    fn erase(&mut self, offset: usize, size: usize) -> Result<()> {
        for i in offset..offset + size {
            self.data[i] = 0xff;
        }

        Ok(())
    }

    fn erase_device(&mut self) -> Result<()> {
        self.data.fill(0xff);
        Ok(())
    }
}
