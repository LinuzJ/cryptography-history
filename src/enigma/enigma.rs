use super::rotor::Rotor;

#[derive(Clone, Debug)]
pub struct Enigma {
    slow: Rotor,
    mid: Rotor,
    fast: Rotor,
    // reflector: Reflector,
    // plugboard: Plugboard,
}

impl Enigma {
    pub fn new(rotors: &str, keys: &str, rings: &str, _reflector: char, _plugboard: &str) -> Enigma {
        let rotors: Vec<usize> = rotors
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|n| n as usize)
            .collect();

        if rotors.len() != 3 {
            panic!("Exactly 3 rotors must be given.");
        }

        let keys: Vec<char> = keys.chars().collect();
        let rings: Vec<char> = rings.chars().collect();

        Enigma {
            slow: Rotor::from_config(rotors[0], keys[0], rings[0]),
            mid: Rotor::from_config(rotors[1], keys[1], rings[1]),
            fast: Rotor::from_config(rotors[2], keys[2], rings[2]),
            // reflector: Reflector::from_enigma(reflector),
            // plugboard: Plugboard::new(plugboard),
        }
    }
}
