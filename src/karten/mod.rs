pub(crate) mod anzug;
mod anzug_buchstabe;
mod anzug_name;
mod anzug_symbol;
pub(crate) mod rang;
mod rang_kurz;
mod rang_name;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use unic_langid::LanguageIdentifier;

use crate::fluent::{ToLocaleString, US_ENGLISH};
use crate::karten::anzug::Anzug;
use crate::karten::rang::Rang;

#[derive(Clone, Debug, PartialEq)]
pub struct Karte {
    pub rang: Rang,
    pub anzug: Anzug,
}

impl Karte {
    pub fn new<S: std::clone::Clone>(rang: S, anzug: S) -> Karte
        where
            S: Into<String>,
    {
        Karte {
            rang: Rang::new(rang),
            anzug: Anzug::new(anzug),
        }
    }

    pub fn new_from_structs(rang: Rang, anzug: Anzug) -> Karte {
        Karte { rang, anzug }
    }

    pub fn to_txt_string(&self, lid: &LanguageIdentifier) -> String {
        let rang = self.rang.to_locale_string(&lid);
        let anzug = self.anzug.buchstabe.to_locale_string(&lid);
        format!("{}{}", rang, anzug)
    }
}

impl ToLocaleString for Karte {
    fn to_locale_string(&self, lid: &LanguageIdentifier) -> String {
        let rang = self.rang.to_locale_string(&lid);
        let anzug = self.anzug.to_locale_string(&lid);
        format!("{}{}", rang, anzug)
    }
}

impl fmt::Display for Karte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_locale_string(&US_ENGLISH))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod card_tests {
    use super::*;
    use crate::fluent::{ToLocaleString, GERMAN};

    #[test]
    fn new() {
        let expected = Karte {
            rang: Rang::new("ace"),
            anzug: Anzug::new("spades"),
        };

        assert_eq!(expected, Karte::new("ace", "spades"));
    }

    #[test]
    fn new_from_structs() {
        let expected = Karte {
            rang: Rang::new("ace"),
            anzug: Anzug::new("spades"),
        };

        assert_eq!(
            expected,
            Karte::new_from_structs(Rang::new("ace"), Anzug::new("spades"))
        );
    }

    #[test]
    fn to_string_by_locale() {
        let karte = Karte::new("queen", "clubs");

        assert_eq!(karte.to_locale_string(&GERMAN), "D♣".to_string());
    }

