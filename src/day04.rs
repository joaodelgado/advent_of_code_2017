use std::collections::HashMap;
use std::collections::HashSet;

use Day;
use utils::read_file;

fn count_chars(s: &str) -> HashMap<char, usize> {
    let mut char_count = HashMap::new();

    for c in s.chars() {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
    }

    char_count
}

fn is_anagram(s1: &str, s2: &str) -> bool {
    count_chars(s1) == count_chars(s2)
}

pub struct Day04 {}

impl Day<usize, usize> for Day04 {
    fn run1() -> usize {
        let input = read_file("data/day04");
        let passphrases = input.split('\n');

        passphrases
            .filter(|passphrase| !passphrase.is_empty())
            .filter(|passphrase| {
                let n_word = passphrase.split(' ').count();
                let n_unique_words = passphrase.split(' ').collect::<HashSet<_>>().len();

                n_word == n_unique_words
            })
            .count()

    }

    fn run2() -> usize {
        let input = read_file("data/day04");
        let passphrases = input.split('\n');

        passphrases
            .filter(|passphrase| !passphrase.is_empty())
            .filter(|passphrase| {
                let n_word = passphrase.split(' ').count();
                let n_unique_words = passphrase.split(' ').collect::<HashSet<_>>().len();

                n_word == n_unique_words
            })
            .filter(|passphrase| {
                let words: Vec<_> = passphrase.split(' ').collect();
                let words_copy = words.clone();

                for w1 in &words {
                    for w2 in &words_copy {
                        // Skip equal words. We already checked for repeating ones
                        if w2 == w1 {
                            continue;
                        }
                        if is_anagram(w1, w2) {
                            return false;
                        }
                    }
                }
                return true;
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day04::run1(), 383);
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day04::run2(), 265);
    }
}
