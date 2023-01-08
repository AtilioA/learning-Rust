// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut entries = HashMap::new();

    // Populate the map with the words in the magazine and their counts.
    for word in magazine {
        *entries.entry(word).or_insert(0) += 1;
    }

    // Check for each word in the note if it exists in the map.
    for word in note {
        // Get word count from map.
        if let Some(count) = entries.get(word) {
            // If it is in the map but doesn't have any counts left (all were taken from the magazine), return false.
            if *count == 0 {
                return false;
            // If there's still counts, decrement the count (take from magazine).
            } else {
                *entries.entry(word).or_insert(0) -= 1;
            }
        } else {
            // If there is not word in the map, return false.
            return false;
        }
    }

    // If we've reached this point, it means that all words in the note were found in the magazine with enough counts.
    return true;
}
