// This is duplicated code from quality.rs.
// I'm working on it to replace it with a macro.
use serde::{
    de::{Error, Visitor},
    Deserialize,
};
use thiserror::Error;

#[derive(Debug)]
pub struct Colors(u8);

impl Colors {
    pub fn value(&self) -> &u8 {
        &self.0
    }
}

impl TryFrom<u8> for Colors {
    type Error = ColorsError;
    fn try_from(value: u8) -> Result<Colors, Self::Error> {
        if (2..=255).contains(&value) {
            Ok(Colors(value))
        } else {
            Err(ColorsError::from(value))
        }
    }
}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct ColorsError(String);

impl From<u8> for ColorsError {
    fn from(value: u8) -> Self {
        ColorsError(format!("{} is not between 2 and 255", value))
    }
}

impl ColorsError {
    fn as_serde_de_error<E>(&self) -> E
    where
        E: serde::de::Error,
    {
        Error::custom(&self.0)
    }
}

struct ColorsVisitor;

impl<'de> Visitor<'de> for ColorsVisitor {
    type Value = Colors;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between 2 and 255")
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Colors::try_from(v).map_err(|e| e.as_serde_de_error())
    }
}

impl<'de> Deserialize<'de> for Colors {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u8(ColorsVisitor)
    }
}

impl Default for Colors {
    fn default() -> Self {
        Colors(5)
    }
}
