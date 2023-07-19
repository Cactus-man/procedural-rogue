use bevy::reflect::{Reflect, TypeUuid};
use serde::{
    de::{Deserializer, Error, Unexpected},
    Deserialize,
};
use std::{fmt, marker::PhantomData};

fn checked_string<'de, D>(de: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    is_constructable_or_error(s).map_err(|err| {
        D::Error::invalid_value(Unexpected::Str(&err.text), &err.to_string().as_str())
    })
}

#[derive(Clone, Debug, Deserialize, TypeUuid, Reflect)]
#[uuid = "d9ca7e66-92d3-49a8-9328-8f97c6157020"]
#[serde(bound = "", transparent)]
pub struct Id<T> {
    #[serde(deserialize_with = "checked_string")]
    inner: String,
    #[reflect(ignore)]
    #[serde(skip)]
    target: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new_checked(s: String) -> Option<Self> {
        match Self::try_from(s) {
            Ok(id) => Some(id),
            Err(_) => None,
        }
    }
}

#[derive(Debug)]
enum ErrorKind {
    BadCharacter { idx: usize, char: char },
    NoNamespace,
    EmptySegment(usize),
}

#[derive(Debug, thiserror::Error)]
pub struct ParseIdError {
    text: String,
    kind: ErrorKind,
}

impl ParseIdError {
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl fmt::Display for ParseIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::BadCharacter { idx, char } => write!(
                f,
                "an alphanumeric character at index {}, but found '{}' instead",
                idx, char
            ),
            ErrorKind::NoNamespace => write!(
                f,
                "namespace is empty but must be at least one character long"
            ),
            ErrorKind::EmptySegment(i) => write!(f, "the name has an empty segment at index {}", i),
        }
    }
}

fn is_constructable_or_error(text: String) -> Result<String, ParseIdError> {
    let found_bad_character = text
        .chars()
        .enumerate()
        .find(|&(_, c)| !c.is_ascii_alphanumeric() && c != '_' && c != '.');
    let kind = if let Some((idx, char)) = found_bad_character {
        ErrorKind::BadCharacter { idx, char }
    } else if text.starts_with(".") {
        ErrorKind::NoNamespace
    } else if let Some(idx) = text.find("..") {
        ErrorKind::EmptySegment(idx)
    } else {
        return Ok(text);
    };
    Err(ParseIdError { text, kind })
}

impl<T> TryFrom<String> for Id<T> {
    type Error = ParseIdError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        is_constructable_or_error(text).map(|inner| Self {
            inner,
            target: PhantomData,
        })
    }
}

impl<T> std::str::FromStr for Id<T> {
    type Err = <Self as TryFrom<String>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_owned())
    }
}
