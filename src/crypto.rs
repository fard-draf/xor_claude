use crate::domain::{Ciphertext, Key, Plaintext};
use crate::error::{AppError, Result};
use secrecy::ExposeSecret;

pub fn compress(plaintext: &Plaintext) -> Result<Vec<u8>> {
    let content = plaintext.reveal();
    zstd::encode_all(content.as_bytes(), 3).map_err(|_| AppError::CompressionError)
}

pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
    zstd::decode_all(data).map_err(|_| AppError::DecompressionError)
}

pub fn xor_encrypt(data: &[u8], key: &Key) -> Ciphertext {
    let key_bytes = key.as_bytes();
    let encrypted: Vec<u8> = data
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key_bytes[i % key_bytes.len()])
        .collect();

    Ciphertext::new(encrypted)
}

pub fn xor_decrypt(ciphertext: &Ciphertext, key: &Key) -> Vec<u8> {
    // XOR est symétrique, donc on utilise la même fonction
    let key_bytes = key.as_bytes();
    ciphertext
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key_bytes[i % key_bytes.len()])
        .collect()
}

pub fn process_file(content: &str, key: &Key) -> Result<Ciphertext> {
    let plaintext = Plaintext::from_str(content)?;
    let compressed = compress(&plaintext)?;
    Ok(xor_encrypt(&compressed, key))
}

pub fn reverse_process(ciphertext: &Ciphertext, key: &Key) -> Result<String> {
    let decrypted = xor_decrypt(ciphertext, key);
    let decompressed = decompress(&decrypted)?;
    String::from_utf8(decompressed).map_err(|_| AppError::UnvalidFormat)
}
