use std::collections::HashMap;

use crate::{errors::{AppError, AppResult}, words::MAXLEN};

// why am i so OOP
pub(crate) struct Matcher<'a> {
    dict: &'a HashMap<u16, char>,
    reverse_dict: &'a HashMap<char, u16>
}

impl<'a> Matcher<'a> {
    pub fn new(dict: &'a HashMap<u16, char>, reverse_dict: &'a HashMap<char, u16>) -> Self {
        Self { dict, reverse_dict }
    }
    pub fn decipher(&self, input: &str) -> AppResult<String> {
        let mut result = String::new();
        let len = input.len();
        let mut i = 0;
        'a: while i < len {
            for len in 1..=MAXLEN {
                if let Ok(key) = &input[i..i+len].parse::<u16>() {
                    if let Some(c) = self.get_char(*key) {
                        i += len;
                        result.push(c);
                        continue 'a;
                    }
                } else {
                    return Err(AppError::ParseError("Invalid character found!".into()))
                }
            }
            // No match was found
            return Err(AppError::ParseError("Invalid sequence found!".into()));
        }
        Ok(result)

    }
    
    pub fn cipher(&self, input: &str) -> AppResult<String> {
        let mut result = String::new();
        for c in input.chars() {
            if let Some(num) = self.reverse_dict.get(&c) {
                result.push_str(&num.to_string())
            } else {
                return Err(AppError::ParseError("Character outside defined codes found!".into()))
            }
        }
        Ok(result)

    }

    fn get_char(&self, key: u16) -> Option<char> {
        self.dict.get(&key).copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::words::{get_dict_map, get_dict_reverse_map};
    use super::Matcher;

    #[test]
    fn check_cipher() {
        let stream = "fuck you";
        let correct = "1218670321965313618";
        let dict = get_dict_map();
        let reverse_dict = get_dict_reverse_map();
        let matcher = Matcher::new(&dict, &reverse_dict);
        assert_eq!(matcher.cipher(stream).unwrap(), correct);
    }

    #[test]
    fn check_decipher() {
        let stream = "1218670321965313618";
        let correct = "fuck you";
        let dict = get_dict_map();
        let reverse_dict = get_dict_reverse_map();
        let matcher = Matcher::new(&dict, &reverse_dict);
        assert_eq!(matcher.decipher(stream).unwrap(), correct);
    }
}
