use aes::Aes128;
use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit, generic_array::GenericArray};

pub fn decrypt_aes_ecb(data: &[u8], key: &[u8]) -> Vec<u8> {
    if data.len() % 16 != 0 {
        return data.to_vec();
    }

    let key = GenericArray::from_slice(key);
    let cipher = Aes128::new(key);
    let mut decrypted = data.to_vec();

    for chunk in decrypted.chunks_mut(16) {
        let block = GenericArray::from_mut_slice(chunk);
        cipher.decrypt_block(block);
    }

    decrypted
}

pub fn encrypt_aes_ecb(data: &[u8], key: &[u8]) -> Vec<u8> {
    if data.len() != 16 {
        return data.to_vec();
    }

    let key = GenericArray::from_slice(key);
    let cipher = Aes128::new(key);
    let mut encrypted = data.to_vec();

    let block = GenericArray::from_mut_slice(&mut encrypted);
    cipher.encrypt_block(block);

    encrypted
}

pub fn procesa_rx(strg: Vec<u8>, key: &[u8]) -> (Vec<u8>, bool) {
    if strg.len() != 32 {
        return (strg, false);
    }

    let decrypted = decrypt_aes_ecb(&strg, key);

    if decrypted.first() == Some(&b'B') {
        let cant = decrypted[1] as usize - 6;
        if cant + 7 <= decrypted.len() {
            let mut result = vec![b'@'];
            result.extend_from_slice(&decrypted[6..6 + cant]);
            return (result, true);
        }
    }

    (strg, false)
}

pub fn procesa_tx(strg: Vec<u8>, key: &[u8], encrypted: bool) -> Vec<u8> {
    if !encrypted || strg.is_empty() {
        return strg;
    }

    let mut block = [b' '; 16];
    block[0] = b'B';
    block[1] = (strg.len() + 5) as u8;
    
    // Timer based part (simplified, maybe use actual timestamp)
    let now = chrono::Local::now();
    let tmr = (now.timestamp() / 60) as u32 & 0xFFFF;
    block[2] = (tmr >> 8) as u8;
    block[3] = (tmr & 0xFF) as u8;
    block[4] = 0;
    block[5] = 0;

    // Copy data skipping '@'
    let data_to_copy = &strg[1..];
    let copy_len = std::cmp::min(data_to_copy.len(), 10); // max 10 chars to fit in 16 bytes block
    block[6..6 + copy_len].copy_from_slice(&data_to_copy[..copy_len]);

    encrypt_aes_ecb(&block, key)
}
