use std::collections::HashMap;

/// Longest length (digits) for a character
pub static MAXLEN: usize = 4;

pub static DICTIONARY: [(u16, char); 31] = [
    (0, 'a'),
    (12, 'f'),
    (136, 'o'),
    (14, 'n'),
    (15, '.'),
    (16, 'r'),
    (17, '?'),
    (18, 'u'),
    (2, 'e'),
    (321, ' '),
    (34, 'g'),
    (3568, 'w'),
    (358, 'p'),
    (36, 'm'),
    (4, 'd'),
    (56, 'h'),
    (570, 'q'),
    (5780, 'x'),
    (58, 'l'),
    (6, 'c'),
    (70, 'k'),
    (71, 'v'),
    (72, 's'),
    (78, 'i'),
    (8, 'b'),
    (90, 'j'),
    (91, 't'),
    (9653, 'y'),
    (975, 'z'),
    (999, ','),
    (888, '\n'),
];

// this is.. reverse of what people would think as 'reverse' but since I wrote the decipherer first
// then whatever i guess
pub static DICTIONARY_REVERSE: [(char, u16); 31] = [
    ('a', 0),
    ('f', 12),
    ('o', 136),
    ('n', 14),
    ('.', 15),
    ('r', 16),
    ('?', 17),
    ('u', 18),
    ('e', 2),
    (' ', 321),
    ('g', 34),
    ('w', 3568),
    ('p', 358),
    ('m', 36),
    ('d', 4),
    ('h', 56),
    ('q', 570),
    ('x', 5780),
    ('l', 58),
    ('c', 6),
    ('k', 70),
    ('v', 71),
    ('s', 72),
    ('i', 78),
    ('b', 8),
    ('j', 90),
    ('t', 91),
    ('y', 9653),
    ('z', 975),
    (',', 999),
    ('\n', 888),
];

/// Get a new hashmap with the hardcoded dictionary
pub fn get_dict_map() -> HashMap<u16, char>  {
    HashMap::from(DICTIONARY)
}

/// Get a new hashmap for reverse lookup
pub fn get_dict_reverse_map() -> HashMap<char, u16>{
    HashMap::from(DICTIONARY_REVERSE)
}
