use rand::distributions::{Alphanumeric, DistString};
use ring::digest::{Context, SHA256};

const SALT_LENGTH: usize = 4;

pub fn salt() -> Vec<u8> {
    // salts are 32 bit long
    let sample = Alphanumeric.sample_string(&mut rand::thread_rng(), SALT_LENGTH);
    let bytes = sample.as_bytes();
    Vec::from(bytes)
}

pub fn salted_password_hash_sha256(salt: &Vec<u8>, password: &str) -> Vec<u8> {
    let mut ctx = Context::new(&SHA256);
    let vec = Vec::from([&salt[..], &password.as_bytes()[..]].concat());

    ctx.update(&vec);
    let digest = ctx.finish();
    let digest_vec = Vec::from(digest.as_ref());

    [&salt[..], &digest_vec[..]].concat()
}

pub fn base64_encoded_salted_password_hash_sha256(salt: &Vec<u8>, password: &str) -> String {
    let salted = salted_password_hash_sha256(salt, password);
    rbase64::encode(salted.as_slice())
}
