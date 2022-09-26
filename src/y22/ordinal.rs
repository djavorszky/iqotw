/// Given a positive integer, return its ordinal
/// ```
/// use iqotw::y22::ordinal::ordinal;
///
/// println!("{}", ordinal(19))
/// // -> 19th
/// ```
pub fn ordinal(n: usize) -> String {
    let ordinal = match n % 100 {
        1 | 21 | 31 | 41 | 51 | 61 | 71 | 81 | 91 => "st",
        2 | 22 | 32 | 42 | 52 | 62 | 72 | 82 | 92 => "nd",
        3 | 23 | 33 | 43 | 53 | 63 | 73 | 83 | 93 => "rd",
        _ => "th",
    };

    format!("{n}{ordinal}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordinal() {
        assert_eq!("1st", ordinal(1));
        assert_eq!("2nd", ordinal(2));
        assert_eq!("3rd", ordinal(3));
        assert_eq!("4th", ordinal(4));
        assert_eq!("6th", ordinal(6));
        assert_eq!("7th", ordinal(7));
        assert_eq!("10th", ordinal(10));
        assert_eq!("11th", ordinal(11));
        assert_eq!("12th", ordinal(12));
        assert_eq!("13th", ordinal(13));
        assert_eq!("21st", ordinal(21));
        assert_eq!("22nd", ordinal(22));
        assert_eq!("23rd", ordinal(23));
        assert_eq!("111th", ordinal(111));
        assert_eq!("112th", ordinal(112));
        assert_eq!("913th", ordinal(913));
        assert_eq!("10012th", ordinal(10012));
    }
}
