use super::table::{LETTERS, PAD_INDEX};
use std::fmt::{Debug, Display, Formatter};
use crate::base64::decoder::DecodeError::{InnerPadding, MissingPadding, Invalid};


#[derive(Debug)]
pub enum DecodeError{
    Invalid(String),
    InnerPadding(String),
    MissingPadding(String)
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}", self.to_string())
    }
}

impl std::error::Error for DecodeError {}

#[derive(Debug)]
enum DecodeItem{
    NoPad(u8, u8, u8, u8),
    OnePad(u8, u8, u8, u8),
    TwoPad(u8, u8, u8, u8)
}

impl DecodeItem {
    fn new(slice: & [u8]) -> Self{
        let chs_len = slice.iter().filter(|&c|*c != PAD_INDEX).count();
        match chs_len{
            2 => Self::TwoPad(slice[0], slice[1], slice[2], slice[3]),
            3 => Self::OnePad(slice[0], slice[1], slice[2], slice[3]),
            4 => Self::NoPad(slice[0], slice[1], slice[2], slice[3]),
            _ => panic!()
        }
    }
    fn decode_to_buffer(&self, buffer: &mut Vec<u8>) -> Result<(), DecodeError>{
        match self {
            Self::NoPad(item_0, item_1, item_2, item_3) => {
                if *item_0 == PAD_INDEX
                    || *item_1 == PAD_INDEX
                    || *item_2 == PAD_INDEX
                    || *item_3 == PAD_INDEX{
                    return Err(InnerPadding("invalid data with '=' which not in end of base64 string".to_string()))
                }
                let unit_0 = (*item_0 << 2) | (*item_1 >> 4);
                buffer.push(unit_0);
                let unit_1 = (*item_1 << 4) | (*item_2 >> 2);
                buffer.push(unit_1);
                let unit_2 = (*item_2 << 6) | *item_3;
                buffer.push(unit_2);
            }
            Self::OnePad(item_0, item_1, item_2, item_3) => {
                if *item_0 == PAD_INDEX || *item_1 == PAD_INDEX || *item_2 == PAD_INDEX{
                    return Err(InnerPadding("invalid data with '=' which not in end of base64 string".to_string()))
                }
                if *item_3 != PAD_INDEX{
                    return Err(MissingPadding("invalid data missing '=' in base64 string end".to_string()))
                }
                let unit_0 = (*item_0 << 2) | (*item_1 >> 4);
                buffer.push(unit_0);
                let unit_1 = (*item_1 << 4) | (*item_2 >> 2);
                buffer.push(unit_1);
            }
            Self::TwoPad(item_0, item_1, item_2, item_3) => {
                if *item_0 == PAD_INDEX || *item_1 == PAD_INDEX{
                    return Err(InnerPadding("invalid data with '=' which not in end of base64 string".to_string()))
                }
                if *item_2 != PAD_INDEX || *item_3 != PAD_INDEX{
                    return Err(MissingPadding("invalid data missing '=' in base64 string end".to_string()))
                }
                let unit_0 = *item_0 << 2 | (*item_1 >> 4);
                buffer.push(unit_0);
            }
        }
        Ok(())
    }
}



pub fn base64_decode(base64_str: &[u8]) -> Result<String, DecodeError>{
    let len = base64_str.len();
    if len == 0{
        return Ok(String::new());
    }
    if len % 4 != 0{
        return Err(Invalid("The length of base64 string must be a multiple of 4 !!".to_string()));
    }
    let mut buffer = Vec::with_capacity(len * 6 / 8);
    for chunk in base64_str.chunks(4){
        let mut chunk_ids = [0u8;4];
        for (i, &ch) in chunk.iter().enumerate(){
            match LETTERS.iter().position(|e| e == &ch) {
                Some(ci) => {
                    chunk_ids[i] = ci as u8;
                }
                None => {
                    return Err(Invalid("invalid data which not found in base64 table".to_string()));
                }
            }
        }
        let decode_item = DecodeItem::new(&chunk_ids);
        decode_item.decode_to_buffer(&mut buffer)?;
    }
    Ok(String::from_utf8(buffer).expect("decode to string error"))
}