pub mod device;
use device::{FlashDevice, NandDevice};

#[test]
fn test_read() {
    let dev = NandDevice::new(1024);
    let data = dev.read(0);
    assert_eq!(data, 0xff);
}

#[test]
fn test_read_edges() {
    let dev = NandDevice::new(8);
    dev.read(0);
    dev.read(7);

    dev.read_block(0, 8);
    dev.read_block(7, 1);
}

#[test]
fn test_write() {
    let mut dev = NandDevice::new(1024);
    dev.write(0, 0);
}

#[test]
fn test_write_edges() {
    let mut dev = NandDevice::new(8);

    dev.write(0, 0xee);
    dev.write(7, 0xee);

    dev.erase_device();
    dev.write_block(0, &[0xee; 8]);

    dev.erase_device();
    dev.write_block(7, &[0xee; 1]);
}

#[test]
fn test_write_read() {
    let mut dev = NandDevice::new(1024);
    dev.write(0, 0xff);
    let data = dev.read(0);
    assert_eq!(data, 0xff);
}

#[test]
fn test_write_read_offset() {
    let mut dev = NandDevice::new(1024);
    dev.write(1, 0xee);
    assert_eq!(dev.read(0), 0xff);
    assert_eq!(dev.read(1), 0xee);
}

#[test]
fn test_read_block() {
    let dev = NandDevice::new(1024);
    let data = dev.read_block(1, 0xfe);
    assert_eq!(data.len(), 0xfe);
    assert_eq!(data[0], 0xff);
    assert_eq!(data[0xfd], 0xff);
}

#[test]
fn test_write_block_read_block() {
    let mut dev = NandDevice::new(1024);
    let org_data: Vec<u8> = (0..0xff).collect();
    assert_eq!(org_data.len(), 0xff);

    dev.write_block(0x01, &org_data);
    let read_data = dev.read_block(1, 0xff);
    assert_eq!(read_data.len(), 0xff);
    assert_eq!(read_data[0], 0x00);
    assert_eq!(read_data[0xaa], 0xaa);
    assert_eq!(read_data[0xfe], 0xfe);

    dev.erase(0xaa);
    dev.write(0xaa, 0x55);
    assert_eq!(dev.read(0xaa), 0x55);
}

#[test]
fn test_erase_offset() {
    let mut dev = NandDevice::new(1024);
    dev.write(0xaa, 0x55);
    dev.erase_block(0xaa, 1);
    assert_eq!(dev.read(0xaa), 0xff);
}
