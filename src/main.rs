use crate::set1::challenge1;
use crate::set1::challenge2;
use crate::set1::challenge3;
use crate::set1::challenge4;
use crate::set1::challenge5;

mod crypto;
mod encoding;
mod set1;

fn main() {
    challenge1::run();
    challenge2::run();
    challenge3::run();
    challenge4::run();
    challenge5::run();
}
