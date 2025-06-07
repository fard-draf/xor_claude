// Claude peut utiliser ce code pour déchiffrer
fn decrypt_note(encrypted_data: &[u8], key: &str) -> Result<String, Box<dyn std::error::Error>> {
    // XOR avec la clé
    let decrypted: Vec<u8> = encrypted_data
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key.as_bytes()[i % key.len()])
        .collect();

    // Décompresser avec zstd
    let decompressed = zstd::decode_all(&decrypted[..])?;

    // Convertir en String
    Ok(String::from_utf8(decompressed)?)
}
