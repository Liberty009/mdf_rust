use std::mem;

use crate::utils;
use crate::MDF4::Block::Block;
pub struct BlockHeader {
    pub id: [u8; 4],
    reserved0: [u8; 4],
    pub length: u64,
    pub link_count: u64,
}

impl Block for BlockHeader {
    fn new() -> Self {
        Self {
            id: [0; 4],
            reserved0: [0; 4],
            length: 0,
            link_count: 0,
        }
    }
    fn default() -> Self {
        Self {
            id: [0; 4],
            reserved0: [0; 4],
            length: 0,
            link_count: 0,
        }
    }
    fn read(stream: &[u8], position: usize, little_endian: bool) -> (usize, Self) {
        let mut pos = position;
        let id: [u8; 4] = utils::read(stream, little_endian, &mut pos);
        let reserved0: [u8; 4] = utils::read(stream, little_endian, &mut pos);

        let length = utils::read(stream, little_endian, &mut pos);
        let link_count = utils::read(stream, little_endian, &mut pos);

        (
            pos,
            Self {
                id,
                reserved0,
                length,
                link_count,
            },
        )
    }

    fn byte_len(&self) -> usize {
        self.id.len()
            + self.reserved0.len()
            + mem::size_of_val(&self.length)
            + mem::size_of_val(&self.link_count)
    }
}
