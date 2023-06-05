fn main() {}

enum Endianness {
    Le,
    Be,
}

#[derive(Debug)]
struct SectionHeaderBlock<'a> {
    major_version: u16,
    minor_version: u16,
    options: &'a [u8],
}

impl<'a> SectionHeaderBlock<'a> {
    fn parse(data: &'a [u8]) -> Self {
        let preamble = 4 + 4 + 4 + 2 + 2 + 8;
        if data.len() < preamble {
            panic!("invalid size")
        }

        let byte_order_magic = u32::from_le_bytes(data[8..][..4].try_into().unwrap());

        let endianness = match byte_order_magic {
            0x1a2b3c4d => Endianness::Le,
            0x4d3c2b1a => Endianness::Be,
            _ => panic!("invalid byte order magic"),
        };

        let block_length_bytes = data[4..][..4].try_into().unwrap();
        let block_length = match endianness {
            Endianness::Le => u32::from_le_bytes(block_length_bytes),
            Endianness::Be => u32::from_be_bytes(block_length_bytes),
        };

        let options = data
            .get(preamble..block_length as usize - preamble - 4)
            .unwrap();

        Self {
            major_version: 0,
            minor_version: 0,
            options,
        }
    }
}

fn parse_u16(endianness: Endianness, slice: &[u8], start_index: usize) -> Option<u32> {
    todo!()
}

fn parse_u32(endianness: Endianness, slice: &[u8], start_index: usize) -> Option<u32> {
    let bytes = *get_array(slice, start_index)?;
    match endianness {
        Endianness::Le => Some(u32::from_le_bytes(bytes)),
        Endianness::Be => Some(u32::from_be_bytes(bytes)),
    }
}

fn get_array<T, const N: usize>(slice: &[T], start_index: usize) -> Option<&[T; N]> {
    slice.get(start_index..)?.get(..N)?.try_into().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &[u8] = &[
        0x0a, 0x0d, 0x0d, 0x0a, 0x8c, 0x00, 0x00, 0x00, 0x4d, 0x3c, 0x2b, 0x1a, 0x01, 0x00, 0x00,
        0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x03, 0x00, 0x2d, 0x00, 0x4d, 0x61,
        0x63, 0x20, 0x4f, 0x53, 0x20, 0x58, 0x20, 0x31, 0x30, 0x2e, 0x31, 0x30, 0x2e, 0x34, 0x2c,
        0x20, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x20, 0x31, 0x34, 0x45, 0x34, 0x36, 0x20, 0x28, 0x44,
        0x61, 0x72, 0x77, 0x69, 0x6e, 0x20, 0x31, 0x34, 0x2e, 0x34, 0x2e, 0x30, 0x29, 0x00, 0x00,
        0x00, 0x04, 0x00, 0x34, 0x00, 0x44, 0x75, 0x6d, 0x70, 0x63, 0x61, 0x70, 0x20, 0x31, 0x2e,
        0x31, 0x32, 0x2e, 0x36, 0x20, 0x28, 0x76, 0x31, 0x2e, 0x31, 0x32, 0x2e, 0x36, 0x2d, 0x30,
        0x2d, 0x67, 0x65, 0x65, 0x31, 0x66, 0x63, 0x65, 0x36, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20,
        0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2d, 0x31, 0x2e, 0x31, 0x32, 0x29, 0x00, 0x00, 0x00,
        0x00, 0x8c, 0x00, 0x00, 0x0,
    ];

    #[test]
    fn it_works() {
        let block = SectionHeaderBlock::parse(DATA);

        assert_eq!(Ok(""), std::str::from_utf8(block.options));
    }
}
