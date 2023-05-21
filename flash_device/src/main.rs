use flash_device::device::FlashDevice;
use flash_device::device::NandDevice;

fn main() {
    let mut flash = NandDevice::new(16 * 1024);
    flash.write(1, 0x55);
}
