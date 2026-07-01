//! Cryptography Engine - Ghost's Secrets Stay Secret
//!
//! Provides:
//! - Encryption/Decryption
//! - Hashing
//! - Key generation
//! - Secure random numbers
//! - Password hashing
//! - Message authentication

use anyhow::Result;
use ring::aead::{AES_256_GCM, Nonce, SealingKey, OpeningKey, UnboundKey};
use ring::digest::{digest, SHA256, SHA512};
use ring::hmac;
use ring::pbkdf2;
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{Ed25519KeyPair, KeyPair};
use std::num::NonZeroU32;

/// Cryptographic engine
pub struct CryptoEngine {
    rng: SystemRandom,
    key: Option<[u8; 32]>,
}

/// Encryption result with nonce and ciphertext
#[derive(Debug, Clone)]
pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
}

/// Key types
#[derive(Debug, Clone)]
pub enum KeyType {
    Symmetric([u8; 32]),
    Asymmetric {
        public: Vec<u8>,
        private: Vec<u8>,
    },
}

impl CryptoEngine {
    /// Create a new crypto engine
    pub fn new() -> Self {
        Self {
            rng: SystemRandom::new(),
            key: None,
        }
    }

    /// Generate a random key
    pub fn generate_key(&self) -> Result<[u8; 32]> {
        let mut key = [0u8; 32];
        self.rng.fill(&mut key)?;
        Ok(key)
    }

    /// Set the encryption key
    pub fn set_key(&mut self, key: [u8; 32]) {
        self.key = Some(key);
    }

    /// Encrypt data using AES-256-GCM
    pub fn encrypt(&self, data: &[u8]) -> Result<EncryptedData> {
        let key = self.key.ok_or_else(|| anyhow::anyhow!("No encryption key set"))?;
        
        // Generate a random nonce
        let mut nonce = [0u8; 12];
        self.rng.fill(&mut nonce)?;

        // Create sealing key
        let unbound_key = UnboundKey::new(&AES_256_GCM, &key)?;
        let sealing_key = SealingKey::new(unbound_key);

        // Encrypt
        let mut in_out = data.to_vec();
        let nonce = Nonce::assume_unique_for_key(nonce);
        sealing_key.seal_in_place_separate_nonce(nonce, &[], &mut in_out)?;

        Ok(EncryptedData {
            ciphertext: in_out,
            nonce: nonce.as_ref().try_into()?,
        })
    }

    /// Decrypt data using AES-256-GCM
    pub fn decrypt(&self, encrypted: &EncryptedData) -> Result<Vec<u8>> {
        let key = self.key.ok_or_else(|| anyhow::anyhow!("No encryption key set"))?;

        // Create opening key
        let unbound_key = UnboundKey::new(&AES_256_GCM, &key)?;
        let opening_key = OpeningKey::new(unbound_key);

        // Decrypt
        let mut ciphertext = encrypted.ciphertext.clone();
        let nonce = Nonce::assume_unique_for_key(encrypted.nonce);
        let plaintext = opening_key.open_in_place(nonce, &[], &mut ciphertext)?;

        Ok(plaintext.to_vec())
    }

    /// Hash data using SHA-256
    pub fn hash_sha256(&self, data: &[u8]) -> [u8; 32] {
        let digest = digest(&SHA256, data);
        let mut result = [0u8; 32];
        result.copy_from_slice(digest.as_ref());
        result
    }

    /// Hash data using SHA-512
    pub fn hash_sha512(&self, data: &[u8]) -> [u8; 64] {
        let digest = digest(&SHA512, data);
        let mut result = [0u8; 64];
        result.copy_from_slice(digest.as_ref());
        result
    }

    /// Create HMAC using SHA-256
    pub fn hmac_sha256(&self, key: &[u8], data: &[u8]) -> [u8; 32] {
        let key = hmac::Key::new(hmac::HMAC_SHA256, key);
        let signature = hmac::sign(&key, data);
        let mut result = [0u8; 32];
        result.copy_from_slice(signature.as_ref());
        result
    }

    /// Verify HMAC
    pub fn verify_hmac(&self, key: &[u8], data: &[u8], signature: &[u8]) -> bool {
        let key = hmac::Key::new(hmac::HMAC_SHA256, key);
        hmac::verify(&key, data, signature).is_ok()
    }

    /// Password hashing using PBKDF2
    pub fn hash_password(&self, password: &[u8], salt: &[u8]) -> Result<[u8; 32]> {
        let mut hash = [0u8; 32];
        let iterations = NonZeroU32::new(100_000).unwrap();
        
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            iterations,
            salt,
            password,
            &mut hash,
        );
        
        Ok(hash)
    }

    /// Verify password
    pub fn verify_password(&self, password: &[u8], salt: &[u8], hash: &[u8]) -> bool {
        let iterations = NonZeroU32::new(100_000).unwrap();
        
        pbkdf2::verify(
            pbkdf2::PBKDF2_HMAC_SHA256,
            iterations,
            salt,
            password,
            hash,
        ).is_ok()
    }

    /// Generate Ed25519 key pair
    pub fn generate_ed25519_keypair(&self) -> Result<Vec<u8>> {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)?;
        Ok(pkcs8_bytes.as_ref().to_vec())
    }

    /// Sign data with Ed25519
    pub fn sign_ed25519(&self, keypair: &[u8], data: &[u8]) -> Result<Vec<u8>> {
        let keypair = Ed25519KeyPair::from_pkcs8(keypair)?;
        let signature = keypair.sign(data);
        Ok(signature.as_ref().to_vec())
    }

    /// Verify Ed25519 signature
    pub fn verify_ed25519(&self, public_key: &[u8], data: &[u8], signature: &[u8]) -> bool {
        use ring::signature::UnparsedPublicKey;
        
        let public_key = UnparsedPublicKey::new(&ring::signature::ED25519, public_key);
        public_key.verify(data, signature).is_ok()
    }

    /// Generate secure random bytes
    pub fn random_bytes(&self, len: usize) -> Result<Vec<u8>> {
        let mut bytes = vec![0u8; len];
        self.rng.fill(&mut bytes)?;
        Ok(bytes)
    }

    /// Generate a random number in range
    pub fn random_range(&self, min: u64, max: u64) -> Result<u64> {
        let bytes = self.random_bytes(8)?;
        let mut arr = [0u8; 8];
        arr.copy_from_slice(&bytes);
        let val = u64::from_le_bytes(arr);
        Ok(min + (val % (max - min + 1)))
    }

    /// Generate UUID v4
    pub fn generate_uuid(&self) -> Result<String> {
        let mut bytes = [0u8; 16];
        self.rng.fill(&mut bytes)?;
        
        // Set version (4) and variant (RFC 4122)
        bytes[6] = (bytes[6] & 0x0f) | 0x40;
        bytes[8] = (bytes[8] & 0x3f) | 0x80;
        
        Ok(format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5],
            bytes[6], bytes[7],
            bytes[8], bytes[9],
            bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
        ))
    }
}

/// Secure memory utilities
pub struct SecureMemory;

impl SecureMemory {
    /// Zero out memory
    pub fn zeroize(data: &mut [u8]) {
        data.fill(0);
    }

    /// Securely compare two values (constant time)
    pub fn secure_compare(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        let mut result = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }
        result == 0
    }
}