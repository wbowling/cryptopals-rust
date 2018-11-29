use crate::encoding::hex2b64;

pub fn run() {
    test_hex2b64();
}

fn test_hex2b64() {
    assert_eq!(
        hex2b64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
        String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"))
}

#[test]
pub fn test() {
    run();
}