use serde_json;

use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub const NUM_LETTERS: usize = 5;

pub type Word = [u8; NUM_LETTERS];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Tile {
    Correct,
    Misplaced,
    Wrong,
}

pub type WordMask = [u8; 26];
pub type Pattern = [Tile; NUM_LETTERS];

pub fn pat_to_str(pat: Pattern) -> String {
    use Tile::*;
    let mut s = String::new();
    for p in pat {
        let c = match p {
            Correct => 'c',
            Misplaced => 'm',
            Wrong => 'x',
        };
        s.push(c);
    }
    s
}

pub fn pat_to_int(pat: Pattern) -> u8 {
    let mut int = 0;
    use Tile::*;
    for i in 0..NUM_LETTERS {
        let p = pat[i];
        let ii = match p {
            Wrong => 0,
            Misplaced => 1,
            Correct => 2,
        };
        int += 3_u8.pow(i as u32)*ii;
    }
    int
}

pub fn int_to_pat(int: u8) -> Pattern {
    let mut pat = [Tile::Wrong; NUM_LETTERS];
    let mut x = int;
    for i in 0..NUM_LETTERS {
        use Tile::*;
        let r = x % 3;
        x = (x - r)/3;
        pat[i] = match r {
            0 => Wrong,
            1 => Misplaced,
            2 => Correct,
            _ => unreachable!(),
        };
    }
    pat
}

pub fn str_to_pat(s: &str) -> Pattern {
    use Tile::*;
    let mut pat = [Wrong; NUM_LETTERS];
    for (i, p) in s.chars().enumerate() {
        pat[i] = match p {
            'c' => Correct,
            'm' => Misplaced,
            'x' => Wrong,
            _ => panic!("Bad pattern string, must only consist of 'm' 'c' and 'w'"),
        }
    }
    pat
}

pub fn word_to_str(word: Word) -> String {
    let mut s = String::new();
    for p in word {
        s.push((p + 97) as char);
    }
    s
}

pub fn compute_wordmask(word: Word) -> WordMask {
    let mut wm: WordMask = [0; 26];
    for i in 0..NUM_LETTERS {
        wm[word[i] as usize] += 1;
    }
    wm
}

pub fn compute_pattern(guess: Word, target: Word, mut target_mask: WordMask) -> Pattern {
    use Tile::*;
    let mut pat = [Wrong; NUM_LETTERS];
    for i in 0..NUM_LETTERS {
        let g = guess[i];

        if g == target[i] {
            pat[i] = Correct;
            target_mask[g as usize] -= 1;
        }
    }
    for i in 0..NUM_LETTERS {
        let g = guess[i];
        if (target_mask[g as usize] > 0) && (pat[i] != Correct) {
            target_mask[g as usize] -= 1;
            pat[i] = Misplaced;
        }
    }
    pat
}

pub fn pattern_consistent(
    guess: Word,
    pattern: Pattern,
    word: Word,
    mut word_mask: WordMask,
) -> bool {
    use Tile::*;
    for i in 0..NUM_LETTERS {
        let guess_letter = guess[i];
        let idx = guess_letter as usize;
        if pattern[i] == Correct {
            if guess_letter != word[i] {
                return false;
            } 
            word_mask[idx] -= 1
        }
    }
    for i in 0..NUM_LETTERS {
        let guess_letter = guess[i];
        let idx = guess_letter as usize;
        if pattern[i] == Misplaced {
            if guess_letter == word[i] {
                return false;
            }
            if word_mask[idx] == 0 {
                return false;
            } else {
                word_mask[idx] -= 1
            }
        }
        if pattern[i] == Wrong {
            if word_mask[idx] > 0 {
                return false;
            }
        }
    }
    return true;
}

pub fn str_to_word(s: &str) -> Word {
    let mut a = [0; NUM_LETTERS];
    for (i, c) in s.chars().enumerate() {
        a[i] = (c as u8) - 97;
        assert!(i < NUM_LETTERS)
    }
    a
}

