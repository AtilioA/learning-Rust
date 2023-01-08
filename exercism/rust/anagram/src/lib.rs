use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // Create a HashSet to store anagrams (we won't actually use values)
    let mut anagrams = HashSet::<&'a str>::new();

    // Sort lowercase input word
    let word_lower = word.to_lowercase();
    let mut word_chars: Vec<char> = word_lower.chars().collect();
    /* "If sorting, consider sort_unstable which is typically faster than stable sorting.
    When applicable, unstable sorting is preferred because it is generally faster than stable sorting"
    and it doesn't allocate auxiliary memory." */
    word_chars.sort_unstable();

    for possible_anagram in possible_anagrams {
        // Sort lowercase anagram
        let lower_possible_anagram = possible_anagram.to_lowercase();
        let mut anagram_chars: Vec<char> = lower_possible_anagram.chars().collect();
        anagram_chars.sort_unstable();

        let mut dont_match: bool = false;
        // If the words are of equal length, check if they are anagrams by comparing each char from the sorted "strings"
        if word_chars.len() == anagram_chars.len() {
            for (i, word_char) in word_chars.iter().enumerate() {
                // If any character doesn't match, skip this anagram
                if !word_char.eq_ignore_ascii_case(&anagram_chars[i]) {
                    dont_match = true;
                    break;
                }
            }

            // If the word and possible_anagrams are the same length and don't match, don't add to anagrams
            if !dont_match {
                // If the word and the anagram aren't identical, add to anagrams
                if !lower_possible_anagram.eq_ignore_ascii_case(&word_lower) {
                    anagrams.insert(possible_anagram);
                }
            }
        }
    }

    return anagrams;
}
