pub fn capitalize_after_vowel(text: &str) -> String {
    let mut after_vowel = false;
    text.chars()
        .into_iter()
        .map(|c| match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                after_vowel = true;
                c
            }
            c if c.is_ascii_alphabetic() => {
                let mut c = c;
                if after_vowel && c.is_ascii_lowercase() {
                    c = (c as u8 - 32) as char
                }
                after_vowel = false;
                c
            }
            _ => c,
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {

    use super::*;

    use test_case::test_case;

    #[test_case("hello world", "heLlo WoRld".to_string())]
    #[test_case("xaabeuekadii", "xaaBeueKaDii".to_string())]
    fn test_capitalize(text: &str, expected: String) {
        assert_eq!(capitalize_after_vowel(text), expected);
    }
}