    #[test]
    fn to_txt_string() {
        let karte = Karte::new("queen", "clubs");

        assert_eq!(karte.to_txt_string(&GERMAN), "DK".to_string());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Karten(Vec<Karte>);

impl Karten {
    pub fn new() -> Karten {
        Karten::new_from_vector(Vec::new())
    }

    pub fn new_from_vector(v: Vec<Karte>) -> Karten {
        Karten(v)
    }

    pub fn add(&mut self, elem: Karte) {
        self.0.push(elem);
    }

    pub fn all(&self) -> &Vec<Karte> {
        &self.0
    }

    pub fn contains(&self, karte: &Karte) -> bool {
        self.0.contains(karte)
    }

    pub fn draw(&mut self, x: usize) -> Option<Karten> {
        if x > self.len() {
            None
        } else {
            let mut karten = Karten::new();
            for _ in 0..x {
                karten.add(self.draw_first().unwrap());
            }
            Some(karten)
        }
    }

    pub fn draw_first(&mut self) -> Option<Karte> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(0)),
        }
    }

    pub fn draw_last(&mut self) -> Option<Karte> {
        match self.len() {
            0 => None,
            _ => Some(self.remove(self.len() - 1)),
        }
    }

    pub fn first(&self) -> Option<&Karte> {
        self.0.first()
    }

    pub fn get(&self, index: usize) -> Option<&Karte> {
        self.0.get(index)
    }

    pub fn get_random(&self) -> Option<&Karte> {
        self.0.choose(&mut rand::thread_rng())
    }

    pub fn last(&self) -> Option<&Karte> {
        self.0.last()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn position(&self, karte: &Karte) -> Option<usize> {
        self.0.iter().position(|k| k == karte)
    }

    pub fn remove(&mut self, index: usize) -> Karte {
        self.0.remove(index)
    }

    pub fn remove_karte(&mut self, karte: &Karte) -> Option<Karte> {
        let position = self.position(karte);
        match position {
            None => None,
            _ => Some(self.0.remove(position.unwrap())),
        }
    }

    pub fn suffle(&self) -> Karten {
        let mut shuffled = self.clone();
        shuffled.0.shuffle(&mut thread_rng());
        shuffled
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod karten_tests {
    use super::*;

    #[test]
    fn new_all_add_new_from_vector() {
        let qclubs = Karte::new("queen", "clubs");
        let qhearts = Karte::new("queen", "hearts");
        let mut expected = Karten::new();
        expected.add(qclubs.clone());
        expected.add(qhearts.clone());

        let actual = Karten::new_from_vector(vec![qclubs, qhearts]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn contains() {
        let qclubs = Karte::new("queen", "clubs");
        let qhearts = Karte::new("queen", "hearts");
        let deck = Karten::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(deck.contains(&qclubs));
        assert!(deck.contains(&qhearts));
    }

    #[test]
    fn draw() {
        let mut zero = Karten::new();
        let qclubs = Karte::new("queen", "clubs");
        let qhearts = Karte::new("queen", "hearts");
        let qspades = Karte::new("queen", "spades");
        let mut deck =
            Karten::new_from_vector(vec![qclubs.clone(), qhearts.clone(), qspades.clone()]);

        assert!(zero.draw(2).is_none());
        assert_eq!(
            deck.draw(2).unwrap(),
            Karten::new_from_vector(vec![qclubs.clone(), qhearts.clone()])
        );
        assert_eq!(1, deck.len());
    }

    #[test]
    fn draw_first() {
        let mut zero = Karten::new();
        let qclubs = Karte::new("queen", "clubs");
        let qhearts = Karte::new("queen", "hearts");
        let mut deck = Karten::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.draw_first().is_none());
        assert_eq!(deck.draw_first().unwrap(), qclubs);
        assert_eq!(1, deck.len());
    }

    #[test]
    fn draw_last() {
        let mut zero = Karten::new();
        let qclubs = Karte::new("queen", "clubs");
        let qhearts = Karte::new("queen", "hearts");
        let mut deck = Karten::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.draw_last().is_none());
        assert_eq!(deck.draw_last().unwrap(), qhearts);
        assert_eq!(1, deck.len());
    }

    #[test]
    fn first() {
        let zero = Karten::new();
        let qclubs = Karte::new("queen", "clubs");
        let qhearts = Karte::new("queen", "hearts");
        let deck = Karten::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.first().is_none());
        assert_eq!(deck.first().unwrap(), &qclubs);
    }

    #[test]
    fn get() {
        let qclubs = Karte::new("queen", "clubs");
        let qhearts = Karte::new("queen", "hearts");
        let mut deck = Karten::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        let gotten = deck.get(1);

        assert_eq!(gotten.unwrap(), &qhearts);
    }

    #[test]
    fn get_random() {
        let qhearts = Karte::new("queen", "hearts");
        let deck = Karten::new_from_vector(vec![qhearts.clone()]);

        let gotten = deck.get_random();

        assert_eq!(gotten.unwrap(), &qhearts);
    }

    #[test]
    fn last() {
        let zero = Karten::new();
        let qclubs = Karte::new("queen", "clubs");
        let qhearts = Karte::new("queen", "hearts");
        let deck = Karten::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert!(zero.last().is_none());
        assert_eq!(deck.last().unwrap(), &qhearts);
    }

    #[test]
    fn len() {
        let zero = Karten::new();
        let qclubs = Karte::new("queen", "clubs");
        let qhearts = Karte::new("queen", "hearts");
        let deck = Karten::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert_eq!(zero.len(), 0);
        assert_eq!(deck.len(), 2);
    }

    #[test]
    fn position() {
        let qclubs = Karte::new("queen", "clubs");
        let qhearts = Karte::new("queen", "hearts");
        let deck = Karten::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        assert_eq!(0, deck.position(&qclubs).unwrap());
        assert_eq!(1, deck.position(&qhearts).unwrap());
    }

    #[test]
    fn remove() {
        let qclubs = Karte::new("queen", "clubs");
        let qhearts = Karte::new("queen", "hearts");
        let mut deck = Karten::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        let removed = deck.remove(0);

        assert_eq!(removed, qclubs);
        assert_eq!(1, deck.len());
    }

    #[test]
    fn remove_karte() {
        let qclubs = Karte::new("queen", "clubs");
        let qhearts = Karte::new("queen", "hearts");
        let mut deck = Karten::new_from_vector(vec![qclubs.clone(), qhearts.clone()]);

        let removed = deck.remove_karte(&qclubs);

        assert_eq!(removed.unwrap(), qclubs);
        assert!(deck.contains(&qhearts));
        assert!(!deck.contains(&qclubs));
    }
}
