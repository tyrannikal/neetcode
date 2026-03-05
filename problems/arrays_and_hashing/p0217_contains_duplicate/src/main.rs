// Problem: Contains Duplicate (LeetCode #217)
// Category: Arrays & Hashing
// Difficulty: Easy
// Link: https://leetcode.com/problems/contains-duplicate/
//
// Pattern:
// Time Complexity:
// Space Complexity:
//
// Key Insight:

#[allow(unused)]
fn contains_duplicate(nums: Vec<i32>) -> bool {
    todo!()
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
