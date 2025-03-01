//! The edition of the PDF language used in a crate.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Edition {
    // The syntax context stuff needs the discriminants to start from 0 and be consecutive.
    Pdf10 = 0,
    Pdf11,
    Pdf12,
    Pdf13,
    Pdf14,
    Pdf15,
    Pdf16,
    Pdf17,
    Pdf20,
}

impl Edition {
    pub const CURRENT: Edition = Edition::Pdf20;
    pub const LATEST: Edition = Edition::Pdf20;
}

#[derive(Debug)]
pub struct ParseEditionError {
    invalid_input: String,
}

impl std::error::Error for ParseEditionError {}
impl fmt::Display for ParseEditionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid edition: {:?}", self.invalid_input)
    }
}

impl std::str::FromStr for Edition {
    type Err = ParseEditionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "1.0" => Edition::Pdf10,
            "1.1" => Edition::Pdf11,
            "1.2" => Edition::Pdf12,
            "1.3" => Edition::Pdf13,
            "1.4" => Edition::Pdf14,
            "1.5" => Edition::Pdf15,
            "1.6" => Edition::Pdf16,
            "1.7" => Edition::Pdf17,
            "2.0" => Edition::Pdf20,
            _ => return Err(ParseEditionError { invalid_input: s.to_owned() }),
        };
        Ok(res)
    }
}
