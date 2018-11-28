pub fn find_xor(payload: &[u8]) -> Option<(u8, String, usize)> {
    let scores: Vec<(usize, Vec<u8>, u8)> = (0..0xff)
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

fn get_score(plain: &Vec<u8>) -> usize {
    plain
        .iter()
        .filter(|&&c| c >= 'a' as u8 && c <= 'z' as u8)
        .count()
}

#[test]
fn test_find_xor() {
    use crate::encoding::UnHexExt;

    assert_eq!(
        find_xor(&"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".unhex()),
        Some((0x58, String::from("Cooking MC's like a pound of bacon")))
    )
}
