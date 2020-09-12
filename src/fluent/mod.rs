#[allow(unused_imports)]
use fluent_templates::{static_loader, Loader};
use unic_langid::{langid, LanguageIdentifier};

pub const US_ENGLISH: LanguageIdentifier = langid!("en-US");
#[allow(dead_code)]
pub const GERMAN: LanguageIdentifier = langid!("de");

static_loader! {
    pub static LOCALES = {
        locales: "./src/fluent/locales",
        fallback_language: "en-US",
        // A fluent resource that is shared with every locale.
        core_locales: "./src/fluent/locales/core.ftl",
    };
}

pub trait ToLocaleString {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String;
}

pub trait Valuable {
    fn revise_value(&mut self, new_value: u8);

    fn get_value(&self) -> u8;
}

pub fn get_value(name: &str) -> String {
    let var = "-value";
    let id = format!("{}{}", name, var).as_str();
    LOCALES.lookup(&US_ENGLISH, id)
}

pub fn get_value_u8(name: &str) -> u8 {
    let s = get_value(name);
    s.parse().unwrap_or(0)
}

#[cfg(test)]
mod fluent_tests {
    use super::*;

    #[test]
    fn doit() {
        let s = LOCALES.lookup(&US_ENGLISH, "spades-letter");

        assert_eq!("S", s);
    }
}
