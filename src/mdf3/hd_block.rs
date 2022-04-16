use crate::utils;


#[derive(Debug, Clone, Copy)]
pub struct Hdblock {
    pub position: usize,
    pub block_type: [u8; 2],
    pub block_size: u16,
    pub data_group_block: u32,
    pub file_comment: u32,
    pub program_block: u32,
    pub data_group_number: u16,
    pub date: [u8; 10],
    pub time: [u8; 8],
    pub author: [u8; 32],
    pub department: [u8; 32],
    pub project: [u8; 32],
    pub subject: [u8; 32],
    pub timestamp: u64,
    pub utc_time_offset: i16,
    pub time_quality: u16,
    pub timer_id: [u8; 32],
}

impl Hdblock {
    pub fn write() {}
    pub fn read(stream: &[u8], position: usize, little_endian: bool) -> (Hdblock, usize) {
        let mut pos = position;
        let block_type: [u8; 2] = stream[position..position + 2].try_into().expect("");

        if !utils::eq(&block_type, &[b'H', b'D']) {
            panic!("Incorrect type for HDBLOCK");
        }

        pos += block_type.len();
        let block_size = utils::read(stream, little_endian, &mut pos);
        let data_group_block = utils::read(stream, little_endian, &mut pos);
        let file_comment = utils::read(stream, little_endian, &mut pos);
        let program_block = utils::read(stream, little_endian, &mut pos);
        let data_group_number = utils::read(stream, little_endian, &mut pos);
        let date: [u8; 10] = stream[pos..pos + 10].try_into().expect("msg");
        pos += date.len();
        let time: [u8; 8] = stream[pos..pos + 8].try_into().expect("msg");
        pos += time.len();
        let author: [u8; 32] = stream[pos..pos + 32].try_into().expect("msg");
        pos += author.len();
        let department: [u8; 32] = stream[pos..pos + 32].try_into().expect("msg");
        pos += department.len();
        let project: [u8; 32] = stream[pos..pos + 32].try_into().expect("msg");
        pos += project.len();
        let subject: [u8; 32] = stream[pos..pos + 32].try_into().expect("msg");
        pos += subject.len();
        let timestamp = utils::read(stream, little_endian, &mut pos);
        let utc_time_offset = utils::read(stream, little_endian, &mut pos);
        let time_quality = utils::read(stream, little_endian, &mut pos);
        let timer_id: [u8; 32] = stream[pos..pos + 32].try_into().expect("msg");
        pos += timer_id.len();

        (
            Hdblock {
                position,
                block_type,
                block_size,
                data_group_block,
                file_comment,
                program_block,
                data_group_number,
                date,
                time,
                author,
                department,
                project,
                subject,
                timestamp,
                utc_time_offset,
                time_quality,
                timer_id,
            },
            pos,
        )
    }
}

#[cfg(test)]
mod hdblock_test {
    use crate::utils;

    use super::*;

