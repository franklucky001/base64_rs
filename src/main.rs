#![feature(unsize)]

mod base64;
use base64::{base64_encode, base64_decode};
use std::mem;
use std::borrow::Borrow;
use std::marker::Unsize;

mod tests{
    extern crate rand;
    use rand::{thread_rng};
    use rand::seq::SliceRandom;
    use super::{base64_encode, base64_decode};

    fn test_one(s: & str){
        let encode_s = base64_encode(s.as_bytes());
        let encode_bytes = encode_s.as_bytes();
        let decode_s = base64_decode(encode_bytes).expect("decode error");
        println!("s : {:}, base64 : {:}, base64 decode : {:}", s, encode_s, decode_s);
        assert_eq!(s, decode_s);
    }

    #[test]
    fn random_test(){
        let letters: Vec<u8> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes().to_vec();
        let mut gen = thread_rng();
        let max_len = 20;
        let sample_size = 10;
        for len in 0..max_len{
            for _ in 0..sample_size{
                let mut sample = Vec::with_capacity(len);
                for _j in 0..len{
                    let chs = letters.choose(&mut gen).unwrap();
                    sample.push(*chs);
                }
                let sample_s = String::from_utf8(sample).unwrap();
                test_one(&sample_s);
            }
        }
    }
}

fn main() {
    let s = "ab";
    let encode_s = base64_encode(s.as_bytes());
    let encode_bytes = encode_s.as_bytes();
    let decode_s = base64_decode(encode_bytes).expect("decode error");
    println!("s : {:}, base64 : {:}, base64 decode : {:}", s, encode_s, decode_s);
}