pub fn words_consistent(words: Vec<(Word, WordMask)>, pat: Pattern, guess: Word) -> Vec<(Word, WordMask)>{
   words.into_iter().filter(
       |(w, wm)| pattern_consistent(guess, pat, *w, *wm)
    ).collect()
}
pub fn load() -> Vec<Word> {
    let s = include_str!("../words.json");
    let json: serde_json::Value = serde_json::from_str(s).unwrap();
    let arr = json.as_array().unwrap();
    let mut v = Vec::with_capacity(arr.len());
    for val in arr {
        let s = val.as_str().unwrap();
        let a = str_to_word(s);
        v.push(a);
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_pattern() {
        let test_cases = vec![
            (
                "there",
                vec![("hello", "mmxxx"), ("river", "mxxmx"), ("thero", "ccccx")],
            ),
            (
                "colon",
                vec![("coals", "ccxmx"), ("pools", "xcmmx"), ("spool", "xxmcm")],
            ),
            ("awake", vec![("piece", "xxxxc"), ("evade", "xxcxc")]),
            ("evade", vec![("awake", "xxcxc")]),
        ];

        for (target, cases) in test_cases.iter() {
            let target = str_to_word(target);
            for (guess, patt) in cases.iter() {
                let guess = str_to_word(guess);
                let mask = compute_wordmask(target);
                let pat = compute_pattern(guess, target, mask);
                let s_pat = pat_to_str(pat);
                assert_eq!(&s_pat.as_str(), patt);
            }
        }
    }

    #[test]
    fn test_pattern_consistent() {
        let test_cases = vec![
            (
                "colon",
                "xcmmx",
                vec![
                    ("pools", true),
                    ("troop", false),
                    ("pooom", false),
                    ("spool", false),
                ],
            ),
            (
                "regex",
                "ccccx",
                vec![("regen", true), ("regex", false), ("rogex", false)],
            ),
            ("piece", "xmxxx", vec![("naval", false), ("sissy", false)]),
        ];
        for (target_s, pattern_s, cases) in test_cases.iter() {
            let pattern = str_to_pat(pattern_s);
            let target = str_to_word(target_s);
            let target_mask = compute_wordmask(target);
            for (guess_s, is_cons) in cases.iter() {
                let guess = str_to_word(guess_s);
                let is_cons_t = pattern_consistent(guess, pattern, target, target_mask);
                let err = format!(
                    "(guess {}) (target {}) (pattern {}) (expected {}) (actual {})",
                    guess_s, target_s, pattern_s, is_cons, is_cons_t
                );
                assert_eq!(is_cons_t, *is_cons, "{}", err);
            }
        }
    }
}

struct BestDat {
    word: Word,
    score: usize
}

pub fn compute_best_word(all_words: &Vec<(Word, WordMask)>, possible_words: &Vec<(Word, WordMask)>) -> Word {

    let n_words = all_words.len();
    let best_dat = Arc::new(Mutex::new(BestDat {word: str_to_word("none"), score: n_words}));
    (0..n_words).into_par_iter().for_each(|i| {
        let (guess, _) = all_words[i];
        let mut narrow = 0;
        let mut cache = [-1_i32;3_usize.pow(NUM_LETTERS as u32)];
        for (target, target_mask) in possible_words.iter() {
            let mut counter = 0;
            let pat = compute_pattern(guess, *target, *target_mask);
            let ipat = pat_to_int(pat);
            let cv = cache[ipat as usize];
            if cv != -1 {
                counter = cv;
            } else {
                for (w, w_mask) in possible_words.iter() {
                    let consis = pattern_consistent(guess, pat, *w, *w_mask);
                    if consis {
                        counter += 1;
                    }
                }
                cache[ipat as usize] = counter;
            }
            narrow += counter;
        }
        let best_dat = Arc::clone(&best_dat);
        let mut best_dat = best_dat.lock().unwrap();
        if (narrow as usize) < best_dat.score {
            best_dat.score = narrow as usize;
            best_dat.word = guess;
        }
    });
    let best_dat = Arc::clone(&best_dat);
    let best_dat = best_dat.lock().unwrap();
    best_dat.word
}