use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String>  {
        if validate_email(&s) {
            return Ok(Self(s));
        }
        return Err(format!("{} is not a valid subscriber email", s));
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    macro_rules! invalid_email_case {
        ( $case_name:ident, $value:expr ) => {
            #[test]
            fn $case_name() {
                let email = $value.to_string();
                assert_err!(SubscriberEmail::parse(email));
            }
        };
    }

    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use super::SubscriberEmail;
    use claim::assert_err;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_email_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }

    invalid_email_case!(empty_string_is_rejected, "");
    invalid_email_case!(missing_at_symbol_rejected, "abc.com");
    invalid_email_case!(missing_subject_is_rejected, "@domain.com");
}
