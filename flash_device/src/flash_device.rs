use anyhow::Result;
pub trait FlashDevice {
    fn read(&self, offset: usize, size: usize) -> Result<Vec<u8>>;
    fn write(&mut self, offset: usize, data: &[u8]) -> Result<()>;
    fn erase(&mut self, offset: usize, size: usize) -> Result<()>;
    fn erase_device(&mut self) -> Result<()>;
}
