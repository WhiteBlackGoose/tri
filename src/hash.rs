use std::{path::Path, fmt::{Display, Write}};

use sha256::{self};

use crate::error::TRIError;

type Sha256 = [u8; 32];

#[derive(Clone, Copy)]
#[derive(Ord, Eq, PartialOrd, PartialEq)]
#[derive(Hash)]
#[derive(Debug)]
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

    pub fn from_string(sha: &str) -> Hash {
        assert_eq!(sha.len(), 32);
        let mut r: Sha256 = [0; 32];
        for i in 0..32 {
            r[i] = sha.as_bytes()[i];
        }
        Hash { sha256: r }
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
