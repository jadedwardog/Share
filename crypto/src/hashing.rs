use blake3;

pub fn hash(data: &[u8]) -> String {
    blake3::hash(data).to_hex().to_string()
}
