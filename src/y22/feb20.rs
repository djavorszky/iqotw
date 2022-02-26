// Given an array of integers, find the length of the longest sub-sequence
// such that elements in the sub-sequence are consecutive integers, the
// consecutive numbers can be in any order.

use std::collections::HashSet;

pub fn longest_sub_seq(input: &[i32]) -> usize {
    let mut clone: Vec<i32> = input.to_vec();
    clone.sort_unstable();

    clone
        .windows(2)
        .fold((1, 1), |acc, w| {
            if w[1] - w[0] == 1 {
                (acc.0, acc.1 + 1)
            } else {
                (acc.0.max(acc.1), 1)
            }
        })
        .0
}

pub fn longest_sub_seq_map(input: &[i32]) -> usize {
    let orig: HashSet<i32> = input.iter().cloned().collect();
    let mut seen = HashSet::with_capacity(input.len());
    let mut highest_count = 1;

    for n in input {
        if seen.contains(n) {
            continue;
        }
        let mut count = 1;

        let mut next = n + 1;
        while orig.contains(&next) {
            seen.insert(next);
            next += 1;
            count += 1;
        }

        let mut prev = n - 1;
        while orig.contains(&prev) {
            seen.insert(prev);
            prev += 1;
            count += 1;
        }

        highest_count = highest_count.max(count);
    }

    highest_count
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_longest_sub_seq() {
        assert_eq!(longest_sub_seq(&[1, 9, 87, 3, 10, 4, 20, 2, 45]), 4);
        assert_eq!(
            longest_sub_seq(&[36, 41, 56, 35, 91, 33, 34, 92, 43, 37, 42]),
            5
        );
    }

    #[test]
    fn test_longest_sub_seq_map() {
        assert_eq!(longest_sub_seq_map(&[1, 9, 87, 3, 10, 4, 20, 2, 45]), 4);
        assert_eq!(
            longest_sub_seq_map(&[36, 41, 56, 35, 91, 33, 34, 92, 43, 37, 42]),
            5
        );
    }
}
