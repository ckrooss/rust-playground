use crate::flash_device::FlashDevice;
use std::cmp::{max, min};
use std::collections::BTreeMap;

pub struct SparseDevice {
    size: usize,
    chunks: BTreeMap<usize, Vec<u8>>,
}

impl SparseDevice {
    pub fn new(size: usize) -> SparseDevice {
        SparseDevice {
            size,
            chunks: BTreeMap::new(),
        }
    }

    pub fn used_chunks(&self) -> usize {
        self.chunks.len()
    }
}

impl FlashDevice for SparseDevice {
    fn read(&self, offset: usize, size: usize) -> Vec<u8> {
        if offset + size > self.size {
            panic!("Error::OutOfBounds");
        }
        let mut read_data = vec![0xFF; size];

        for (chunk_offset, chunk_data) in &self.chunks {
            if *chunk_offset >= offset + size {
                break;
            }

            if *chunk_offset + chunk_data.len() <= offset {
                continue;
            }

            let write_start = max(*chunk_offset, offset);
            let write_end = min(*chunk_offset + chunk_data.len(), offset + size);
            let copy_size = write_end - write_start;

            let src_start = write_start - chunk_offset;
            let src_end = src_start + copy_size;
            let dst_start = write_start - offset;

            read_data[dst_start..dst_start + copy_size]
                .copy_from_slice(&chunk_data[src_start..src_end]);
        }

        read_data
    }

    fn write(&mut self, offset: usize, data: &[u8]) {
        if offset + data.len() > self.size {
            panic!("Error::OutOfBounds");
        }

        let end = offset + data.len();

        let mut result_chunks: BTreeMap<usize, Vec<u8>> = BTreeMap::new();
        let mut data_inserted: bool = false;

        for (chunk_offset, chunk_data) in self.chunks.iter_mut() {
            let chunk_len = chunk_data.len();
            let chunk_end = chunk_offset + chunk_len;

            // Irrelevant: reuse chunk
            if *chunk_offset >= end || chunk_end <= offset {
                result_chunks.insert(*chunk_offset, chunk_data.to_vec());
                continue;
            }

            // New chunk exactly replaces old chunk
            if *chunk_offset == offset && chunk_len == data.len() {
                result_chunks.insert(offset, data.to_vec());
                data_inserted = true;
                break;
            }

            // New chunk starts before, ends after and completely replaces old chunk
            if offset <= *chunk_offset && end >= chunk_end + (*chunk_offset - offset) {
                result_chunks.insert(offset, data.to_vec());
                data_inserted = true;
                break;
            }

            // New chunk starts inside existing chunk
            if *chunk_offset <= offset && chunk_end >= offset {
                let new_len = chunk_len + end - chunk_end;
                chunk_data.resize(new_len, 0);
                chunk_data[new_len - data.len()..].copy_from_slice(data);
                result_chunks.insert(*chunk_offset, chunk_data.to_vec());
                data_inserted = true;
                break;
            }

            if *chunk_offset <= end && chunk_end >= end {
                // How many bytes more will we have at the end? Reserve space for that
                let additional = chunk_end - end;
                let mut new_data = data.to_vec();
                new_data.resize(data.len() + additional, 0);
                // Where the old data we need starts
                let old_data_begin = chunk_data.len() - additional;
                // Where the new data we need stops
                new_data[data.len()..].copy_from_slice(&chunk_data[old_data_begin..]);
                result_chunks.insert(offset, new_data);
                data_inserted = true;
                break;
            }
        }

        if !data_inserted {
            result_chunks.insert(offset, data.to_vec());
        }

        self.chunks = result_chunks;
    }

    fn erase(&mut self, offset: usize, size: usize) {
        if offset + size > self.size {
            panic!("Error::OutOfBounds");
        }

        let mut remove_keys = Vec::new();

        for (chunk_offset, _) in self.chunks.range(offset..(offset + size)) {
            remove_keys.push(*chunk_offset);
        }

        for key in remove_keys {
            self.chunks.remove(&key);
        }
    }

    fn erase_device(&mut self) {
        self.chunks.clear();
    }
}
