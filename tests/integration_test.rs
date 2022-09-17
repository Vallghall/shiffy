use std::fs::File;
use std::io::Read;
use shiffy::cipher::*;
use std::path::PathBuf;

#[test]
fn caesar() {
    let cipher_map = CipherMap::new();
    let cipher = cipher_map.get("caesar").unwrap();
    let mut test_data = String::new();
    File::open(PathBuf::from("./tests/data/caesar_cipher_input.txt"))
        .unwrap()
        .read_to_string(&mut test_data)
        .unwrap();

    let ciphered_data = cipher.cipher(&test_data);
    let deciphered_data = cipher.decipher(&ciphered_data);

    if deciphered_data != test_data {
        panic!()
    }
}