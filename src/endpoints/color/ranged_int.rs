use std::{fmt::Display, ops::Range};

use thiserror::Error;


#[derive(Error, Debug)]
#[error("The value {0} is not in range {1}")]
pub struct OutOfRangeError<T: Display>(T, Range<T>);

