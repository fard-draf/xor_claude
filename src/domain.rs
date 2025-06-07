use crate::error::*;
use secrecy::{
    zeroize::{Zeroize, ZeroizeOnDrop},
    ExposeSecret, SecretBox, SecretString,
};

pub struct Plaintext {
    content: SecretString,
    length: usize,
}

impl Plaintext {
    pub fn from_str(plaintxt: &str) -> Result<Self> {
        if plaintxt.is_empty() {
            return Err(AppError::EmptyFile);
        }

        let cleaned_text = plaintxt.trim();
        let length = cleaned_text.len();

        Ok(Self {
            content: plaintxt.into(),
            length,
        })
    }

    pub fn reveal(&self) -> String {
        self.content.expose_secret().to_string()
    }

    pub fn is_big(&self) -> bool {
        self.length > 3000
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl Zeroize for Plaintext {
    fn zeroize(&mut self) {
        self.content.zeroize();
        self.length = 0;
    }
}

impl ZeroizeOnDrop for Plaintext {}

pub struct Ciphertext {
    data: Vec<u8>,
    length: usize,
}

impl Ciphertext {
    pub fn new(data: Vec<u8>) -> Self {
        let length = data.len();
        Self { data, length }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl Zeroize for Ciphertext {
    fn zeroize(&mut self) {
        self.data.zeroize();
        self.length = 0;
    }
}

impl ZeroizeOnDrop for Ciphertext {}

pub struct Key(SecretBox<[u8]>);

impl Key {
    pub fn from_str(key: &str) -> Result<Self> {
        if key.len() != 25 || !key.chars().all(|c| c.is_ascii()) {
            return Err(AppError::UnvalidKey);
        }

        let key_bytes: Box<[u8]> = key.as_bytes().into();
        let secret_box = SecretBox::new(key_bytes);

        Ok(Self(secret_box))
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.expose_secret()
    }
}

impl Zeroize for Key {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl ZeroizeOnDrop for Key {}
