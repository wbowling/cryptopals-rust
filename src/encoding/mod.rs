pub trait UnHexExt {
    fn unhex(&self) -> Vec<u8>;
}

impl UnHexExt for &str {
    fn unhex(&self) -> Vec<u8> {
        hex::decode(&self).expect("Invalid hex string")
    }
}

pub trait HexExt {
    fn hex(&self) -> String;
}

impl HexExt for &[u8] {
    fn hex(&self) -> String {
        hex::encode(&self)
    }
}

impl HexExt for Vec<u8> {
    fn hex(&self) -> String {
        hex::encode(&self)
    }
}