    #[test]
    fn read() {
        let hd_data = [
            0x48, 0x44, 0xD0, 0x00, 0xD8, 0xDF, 0x10, 0x00, 0x10, 0x01, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x06, 0x00, 0x32, 0x32, 0x3A, 0x31, 0x31, 0x3A, 0x32, 0x30, 0x31, 0x38,
            0x31, 0x34, 0x3A, 0x32, 0x36, 0x3A, 0x33, 0x35, 0x4A, 0x61, 0x63, 0x6B, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x46, 0xF9,
            0x75, 0x78, 0x69, 0x15, 0x00, 0x00, 0x00, 0x00, 0x4C, 0x6F, 0x63, 0x61, 0x6C, 0x20,
            0x50, 0x43, 0x20, 0x52, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6E, 0x63, 0x65, 0x20, 0x54,
            0x69, 0x6D, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0x58,
            0xCC, 0x02, 0x3C, 0x48, 0x44, 0x63, 0x6F, 0x6D, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x78,
            0x6D, 0x6C, 0x6E, 0x73, 0x3D, 0x22, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77,
            0x77, 0x77, 0x2E, 0x61, 0x73, 0x61, 0x6D, 0x2E, 0x6E, 0x65, 0x74, 0x2F, 0x6D, 0x64,
            0x66, 0x2F, 0x76, 0x34, 0x22, 0x3E, 0x3C, 0x54, 0x58, 0x3E, 0x44, 0x61, 0x74, 0x65,
            0x3A, 0x20, 0x32, 0x32, 0x2E, 0x31, 0x31, 0x2E, 0x32, 0x30, 0x31, 0x38, 0x0D, 0x0A,
            0x54, 0x69, 0x6D, 0x65, 0x3A, 0x20, 0x31, 0x35, 0x3A, 0x32, 0x37, 0x0D, 0x0A, 0x52,
            0x65, 0x63, 0x6F, 0x72, 0x64, 0x69, 0x6E, 0x67, 0x20, 0x44, 0x75, 0x72, 0x61, 0x74,
            0x69, 0x6F, 0x6E, 0x3A, 0x20, 0x30, 0x30, 0x3A, 0x30, 0x30, 0x3A, 0x31, 0x32, 0x0D,
            0x0A, 0xA7, 0x40, 0x0D, 0x0A, 0x44, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x3A,
            0x20, 0x54, 0x65, 0x73, 0x74, 0x0D, 0x0A, 0x45, 0x78, 0x70, 0x65, 0x72, 0x69, 0x6D,
            0x65, 0x6E, 0x74, 0x3A, 0x20, 0x45, 0x78, 0x70, 0x65, 0x72, 0x69, 0x6D, 0x65, 0x6E,
            0x74, 0x0D, 0x0A, 0x57, 0x6F, 0x72, 0x6B, 0x73, 0x70, 0x61, 0x63, 0x65, 0x3A, 0x20,
            0x57, 0x6F, 0x72, 0x6B, 0x73, 0x70, 0x61, 0x63, 0x65, 0x0D, 0x0A, 0x44, 0x65, 0x76,
            0x69, 0x63, 0x65, 0x73, 0x3A, 0x20, 0x45, 0x54, 0x4B, 0x20, 0x74, 0x65, 0x73, 0x74,
            0x20, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x3A, 0x31, 0x0D, 0x0A, 0x50, 0x72, 0x6F,
            0x67, 0x72, 0x61, 0x6D, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69,
            0x6F, 0x6E, 0x3A, 0x20, 0x41, 0x53, 0x41, 0x50, 0x32, 0x5F,
        ];

        let (hd_block, position) = Hdblock::read(&hd_data, 0, true);

        println!("Length {}", position);
        assert_eq!(position, 208);

        assert_eq!(hd_block.block_size, 208);
        assert_eq!(hd_block.data_group_block, 1105880);
        assert_eq!(hd_block.file_comment, 272);
        assert_eq!(hd_block.program_block, 0);
        assert_eq!(hd_block.data_group_number, 6);
        assert!(utils::eq(
            &hd_block.date,
            &[0x32, 0x32, 0x3A, 0x31, 0x31, 0x3A, 0x32, 0x30, 0x31, 0x38,]
        ));
        assert!(utils::eq(
            &hd_block.time,
            &[0x31, 0x34, 0x3A, 0x32, 0x36, 0x3A, 0x33, 0x35,]
        ));
        assert!(utils::eq(
            &hd_block.author,
            &[
                0x4A, 0x61, 0x63, 0x6B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ]
        ));
        assert!(utils::eq(
            &hd_block.department,
            &[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ]
        ));
        assert!(utils::eq(
            &hd_block.project,
            &[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ]
        ));
        assert!(utils::eq(
            &hd_block.subject,
            &[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ]
        ));
        assert_eq!(hd_block.timestamp, 1542896795439737088);
        assert_eq!(hd_block.utc_time_offset, 0);
        assert_eq!(hd_block.time_quality, 0);
        assert!(utils::eq(
            &hd_block.timer_id,
            &[
                0x4C, 0x6F, 0x63, 0x61, 0x6C, 0x20, 0x50, 0x43, 0x20, 0x52, 0x65, 0x66, 0x65, 0x72,
                0x65, 0x6E, 0x63, 0x65, 0x20, 0x54, 0x69, 0x6D, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ]
        ));
    }

    #[test]
    fn write() {}
}