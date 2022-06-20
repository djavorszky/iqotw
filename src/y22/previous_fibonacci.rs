use std::iter::successors;

/// If n is a Fibonacci number, returns the previous Fibonacci number, None otherwise.
/// 
/// # Examples
/// ```
/// # use iqotw::y22::previous_fibonacci::*; 
/// assert_eq!(previous_fibonacci(15), None);
/// assert_eq!(previous_fibonacci(13), Some(8));
///
/// ```
pub fn previous_fibonacci(n: usize) -> Option<usize> {
    successors(Some((0, 1)), |(n1, n2)| Some((*n2, n1 + n2)))
        .take_while(|(n1, _)| n1 < &n)
        .last()
        .filter(|(_, n2)| *n2 == n)
        .map(|(n1, _)| n1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_found() {
        [9, 15, 42].iter().for_each(|n| assert_eq!(previous_fibonacci(*n), None))
    }

    #[test]
    fn found_fibonacci() {
        [(5, 3), (8, 5), (6765, 4181)].iter()
            .for_each(|(next, prev)| assert_eq!(previous_fibonacci(*next), Some(*prev)))
    }
}
