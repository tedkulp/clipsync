use sha2::{Sha256, Digest};

/// Hash a shared secret to create a room identifier
pub fn hash_secret(secret: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(secret.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_secret() {
        let secret = "my-secret-key";
        let hash1 = hash_secret(secret);
        let hash2 = hash_secret(secret);
        
        // Same secret should produce same hash
        assert_eq!(hash1, hash2);
        
        // Different secret should produce different hash
        let hash3 = hash_secret("different-secret");
        assert_ne!(hash1, hash3);
        
        // Hash should be 64 characters (256 bits in hex)
        assert_eq!(hash1.len(), 64);
    }
}
