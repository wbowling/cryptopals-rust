use crate::encoding::UnHexExt;


pub fn find_xor(payload: &[u8]) -> Option<(u8, String, i32)> {
    let scores: Vec<(i32, Vec<u8>, u8)> = (0..0xff)
        .map(|i| {
            let plain: Vec<u8> = payload.iter().map(|c| c ^ i).collect();
            let score = if let Err(_) = String::from_utf8(plain.clone()) {
                0
            } else {
                get_score(&plain)
            };

            (score, plain, i)
        })
        .collect();
    if let Some((score, plain, key)) = scores.iter().max_by_key(|(scr, _, _)| scr) {
        let s = String::from_utf8_lossy(&plain);
        Some((*key, String::from(s), *score))
    } else {
        None
    }
}

fn letter_rank(c: u8) -> f32 {
    match c as char {
        'e' => 12.702,
        't' => 9.256,
        'a' => 8.167,
        'o' => 7.507,
        'i' => 6.966,
        'n' => 6.749,
        's' => 6.327,
        'h' => 6.094,
        'r' => 5.987,
        'w' => 5.370,
        'd' => 4.253,
        'l' => 4.025,
        'y' => 3.978,
        'k' => 3.872,
        'c' => 2.782,
        'u' => 2.758,
        'm' => 2.406,
        'f' => 2.228,
        'g' => 2.015,
        'p' => 1.929,
        'b' => 1.492,
        'v' => 0.978,
        'j' => 0.153,
        'x' => 0.150,
        'q' => 0.095,
        'z' => 0.074,
        ' ' => 2.0,
        _ => -1.0,
    }
}

fn get_score(plain: &Vec<u8>) -> i32 {
    let scores = plain.iter().map(|&c| letter_rank(c));
    let tot:f32 = scores.sum();
    return tot as i32;
}

pub fn run() {
    println!("{:?}", find_xor(&"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".unhex()));
}

#[test]
fn test_find_xor() {

    assert_eq!(
        find_xor(&"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".unhex()),
        Some((0x58, String::from("Cooking MC's like a pound of bacon"), 144))
    )
}
