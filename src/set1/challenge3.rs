use crate::crypto::xor::find_xor;
use crate::crypto::xor::xor;
use crate::crypto::xor::XorScore;
use crate::encoding::UnHexExt;

pub fn run() {
    test_find_xor();
}

fn test_find_xor() {
    let payload = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".unhex();
    let score = find_xor(&payload);

    assert_eq!(score, XorScore(0x58, 154));
    assert_eq!(
        xor(&payload, b"\x58"),
        "Cooking MC's like a pound of bacon".as_bytes()
    );
}

#[test]
pub fn test() {
    run();
}