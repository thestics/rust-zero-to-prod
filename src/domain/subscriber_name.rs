use unicode_segmentation::UnicodeSegmentation;

pub static SUBSCRIBER_NAME_MAX_LEN: usize = 256;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {

    pub fn parse(s: String) -> Result<Self, String> {

        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > SUBSCRIBER_NAME_MAX_LEN;
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_chars = s.chars().any(|c| forbidden_chars.contains(&c));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_chars {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }

    pub fn inner(self) -> String {
        self.0
    }

    pub fn inner_mut(&mut self) -> &mut str {
        &mut self.0
    }
    
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "Ґ".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_longer_name_rejected() {
        let name = "Ґ".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }
    
    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err! ( SubscriberName:: parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_ok! ( SubscriberName:: parse(name));
    }
}