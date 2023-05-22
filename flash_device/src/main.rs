use rs_flash::flash_device::FlashDevice;
use rs_flash::nand_device::NandDevice;

fn main() {
    let mut flash = NandDevice::new(16 * 1024);
    match flash.write(1, &[0x55]) {
        Ok(()) => {}
        Err(e) => {
            println!("Error writing data {:?}", e);
        }
    }
}
