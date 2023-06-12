use std::{
    collections::hash_map::DefaultHasher,
    fmt::Display,
    hash::{Hash, Hasher},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Sentence {
    data: String,
    // Need something else ?
}

impl Sentence {
    pub fn new(data: String) -> Sentence {
        Sentence { data }
    }

    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.data.hash(&mut hasher);
        hasher.finish()
    }
}

impl Display for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[cfg(test)]
pub mod tests {
    use std::{assert_eq, assert_ne};

    use super::*;
    #[test]
    fn test_hash() {
        let s1 = Sentence::new("toto".to_owned());
        let s2 = Sentence::new("tata".to_owned());
        let s3 = Sentence::new("toto".to_owned());
        assert_eq!(s1.hash(), s3.hash());
        assert_ne!(s1.hash(), s2.hash());
    }
}
