use crate::fluent::{ToLocaleString, US_ENGLISH};
use std::fmt;

/// Card Rank Short - Single field struct representing the short string of a card rank.
///
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct RankShort(String);

impl RankShort {
    // Accepts String or &str
    pub fn new<S>(name: S) -> RankShort
    where
        S: Into<String>,
    {
        RankShort(name.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl ToLocaleString for RankShort {
    fn get_fluent_key(&self) -> String {
        self.0.to_owned() + &*"-short".to_owned()
    }

    fn get_raw_name(&self) -> String {
        self.0.clone()
    }
}

impl fmt::Display for RankShort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_locale_string(&US_ENGLISH))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod rank_letter_tests {
    use super::*;
    use crate::fluent::{ToLocaleString, GERMAN};

    #[test]
    fn display() {
        assert_eq!(
            "RangKurz: A",
            format!("RangKurz: {}", RankShort::new("ace"))
        );
    }

    #[test]
    fn as_str() {
        assert_eq!(RankShort::new("bar").as_str(), "bar");
    }

    #[test]
    fn to_string() {
        assert_eq!(RankShort::new("king").to_string(), "K".to_string());
    }

    #[test]
    fn new() {
        let from_string = "from".to_string();

        assert_eq!(RankShort("from".to_string()), RankShort::new(from_string));
        assert_eq!(RankShort("from".to_string()), RankShort::new("from"));
    }

    #[test]
    fn to_string_by_locale() {
        let clubs = RankShort::new("ten");

        assert_eq!(clubs.to_locale_string(&GERMAN), "10".to_string());
    }
}
