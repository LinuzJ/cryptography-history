use crate::{IndexForChar, ToChar, NOTCHES, ROTORS};

#[derive(Clone, Debug)]
pub struct Rotor {
    mapping: Vec<char>,
    inverse: Vec<char>,
    notches: Vec<usize>,
    pub offset: usize,
    pub key_setting: usize,
    pub ring_setting: usize,
}

impl Rotor {
    pub fn new(mapping: &str, notches: &str, key: char, ring: char) -> Rotor {
        let mapping: Vec<char> = mapping.chars().collect();

        if mapping.len() != 26 {
            panic!("Rotor mappings must be 26 characters long.");
        }

        let mut inverse = vec!['A'; 26];

        for (i, &c) in mapping.iter().enumerate() {
            inverse[c.index()] = i.to_char();
        }

        Rotor {
            mapping,
            inverse,
            notches: notches.chars().map(|c| c.index()).collect(),
            offset: key.index(),
            key_setting: key.index(),
            ring_setting: ring.index(),
        }
    }

    pub fn from_config(num: usize, key: char, ring: char) -> Rotor {
        Rotor::new(ROTORS[num - 1], NOTCHES[num - 1], key, ring)
    }

    fn map(&self, c: char, mapping: &[char]) -> char {
        let offset = 26 + self.offset - self.ring_setting;
        let index = mapping[(c.index() + offset) % 26].index();
        (52 + index - offset).to_char()
    }

    pub fn substitute(&self, c: char) -> char {
        self.map(c, &self.mapping)
    }

    pub fn invert(&self, c: char) -> char {
        self.map(c, &self.inverse)
    }

    pub fn advance(&mut self) {
        self.offset = (self.offset + 1) % 26;
    }

    pub fn on_notches(&self) -> bool {
        self.notches.iter().any(|&n| n == self.offset)
    }
}
