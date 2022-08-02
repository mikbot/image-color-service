// This is duplicated code from colors.rs.
// I'm working on it to replace it with a macro.
use serde::{
    de::{Error, Visitor},
    Deserialize,
};
use thiserror::Error;

#[derive(Debug)]
pub struct Quality(u8);

impl Quality {
    pub fn value(&self) -> &u8 {
        &self.0
    }
}

impl TryFrom<u8> for Quality {
    type Error = QualityError;
    fn try_from(value: u8) -> Result<Quality, Self::Error> {
        if (1..=10).contains(&value) {
            Ok(Quality(value))
        } else {
            Err(QualityError::from(value))
        }
    }
}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct QualityError(String);

impl From<u8> for QualityError {
    fn from(value: u8) -> Self {
        QualityError(format!("{} is not between 1 and 10", value))
    }
}

impl QualityError {
    fn as_serde_de_error<E>(&self) -> E
    where
        E: serde::de::Error,
    {
        Error::custom(&self.0)
    }
}

struct QualityVisitor;

impl<'de> Visitor<'de> for QualityVisitor {
    type Value = Quality;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between 1 and 10")
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Quality::try_from(v).map_err(|e| e.as_serde_de_error())
    }
}

impl<'de> Deserialize<'de> for Quality {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u8(QualityVisitor)
    }
}

impl Default for Quality {
    fn default() -> Self {
        Quality(10)
    }
}
