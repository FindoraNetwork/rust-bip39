use argon2::{self, Config};

// Method borrowed from rust-bitcoin's endian module.
/// PBKDF2-HMAC-SHA512 implementation using bitcoin_hashes.
pub(crate) fn argon(passphrase: &[u8], salt: &[u8]) -> Vec<u8> {
    argon2::hash_raw(passphrase, salt, &Config::default()).expect(
        "Failure 
    hashing password",
    )
}
