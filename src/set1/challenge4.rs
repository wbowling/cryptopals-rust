use std::fs::File;
use std::io::Read;

use crate::crypto::xor::find_xor;
use crate::crypto::xor::xor;
use crate::crypto::xor::XorScore;
use crate::encoding::UnHexExt;

pub fn run() {
    test_find_xor();
}

fn get_lines() -> Vec<String> {
    let mut f = File::open("data/4.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.split("\n").map(|s| s.to_string()).collect()
}


fn test_find_xor() {
    let ( line, score) = get_lines()
        .iter()
        .map(|l| (l.clone(), find_xor(&l.as_str().unhex())))
        .max_by(|a, b| a.1.cmp(&b.1))
        .expect("xor not found");

    assert_eq!(score, XorScore(53, 154));
    assert_eq!(
        String::from_utf8_lossy(&xor(
            &line.as_str().unhex(),
            &[score.0])
        ),
        String::from("Now that the party is jumping\n")
    );
}

#[test]
pub fn test() {
    run();
}