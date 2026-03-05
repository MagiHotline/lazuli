//! A `.cso` or '.ciso' file is a disc format designed
//! to save space without the CPU overhead of actual compression

use std::io::{Read, Seek, SeekFrom};
use binrw::{BinRead, BinWrite, meta::ReadMagic};
use crate::apploader;

const CSO_HEADER_SIZE: usize = 0x8000; // 32KB
const CSO_MAP_SIZE: usize = CSO_HEADER_SIZE - size_of::<u32>() - 4; // 0x8000 (32768) - 4 (magic) - 4 (block_size)

// IMPLEMENT Cso<R> and CsoReader<R>

/// The header of a CSO file.
#[derive(Debug, Clone, BinRead, BinWrite)]
#[br(big, magic = b"CISO")]
pub struct CsoHeader {
    #[br(little)]
    /// Chunks of 2MB of data
    block_size: u32,
    /// Used (1) or Unused (0)
    map: [u8; CSO_MAP_SIZE]
}

impl CsoHeader {
    pub fn block_size(&self) -> u32 {
        self.block_size
    }

    pub fn map(&self) -> &[u8] {
        &self.map
    }
}

/// A Gamecube .cso file.
pub struct Cso<R> {
    /// Header of the file
    header: CsoHeader,
    /// LUT
    cso_map: Vec<Option<u64>>,
    /// Reader of the contents
    reader: R
}

impl<R> Cso<R>
where
    R: Read + Seek
{
    pub fn new(mut reader: R) -> Result<Self, binrw::Error> {
        let header = CsoHeader::read(&mut reader)?;

        let mut cso_map = Vec::with_capacity(CSO_MAP_SIZE);
        let mut current_offset = CSO_HEADER_SIZE as u64;

        for &is_present in header.map() {
            if is_present == 1
            {
                cso_map.push(Some(current_offset));
                current_offset += header.block_size() as u64;
            }
            else
            {
                cso_map.push(None);
            }
        }

        Ok(Self { header, cso_map, reader })
    }

    pub fn header(&self) -> &CsoHeader {
        &self.header
    }

    pub fn map(&self) -> &Vec<Option<u64>> {
        &self.cso_map
    }

    pub fn reader(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Read from disk at the given offset and writes it into the output buffer.
    /// Returns how many bytes were actually read.
    pub fn read(&mut self, disk_offset: u64, out: &mut [u8]) -> std::io::Result<u64> {
        let mut current_disk_offset = disk_offset;
        let mut remaining = out.len() as u64; // Remaining bytes to read
        let block_size = self.header().block_size() as u64;

        while remaining > 0 {

            let block = (current_disk_offset / block_size) as usize;
            let data_offset = current_disk_offset % block_size;
            let to_read = remaining.min(block_size - data_offset);

            let out_start = (current_disk_offset - disk_offset) as usize;
            let out = &mut out[out_start as usize..][..to_read as usize];

            match self.cso_map[block] {
                Some(cso_block) => {
                    self.reader.seek(SeekFrom::Start(cso_block + data_offset))?;
                    self.reader.read_exact(out)?;
                }
                None => {
                    out.fill(0);
                }
            }

            current_disk_offset += to_read;
            remaining -= to_read;
        }

        Ok(out.len() as u64 - remaining)
    }
}
