// Problem: Contains Duplicate (LeetCode #217)
// Category: Arrays & Hashing
// Difficulty: Easy
// Link: https://leetcode.com/problems/contains-duplicate/
//
// Pattern: HashSet for O(1) lookup
// Time Complexity: O(n) — single pass, short-circuits on first duplicate
// Space Complexity: O(n) — HashSet stores up to n elements
//
// Key Insight: HashSet::insert returns false if the value already exists,
// doubling as both insertion and duplicate check in one call.
// Alternative: sort + scan is O(n log n)/O(1) if space matters more than time.

use std::collections::HashSet;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut nums_set = HashSet::new();
    for num in nums {
        if !nums_set.insert(num) {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", contains_duplicate(vec![1, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_duplicate() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn test_no_duplicate() {
        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_empty() {
        assert!(!contains_duplicate(vec![]));
    }

    #[test]
    fn test_single() {
        assert!(!contains_duplicate(vec![1]));
    }
}
