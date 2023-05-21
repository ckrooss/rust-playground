pub trait FlashDevice {
    fn read(&self, offset: usize) -> u8;
    fn read_block(&self, offset: usize, size: usize) -> &[u8];
    fn write(&mut self, offset: usize, data: u8);
    fn write_block(&mut self, offset: usize, data: &[u8]);
    fn erase(&mut self, offset: usize);
    fn erase_block(&mut self, offset: usize, size: usize);
    fn erase_device(&mut self);
}
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
    /// use flash_device::device::{FlashDevice, NandDevice};
    /// let dev = NandDevice::new(1024);
    /// let data = dev.read(0);
    /// assert_eq!(data, 0xff);
    /// ```
    fn read(&self, offset: usize) -> u8 {
        if offset >= self.data.len() {
            panic!("Read out of bounds");
        }
        self.data[offset]
    }

    fn read_block(&self, offset: usize, size: usize) -> &[u8] {
        if size == 0 {
            panic!("Read block size is 0");
        }

        if offset + size > self.data.len() {
            panic!("Read out of bounds");
        }

        &self.data[offset..offset + size]
    }

    fn write(&mut self, offset: usize, data: u8) {
        if offset >= self.data.len() {
            panic!("Write out of bounds");
        }

        if self.data[offset] != 0xff {
            panic!("Write to non-erased block");
        }
        self.data[offset] &= data;
    }

    fn write_block(&mut self, offset: usize, data: &[u8]) {
        //self.data[offset..offset + data.len()].copy_from_slice(data);
        for i in 0..data.len() {
            if self.data[offset + i] != 0xff {
                panic!("Write to non-erased block");
            }
            self.data[offset + i] &= data[i];
        }
    }

    fn erase(&mut self, offset: usize) {
        self.data[offset] = 0xff;
    }

    fn erase_block(&mut self, offset: usize, size: usize) {
        for i in offset..offset + size {
            self.data[i] = 0xff;
        }
    }

    fn erase_device(&mut self) {
        self.data.fill(0xff);
    }
}
