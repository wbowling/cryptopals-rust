pub fn run() {
    test_xor();
}

fn test_xor() {
    use crate::encoding::{HexExt, UnHexExt};
    use crate::crypto::xor::xor;

    assert_eq!(
        xor(&"1c0111001f010100061a024b53535009181c".unhex(), &"686974207468652062756c6c277320657965".unhex()).hex(),
        "746865206b696420646f6e277420706c6179")
}

#[test]
pub fn test() {
    run();
}