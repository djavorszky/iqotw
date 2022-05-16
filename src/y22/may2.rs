pub fn sac(words: &[String], sub: &str) -> Vec<String> {
    words
        .iter()
        .filter(|w| w.contains(sub))
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn works() {
        assert_eq!(
            sac(
                &["apple", "appoint", "charlie"].map(|w| w.to_string()),
                "app"
            ),
            vec![String::from("apple"), String::from("appoint")]
        )
    }

    #[test]
    fn not_found() {
        assert_eq!(
            sac(
                &["apple", "appoint", "charlie"].map(|w| w.to_string()),
                "bob"
            ),
            Vec::<String>::new()
        )
    }

}
