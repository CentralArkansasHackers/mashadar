use aes::Aes128;
use ctr::cipher::{NewCipher, StreamCipher};
use ctr::Ctr128BE;

pub fn encrypt_shellcode(shellcode: &[u8], key: &[u8; 16], nonce: &[u8; 16]) -> Vec<u8> {
    let mut data = shellcode.to_vec();
    let mut cipher = Ctr128BE::<Aes128>::new(key.into(), nonce.into());
    cipher.apply_keystream(&mut data);
    data
}

pub fn decrypt_shellcode(shellcode: &[u8], key: &[u8; 16], nonce: &[u8; 16]) -> Vec<u8> {
    encrypt_shellcode(shellcode, key, nonce) // CTR is symmetric
}
