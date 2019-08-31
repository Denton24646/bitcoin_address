use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn get_sha256(bytes: &[u8; 65]) -> String {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.input(bytes);

    // read hash digest
    let hex = hasher.result_str();
    hex
}
