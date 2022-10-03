///  Given two integers, generate a “fibonacci-like” sequence of n digits
/// (where the next number in the pattern is the sum of the previous two numbers).
pub struct FibIterator {
    sequence_length: usize,
    index: usize,
    n1: usize,
    n2: usize,
}

impl FibIterator {
    pub fn new(n1: usize, n2: usize, sequence_length: usize) -> Self {
        Self {
            sequence_length,
            n1,
            n2,
            index: 0,
        }
    }
}

/// Implements iterator such that it yields numbers in a Fibonacci-like fashion.
/// ```
/// use iqotw::y22::fib_like::FibIterator;
///
/// let expected_sequence = &[5, 10, 15, 25, 40, 65];
///
/// // For loops and everything else granted by the `Iterator` trait
/// for (idx, n) in FibIterator::new(5, 10, 6).enumerate() {
///     assert_eq!(n, expected_sequence[idx]);
/// }
///
/// // Collect to Vec
/// let result: Vec<usize> = FibIterator::new(8, 3, 6).collect();
/// let expected = vec![8, 3, 11, 14, 25, 39];
///
/// assert_eq!(result, expected);
/// ```
impl Iterator for FibIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.sequence_length {
            None
        } else {
            self.index += 1;
            let res = self.n1;
            self.n1 = self.n2;
            self.n2 = self.n1 + res;
            Some(res)
        }
    }
}

#[cfg(test)]
mod fib_iterator_tests {

    use super::FibIterator;

    #[test]
    fn test_question_cases() {
        assert_eq!(
            FibIterator::new(10, 20, 5).collect::<Vec<usize>>(),
            vec![10, 20, 30, 50, 80]
        );
        assert_eq!(
            FibIterator::new(3, 7, 5).collect::<Vec<usize>>(),
            vec![3, 7, 10, 17, 27]
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(FibIterator::new(1, 1, 0).next(), None);
        assert_eq!(FibIterator::new(1, 1, 1).collect::<Vec<usize>>(), vec![1]);
        assert_eq!(
            FibIterator::new(1, 1, 2).collect::<Vec<usize>>(),
            vec![1, 1]
        );
    }
}

/// Given a sequence, determine if the sequence is “Fibonacci-like”. A sequence is considered to be Fibonacci-like if:
/// 1. It has at least 3 items
/// 2. Each item is the sum of the previous two items
/// 3. The second item in the list is greater than, or equal to the first item
///
/// ```
/// use iqotw::y22::fib_like::is_fibonacci_like;
///
/// assert!(is_fibonacci_like(&[1, 1, 2, 3]));
/// assert!(is_fibonacci_like(&[10, 20, 30, 50]));
/// assert!(!is_fibonacci_like(&[10, 20, 20, 10]));
/// ```
pub fn is_fibonacci_like(seq: &[usize]) -> bool {
    if seq.len() < 3 || seq[0] > seq[1] {
        return false;
    }

    let mut sum = seq[0] + seq[1];

    for (idx, n) in seq.iter().enumerate().skip(2) {
        if *n != sum {
            return false;
        }
        sum -= seq[idx - 2];
        sum += n;
    }

    true
}

#[cfg(test)]
mod is_fibonacci_like_tests {
    use super::*;

    #[test]
    fn test_question_cases() {
        assert!(is_fibonacci_like(&[10, 20, 30, 50, 80]));
        assert!(is_fibonacci_like(&[3, 7, 10, 17, 27]));
    }

    #[test]
    fn test_fibonacci_sequence() {
        assert!(is_fibonacci_like(&[1, 1, 2, 3, 5, 8, 13]));
        assert!(is_fibonacci_like(&[0, 1, 1, 2, 3, 5, 8, 13]));
    }

    #[test]
    fn test_fib_iterator_result() {
        assert!(is_fibonacci_like(
            &FibIterator::new(5, 9, 17).collect::<Vec<usize>>()
        ))
    }

    #[test]
    fn test_too_few_items() {
        assert!(!is_fibonacci_like(&[]));
        assert!(!is_fibonacci_like(&[1]));
        assert!(!is_fibonacci_like(&[1, 1]));
    }

    #[test]
    fn test_not_fibonacci_like() {
        assert!(!is_fibonacci_like(&[5, 2, 3, 6]));
        assert!(!is_fibonacci_like(&[1, 1, 2, 3, 5, 9]));
        assert!(!is_fibonacci_like(&[5, 3, 8, 11, 19]));
    }
}
