pub trait FlashDevice {
    fn read(&self, offset: usize, size: usize) -> Vec<u8>;
    fn write(&mut self, offset: usize, data: &[u8]);
    fn erase(&mut self, offset: usize, size: usize);
    fn erase_device(&mut self);
}
