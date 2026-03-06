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

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut sorted_nums = nums;
    sorted_nums.sort();
    for i in 1..sorted_nums.len() {
        if sorted_nums[i] == sorted_nums[i - 1] {
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
