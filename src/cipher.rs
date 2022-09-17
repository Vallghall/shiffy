mod caesar;

use std::collections::HashMap;

/// CipherMap contains CipherCommands
/// used in ciphering and deciphering
pub struct CipherMap<'a> {
    funcs: HashMap<&'a str, Box<dyn CipherCommand>>
}

impl CipherMap<'_> {
    pub fn new() -> Self {
        CipherMap {
            funcs: HashMap::from([
                ("caesar", Box::new(caesar::Caesar{}) as Box<dyn CipherCommand>)
            ])
        }
    }

    pub fn get(&self, cipher: &str) -> Option<&Box<dyn CipherCommand>> {
        match self.funcs.get(cipher) {
            Some(func) => Some(func),
            None => None
        }
    }
}
/// CipherCommand describes method for cipher structs
pub trait CipherCommand {
    fn cipher(&self, text: &str) -> String;
    fn decipher(&self, text: &str) -> String;
}

