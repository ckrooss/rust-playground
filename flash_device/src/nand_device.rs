use crate::flash_device::FlashDevice;

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
    /// ```
    /// /// Read a single u8 block from the NAND device
    /// use rs_flash::nand_device::NandDevice;
    /// use rs_flash::flash_device::FlashDevice;
    /// let dev = NandDevice::new(1024);
    /// let data = dev.read(0, 1);
    /// assert_eq!(data, &[0xff]);
    /// ```
    fn read(&self, offset: usize, size: usize) -> Vec<u8> {
        if size == 0 {
            panic!("Read block size is 0");
        }

        if offset + size > self.data.len() {
            panic!("Read out of bounds");
        }

        self.data[offset..offset + size].to_vec()
    }

    fn write(&mut self, offset: usize, data: &[u8]) {
        //self.data[offset..offset + data.len()].copy_from_slice(data);
        for i in 0..data.len() {
            if self.data[offset + i] != 0xff {
                panic!("Write to non-erased block");
            }
            self.data[offset + i] &= data[i];
        }
    }

    fn erase(&mut self, offset: usize, size: usize) {
        for i in offset..offset + size {
            self.data[i] = 0xff;
        }
    }

    fn erase_device(&mut self) {
        self.data.fill(0xff);
    }
}
