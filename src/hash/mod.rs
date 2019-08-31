use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::ripemd160::Ripemd160;

pub fn get_sha256(bytes: &[u8; 65]) -> String {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.input(bytes);

    // read hash digest
    let hex = hasher.result_str();
    hex
}

// TODO pass bytes here instead of string
pub fn get_ripemd(s: &str) -> String {
    let mut hasher = Ripemd160::new();

    hasher.input_str(s);

    let hex = hasher.result_str();
    hex
}