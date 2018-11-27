extern crate base64;

use base64::encode;

use crate::encoding::UnHexExt;

pub fn hex2b64(s: &str) -> String {
    encode(&s.unhex())
}

#[test]
fn test_hex2b64() {
    assert_eq!(
        hex2b64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
        String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"))
}
