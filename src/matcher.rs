use std::collections::HashMap;

use crate::{
    errors::{AppError, AppResult},
    words::Trie,
};

// why am i so OOP patterned™
// TODO: idk if borrowed fields is a good idea since this Matcher would stand alone anyways
/// Cipher and deciphering engine.
pub(crate) struct Matcher<'a> {
    /// Prefix tree of the dictionary
    trie: &'a Trie,
    reverse_dict: &'a HashMap<char, u16>,
}

impl<'a> Matcher<'a> {
    /// Create a new instance of this whatever text cipher and deciphering tool. (idk why i named
    /// it matcher..)
    pub fn new(trie: &'a Trie, reverse_dict: &'a HashMap<char, u16>) -> Self {
        Self { trie, reverse_dict }
    }

    /// Decipher a given string input.
    pub fn decipher(&self, input: &str) -> AppResult<String> {
        let mut result = String::new();
        let mut idx: usize = 0; // the index of node we're in rn. initially set to the root
        for c in input.chars() {
            if c == '\n' || c == ' ' {
                continue;
            }

            // convert the character to a digit
            let n = if let Some(n) = c.to_digit(10) {
                n as usize
            } else {
                return Err(AppError::ParseError("Invalid character found!".into()));
            };

            // traverse
            if let Some(i) = self.trie.nodes[idx].children[n] {
                idx = i
            } else {
                // if we're still somehow traversing then there's something seriously wrong going
                // on with the trie (unlikely) or the user input. It's 99% not our fault so like
                // just blame the user I guess lol
                return Err(AppError::ParseError("Invalid sequence found!".into()));
            }
            // this is a leaf!
            if let Some(c) = self.trie.nodes[idx].letter {
                result.push(c);
                // reset (go to root again)
                idx = 0;
            }
        }
        Ok(result)
    }

    pub fn cipher(&self, input: &str) -> AppResult<String> {
        let mut result = String::new();
        for c in input.chars() {
            if let Some(num) = self.reverse_dict.get(&c) {
                result.push_str(&num.to_string())
            } else {
                return Err(AppError::ParseError(
                    "Character outside defined codes found!".into(),
                ));
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::Matcher;
    use crate::words::{get_encoder_dict_map, Trie, DICTIONARY};

    #[test]
    fn check_cipher() {
        let stream = "fuck you";
        let correct = "1218670321965313618";
        let trie = Trie::from_dict(&DICTIONARY);
        let reverse_dict = get_encoder_dict_map();
        let matcher = Matcher::new(&trie, &reverse_dict);
        assert_eq!(matcher.cipher(stream).unwrap(), correct);
    }

    #[test]
    fn check_invalid_cipher() {
        let stream = "*#^&!*#@&@#^*!!^&#!*^!#";
        let trie = Trie::from_dict(&DICTIONARY);
        let reverse_dict = get_encoder_dict_map();
        let matcher = Matcher::new(&trie, &reverse_dict);
        assert!(matcher.decipher(stream).is_err());
    }

    #[test]
    fn check_decipher() {
        let stream = "1218670321965313618";
        let correct = "fuck you";
        let trie = Trie::from_dict(&DICTIONARY);
        let reverse_dict = get_encoder_dict_map();
        let matcher = Matcher::new(&trie, &reverse_dict);
        assert_eq!(matcher.decipher(stream).unwrap(), correct);
    }

    #[test]
    fn check_invalid_decipher() {
        let stream = "99835";
        let trie = Trie::from_dict(&DICTIONARY);
        let reverse_dict = get_encoder_dict_map();
        let matcher = Matcher::new(&trie, &reverse_dict);
        assert!(matcher.decipher(stream).is_err());
    }
}
