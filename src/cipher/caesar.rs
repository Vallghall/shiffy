use super::CipherCommand;

pub struct Caesar();

impl Caesar {
    pub const SHIFT: u8 = 3;
    pub const ALPHABET_LENGTH: u8 = 26;

    pub fn shift(sym: char) -> char {
        match sym {
            'a'..='z' => {
                let s = sym as u8 + Caesar::SHIFT;
                (s - if s > 'z' as u8 {Caesar::ALPHABET_LENGTH} else { 0 }) as char
            }

            'A'..='Z' => {
                let s = sym as u8 + Caesar::SHIFT;
                (s - if s > 'Z' as u8 {Caesar::ALPHABET_LENGTH} else { 0 }) as char
            }

            other => other
        }
    }

    pub fn unshift(sym: char) -> char {
        match sym {
            'a'..='z' => {
                let s = sym as u8 - Caesar::SHIFT;
                (s + if s < 'a' as u8 {Caesar::ALPHABET_LENGTH} else { 0 }) as char
            }

            'A'..='Z' => {
                let s = sym as u8 - Caesar::SHIFT;
                (s + if s < 'A' as u8 {Caesar::ALPHABET_LENGTH} else { 0 }) as char
            }

            other => other
        }
    }
}

impl CipherCommand for Caesar {
    fn cipher(&self, text: &str) -> String {
        text.chars().map(Caesar::shift).collect()
    }

    fn decipher(&self, text: &str) -> String {
        text.chars().map(Caesar::unshift).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn caesar_cipher() {
        let expected: &'static str = "def !@#$ abc DEF %^&* ABC";
        let result = Caesar{}.cipher("abc !@#$ xyz ABC %^&* XYZ");
        if result != expected {
            panic!("want: {}\ngot:  {}", expected, result)
        }
    }
    #[test]
    fn caesar_decipher() {
        let expected: &'static str = "abc !@#$ xyz ABC %^&* XYZ";
        let result = Caesar{}.decipher("def !@#$ abc DEF %^&* ABC");
        if result != expected {
            panic!("want: {}\ngot:  {}", expected, result)
        }
    }

}