use std::fs::File;

use crate::encoding::UnHexExt;
use crate::set1::challenge3::find_xor;
use std::io::Read;

pub fn run() -> bool {
    let mut f = File::open("src/set1/data/4.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let scores: Vec<(u8, String, i32)> = contents
        .split("\n")
        .map(|l| find_xor(&l.unhex()))
        .filter_map(|l| l)
        .collect();
    if let Some((key, plain, score)) = scores.iter().max_by_key(|&o| o.2) {
        println!("Key: 0x{:x}, {} ({})", key, plain, score);
        true
    } else {
        false
    }
}

#[test]
fn test_find_xor() {
    assert_eq!(run(), true);
}

