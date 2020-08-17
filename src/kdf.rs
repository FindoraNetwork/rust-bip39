use argon2::{self, Config};

pub(crate) fn argon(passphrase: &[u8], salt: &[u8]) -> Vec<u8> {
    argon2::hash_raw(passphrase, salt, &Config::default()).expect(
        "Failure 
    hashing password",
    )
}
