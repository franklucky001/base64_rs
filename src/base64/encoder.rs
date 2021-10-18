use super::table::{LETTERS, PADDING};

#[derive(Debug)]
enum EncodeItem{
    FullItem(u8, u8, u8),
    TwoItem(u8, u8),
    OneItem(u8),
    EmptyItem
}

impl EncodeItem {
    pub fn new(slice: &[u8]) -> Self{
        match slice.len() {
            0 => Self::EmptyItem,
            1 => Self::OneItem(slice[0]),
            2 => Self::TwoItem(slice[0], slice[1]),
            3 => Self::FullItem(slice[0], slice[1], slice[2]),
            _ => panic!()
        }
    }
    pub fn encode_to_buffer(&self, buffer: &mut Vec<u8>){
        match self {
            Self::FullItem(item_0, item_1, item_2) => {
                let unit_0 = item_0 >> 2;
                buffer.push(LETTERS[unit_0 as usize]);
                let unit_1: u8 = ((item_0 << 4) & 0x30) | (item_1 >> 4);
                buffer.push(LETTERS[unit_1 as usize]);
                let unit_2: u8 = ((item_1 << 2) & 0x3c) | (item_2 >> 6);
                buffer.push(LETTERS[unit_2 as usize]);
                let unit_3: u8 = item_2 & 0x3f;
                buffer.push(LETTERS[unit_3 as usize]);
            }
            Self::TwoItem(item_0, item_1) => {
                let unit_0 = item_0 >> 2;
                buffer.push(LETTERS[unit_0 as usize]);
                let unit_1 = ((item_0 << 4) & 0x30) | (item_1 >> 4);
                buffer.push(LETTERS[unit_1 as usize]);
                let unit_2 = (item_1 << 2) & 0x3c;
                buffer.push(LETTERS[unit_2 as usize]);
                buffer.push(PADDING);
            }
            Self::OneItem(item_0) => {
                let unit_0 = item_0 >> 2;
                buffer.push(LETTERS[unit_0 as usize]);
                let unit_1 = (item_0 << 4) & 0x30;
                buffer.push(LETTERS[unit_1 as usize]);
                // pad two '=' of index
                buffer.push(PADDING);
                buffer.push(PADDING);
            }
            Self::EmptyItem => {}
        }
    }
}

pub fn base64_encode(source: &[u8]) -> String{
    let len = source.len();
    if len == 0 {
        return String::new();
    }
    let mut buffer = Vec::with_capacity(len * 8 / 6 + 2);
    for chunk in source.chunks(3){
        let encode_item = EncodeItem::new(chunk);
        encode_item.encode_to_buffer(&mut buffer);
    }
    String::from_utf8(buffer).expect("encode error")
}