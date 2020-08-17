use argon2::{self, Config};

// Method borrowed from rust-bitcoin's endian module.
#[inline]
fn u32_to_array_be(val: u32) -> [u8; 4] {
    debug_assert_eq!(::std::mem::size_of::<u32>(), 4); // size_of isn't a constfn in 1.22

    let mut res = [0; 4];
    for i in 0..4 {
        res[i] = ((val >> (4 - i - 1) * 8) & 0xff) as u8;
    }
    res
}

#[inline]
fn xor(res: &mut [u8], salt: &[u8]) {
    debug_assert!(salt.len() >= res.len(), "length mismatch in xor");

    res.iter_mut().zip(salt.iter()).for_each(|(a, b)| *a ^= b);
}

/// PBKDF2-HMAC-SHA512 implementation using bitcoin_hashes.
pub(crate) fn argon(passphrase: &[u8], salt: &[u8]) -> Vec<u8> {
    argon2::hash_raw(passphrase, salt, Config::default()).expect(
        "Failure 
    hashing password",
    )
}
