use std::cmp::Ordering;

pub fn xor(str1: &[u8], str2: &[u8]) -> Vec<u8> {
    let (s1,s2) = if str1.len() < str2.len() {
        (str1.iter().cycle(), str2.iter())
    } else {
        (str2.iter().cycle(), str1.iter())
    };
    s1.zip(s2).map(|(a, b)| a ^ b ).collect()
}

#[derive(Eq,PartialEq,Debug)]
pub struct XorScore(pub u8, pub i32);

impl PartialOrd for XorScore {
    fn partial_cmp(&self, other: &XorScore) -> Option<Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

impl Ord for XorScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

pub fn find_xor(payload: &[u8]) -> XorScore {
    (0..0xff).map(|i| {
            XorScore(i, get_score(&payload.iter().map(|c| c ^ i).collect()))
        }).max().unwrap()
}

fn get_score(plain: &Vec<u8>) -> i32 {
    let scores = plain.iter().map(|&c| letter_rank(c));
    let tot:f32 = scores.sum();
    return tot as i32;
}

fn letter_rank(c: u8) -> f32 {
    match c as char {
        'e' | 'E' => 12.702,
        't' | 'T' => 9.256,
        'a' | 'A' => 8.167,
        'o' | 'O' => 7.507,
        'i' | 'I' => 6.966,
        'n' | 'N' => 6.749,
        's' | 'S' => 6.327,
        'h' | 'H' => 6.094,
        'r' | 'R' => 5.987,
        'w' | 'W' => 5.370,
        'd' | 'D' => 4.253,
        'l' | 'L' => 4.025,
        'y' | 'Y' => 3.978,
        'k' | 'K' => 3.872,
        'c' | 'C' => 2.782,
        'u' | 'U' => 2.758,
        'm' | 'M' => 2.406,
        'f' | 'F' => 2.228,
        'g' | 'G' => 2.015,
        'p' | 'P' => 1.929,
        'b' | 'B' => 1.492,
        'v' | 'V' => 0.978,
        'j' | 'J' => 0.153,
        'x' | 'X' => 0.150,
        'q' | 'Q' => 0.095,
        'z' | 'Z' => 0.074,
        ' ' => 2.0,
        '0'...'9' => 1.0,
        _ => -2.0,
    }
}