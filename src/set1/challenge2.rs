pub fn xor(str1: &[u8], str2: &[u8]) -> Vec<u8> {
    assert_eq!(str1.len(), str2.len());
    str1.iter().zip(str2).map(|(a, b)| a ^ b ).collect()
}

#[test]
fn test_xor() {
    use crate::encoding::{HexExt, UnHexExt};
    assert_eq!(
        xor(&"1c0111001f010100061a024b53535009181c".unhex(), &"686974207468652062756c6c277320657965".unhex()).hex(),
        "746865206b696420646f6e277420706c6179")
}
