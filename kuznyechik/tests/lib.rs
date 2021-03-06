#![no_std]
#[macro_use]
extern crate crypto_tests;
extern crate kuznyechik;

use crypto_tests::block_cipher::{BlockCipherTest, encrypt_decrypt};

#[test]
fn kuznyechik() {
    let tests = new_block_cipher_tests!("1");
    encrypt_decrypt::<kuznyechik::Kuznyechik>(&tests);
}
