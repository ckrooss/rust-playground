use rs_flash::flash_device::FlashDevice;
use rs_flash::nand_device::NandDevice;

fn main() {
    let mut flash = NandDevice::new(16 * 1024);
    flash.write(1, &[0x55]);
}
