// Problem: Valid Anagram (LeetCode #242)
// Category: Arrays & Hashing
// Difficulty: Easy
// Link: https://leetcode.com/problems/valid-anagram/
//
// Pattern:
// Time Complexity:
// Space Complexity:
//
// Key Insight:

use std::collections::HashMap;

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut check = HashMap::new();
    for chr in s.chars() {
        *check.entry(chr).or_insert(0) += 1;
    }
    for chr in t.chars() {
        *check.entry(chr).or_insert(0) -= 1;
    }
    check.values().all(|&v| v == 0)
}

fn main() {
    println!(
        "{}",
        is_anagram("anagram".to_string(), "nagaram".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
    }

    #[test]
    fn test_not_anagram() {
        assert!(!is_anagram("rat".to_string(), "car".to_string()));
    }

    #[test]
    fn test_different_lengths() {
        assert!(!is_anagram("ab".to_string(), "a".to_string()));
    }

    #[test]
    fn test_empty() {
        assert!(is_anagram("".to_string(), "".to_string()));
    }
}
