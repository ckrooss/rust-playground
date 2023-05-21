pub mod flash_device;
pub mod nand_device;
pub mod sparse_device;

#[cfg(test)]
mod test {
    use crate::flash_device::FlashDevice;
    use crate::nand_device::NandDevice;
    use crate::sparse_device::SparseDevice;
    use test_case::test_case;

    #[test_case(&mut NandDevice::new(1024))]
    #[test_case(&mut SparseDevice::new(1024))]
    fn test_read(dev: &mut dyn FlashDevice) {
        let data = dev.read(0, 1);
        assert_eq!(data[0], 0xff);
    }

    #[test_case(&mut NandDevice::new(8))]
    #[test_case(&mut SparseDevice::new(8))]
    fn test_read_edges(dev: &mut dyn FlashDevice) {
        dev.read(0, 1);
        dev.read(7, 1);

        dev.read(0, 8);
        dev.read(7, 1);
    }

    #[test_case(&mut NandDevice::new(1024))]
    #[test_case(&mut SparseDevice::new(1024))]
    fn test_write(dev: &mut dyn FlashDevice) {
        dev.write(0, &[0]);
    }

    #[test_case(&mut NandDevice::new(8))]
    #[test_case(&mut SparseDevice::new(8))]
    fn test_write_edges(dev: &mut dyn FlashDevice) {
        dev.write(0, &[0xee]);
        dev.write(7, &[0xee]);

        dev.erase_device();
        dev.write(0, &[0xee; 8]);

        dev.erase_device();
        dev.write(7, &[0xee; 1]);
    }

    #[test_case(&mut NandDevice::new(1024))]
    #[test_case(&mut SparseDevice::new(1024))]
    fn test_write_read(dev: &mut dyn FlashDevice) {
        dev.write(0, &[0xff]);
        let data = dev.read(0, 1);
        assert_eq!(data[0], 0xff);
    }

    #[test_case(&mut NandDevice::new(1024))]
    #[test_case(&mut SparseDevice::new(1024))]
    fn test_write_read_offset(dev: &mut dyn FlashDevice) {
        dev.write(1, &[0xee]);
        assert_eq!(dev.read(0, 2), &[0xff, 0xee]);
    }

    #[test_case(&mut NandDevice::new(1024))]
    #[test_case(&mut SparseDevice::new(1024))]
    fn test_read_block(dev: &mut dyn FlashDevice) {
        let data = dev.read(1, 0xfe);
        assert_eq!(data.len(), 0xfe);
        assert_eq!(data[0], 0xff);
        assert_eq!(data[0xfd], 0xff);
    }

    #[test_case(&mut NandDevice::new(1024))]
    #[test_case(&mut SparseDevice::new(1024))]
    fn test_write_block_read_block(dev: &mut dyn FlashDevice) {
        let org_data: Vec<u8> = (0..0xff).collect();
        assert_eq!(org_data.len(), 0xff);

        dev.write(0x01, &org_data);
        let read_data = dev.read(1, 0xff);
        assert_eq!(read_data.len(), 0xff);
        assert_eq!(read_data[0], 0x00);
        assert_eq!(read_data[0xaa], 0xaa);
        assert_eq!(read_data[0xfe], 0xfe);

        dev.erase(0xaa, 1);
        dev.write(0xaa, &[0x55]);
        assert_eq!(dev.read(0xaa, 1), &[0x55]);
    }

    #[test_case(&mut NandDevice::new(1024))]
    #[test_case(&mut SparseDevice::new(1024))]
    fn test_erase_offset(dev: &mut dyn FlashDevice) {
        dev.write(0xaa, &[0x55]);
        dev.erase(0xaa, 1);
        assert_eq!(dev.read(0xaa, 1), &[0xff]);
    }
    #[test]
    fn write_overlapping_blocks() {
        let mut dev = SparseDevice::new(1024);

        let testdata: Vec<u8> = vec![0x01, 0x02, 0x03];
        dev.write(0, &testdata);

        assert_eq!(dev.read(0, 3), testdata);
        dev.write(1, &testdata);

        let expected_data: Vec<u8> = vec![0x01, 0x01, 0x02, 0x03];
        assert_eq!(dev.read(0, 4), expected_data);
        assert_eq!(dev.used_chunks(), 1);
    }

    #[test]
    fn write_overlap_from_left() {
        {
            let mut dev = SparseDevice::new(1024);
            let data1: Vec<u8> = vec![0x01, 0x02, 0x03];
            let data2: Vec<u8> = vec![0x04, 0x05, 0x06, 0x07];
            let result: Vec<u8> = vec![0x04, 0x05, 0x06, 0x07, 0x02];
            dev.write(3, &data1);
            dev.write(0, &data2);
            assert_eq!(dev.read(0, 5), result);
            assert_eq!(dev.used_chunks(), 1);
        }
        {
            let mut dev = SparseDevice::new(1024);
            let data1: Vec<u8> = vec![0x01, 0x02, 0x03];
            let data2: Vec<u8> = vec![0x04, 0x05, 0x06];
            let result: Vec<u8> = vec![0x04, 0x05, 0x06, 0x02];
            dev.write(2, &data1);
            dev.write(0, &data2);
            assert_eq!(dev.read(0, 4), result);
            assert_eq!(dev.used_chunks(), 1);
        }
    }

    #[test]
    fn write_overlap_from_right() {
        let mut dev = SparseDevice::new(1024);

        let data1: Vec<u8> = vec![0x01, 0x02, 0x03];
        let data2: Vec<u8> = vec![0x04, 0x05, 0x06];
        let result: Vec<u8> = vec![0x01, 0x02, 0x04, 0x05, 0x06];
        dev.write(0, &data1);
        dev.write(2, &data2);
        assert_eq!(dev.read(0, 5), result);
        assert_eq!(dev.used_chunks(), 1);
    }

    #[test]
    fn write_overlap_complete() {
        let mut dev = SparseDevice::new(1024);

        let data1: Vec<u8> = vec![0x01, 0x02, 0x03];
        let data2: Vec<u8> = vec![0x04, 0x05, 0x06];
        let result: Vec<u8> = vec![0x04, 0x05, 0x06];
        dev.write(0, &data1);
        dev.write(0, &data2);
        assert_eq!(dev.read(0, 3), result);
        assert_eq!(dev.used_chunks(), 1);
    }

    #[test]
    fn read_overlap() {
        let mut dev = SparseDevice::new(1024);

        let write_data: Vec<u8> = vec![0x01, 0x02, 0x03];
        dev.write(20, &write_data);

        let expected_data: Vec<u8> =
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x02, 0x03, 0xff, 0xff];
        assert_eq!(dev.read(15, 10), expected_data);
        assert_eq!(dev.used_chunks(), 1);
    }

    #[test]
    fn read_overlap_multiple_chunks() {
        let mut dev = SparseDevice::new(1024);

        let write_data: Vec<u8> = vec![0x01, 0x02, 0x03];
        dev.write(3, &write_data);
        assert_eq!(dev.used_chunks(), 1);
        dev.write(7, &write_data);
        assert_eq!(dev.used_chunks(), 2);

        // Should now be ff ff ff 01 02 03 ff 01 02 03
        assert_eq!(
            dev.read(0, 10),
            &[0xff, 0xff, 0xff, 0x01, 0x02, 0x03, 0xff, 0x01, 0x02, 0x03]
        );
    }
}
