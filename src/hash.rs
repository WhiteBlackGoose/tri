use std::{
    fmt::{Display, Write},
    path::Path,
};

use sha256::{self};

use crate::error::TRIError;

type Sha256 = [u8; 32];

#[derive(Clone, Copy, Ord, Eq, PartialOrd, PartialEq, Hash, Debug)]
pub struct Hash {
    pub sha256: Sha256,
}

impl Hash {
    pub fn new(path: &Path) -> Result<Hash, TRIError> {
        let mut r: Sha256 = [0; 32];
        let sha = sha256::try_digest(path)
            .map_err(|_| TRIError::HashFromFileError(path.to_path_buf()))?;
        for i in 0..32 {
            r[i] = sha.as_bytes()[i];
        }
        Ok(Hash { sha256: r })
    }

    pub fn from_string(sha: &str) -> Result<Hash, TRIError> {
        if sha.len() != 32 {
            return Err(TRIError::HashFromStringError(String::from(sha)));
        }
        let mut r: Sha256 = [0; 32];
        for i in 0..32 {
            r[i] = sha.as_bytes()[i];
        }
        Ok(Hash { sha256: r })
    }

    pub fn eq(&self, other: &Hash) -> bool {
        for (a, b) in self.sha256.iter().zip(other.sha256) {
            if *a != b {
                return false;
            }
        }
        true
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.sha256 {
            f.write_char(c as char).expect("Error");
        }
        Ok(())
    }
}

// Write test for this file

#[cfg(test)]
mod tests {
    use super::Hash;

    #[test]
    fn test_hash_from_string() {
        let h1 = Hash::from_string("01234567890123456789012345678901").unwrap();
        let h2 = Hash::from_string("01234567890123456789012345678901").unwrap();
        let h3 = Hash::from_string("01234567890123456789012345678902").unwrap();
        assert!(h1.eq(&h2));
        assert!(!h1.eq(&h3));
    }

    #[test]
    #[should_panic]
    fn test_panic_hash_from_string() {
        let h1 = Hash::from_string("too_short").unwrap();
    }
}
