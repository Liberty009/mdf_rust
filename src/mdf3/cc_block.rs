use crate::utils;

use super::conversion_data::ConversionData;


#[derive(Debug, Clone, PartialEq)]
pub struct Ccblock {
    pub block_type: [u8; 2],
    pub block_size: u16,
    pub physical_range_valid: u16,
    pub physical_min: f64,
    pub physical_max: f64,
    pub unit: [u8; 20],
    pub conversion_type: u16,
    pub size_info: u16,
    pub conversion_data: ConversionData,
}

impl Ccblock {
    #[allow(dead_code)]
    pub fn write() {}
    #[allow(dead_code)]
    pub fn read(stream: &[u8], little_endian: bool) -> (Self, usize) {
        let mut position = 0;
        let block_type: [u8; 2] = stream[position..position + 2].try_into().expect("msg");
        position += block_type.len();

        if !utils::eq(&block_type, &[b'C', b'C']) {
            panic!("CC not found");
        }

        let block_size: u16 = utils::read(stream, little_endian, &mut position);
        let physical_range_valid: u16 = utils::read(stream, little_endian, &mut position);
        let physical_min: f64 = utils::read(stream, little_endian, &mut position);
        let physical_max: f64 = utils::read(stream, little_endian, &mut position);
        let unit: [u8; 20] = stream[position..position + 20].try_into().expect("msg");
        position += unit.len();
        let conversion_type: u16 = utils::read(stream, little_endian, &mut position);
        let size_info: u16 = utils::read(stream, little_endian, &mut position);

        let datatype = 1;

        let (conversion_data, pos) = ConversionData::read(stream, little_endian, datatype);
        position += pos;

        (
            Self {
                block_type,
                block_size,
                physical_range_valid,
                physical_min,
                physical_max,
                unit,
                conversion_type,
                size_info,
                conversion_data,
            },
            position,
        )
    }
}

#[cfg(test)]
mod ccblock_test {
    use crate::utils;

    use super::*;

    #[test]
    fn read() {
        let cc_data = [
            0x43, 0x43, 0x2E, 0x00, 0x01, 0x00, 0x04, 0x19, 0x60, 0x9C, 0xAE, 0xDD, 0xBC, 0x3F,
            0x52, 0xE8, 0x62, 0xFA, 0x56, 0xD3, 0x28, 0x40, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xFF, 0xFF, 0x00, 0x00, 0x43, 0x45, 0x80, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x43, 0x68, 0x61, 0x6E, 0x6E, 0x65, 0x6C, 0x20, 0x69, 0x6E, 0x73, 0x65,
            0x72, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x50, 0x79, 0x74, 0x68, 0x6F, 0x6E,
            0x20, 0x53, 0x63, 0x72, 0x69, 0x70, 0x74, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x43, 0x4E, 0xE4, 0x00, 0xA6, 0xE3, 0x10, 0x00,
            0x80, 0xE0, 0x10, 0x00, 0xAE, 0xE0, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x00, 0x74, 0x69, 0x6D, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let (cc_block, position) = Ccblock::read(&cc_data, true);

        assert_eq!(position, 47); // should match the block size
        assert_eq!(cc_block.block_size, 46);
        assert_eq!(cc_block.physical_range_valid, 1);

        assert!( (
            cc_block.physical_min - 
            utils::read::<f64>(
                &[0x04, 0x19, 0x60, 0x9C, 0xAE, 0xDD, 0xBC, 0x3F],
                true,
                &mut 0_usize
            )).abs() < 0.1
        );
        assert!((
            cc_block.physical_max -
            utils::read::<f64>(
                &[0x52, 0xE8, 0x62, 0xFA, 0x56, 0xD3, 0x28, 0x40],
                true,
                &mut 0_usize
            )).abs() < 0.1

        );
        assert!(utils::eq(
            &cc_block.unit,
            &[
                0x73, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00
            ]
        ));

        assert_eq!(cc_block.conversion_type, 65535);
        assert_eq!(cc_block.size_info, 0);
        // assert_eq!(conversion_data: ConversionData,);
    }

    #[test]
    fn write() {}
}