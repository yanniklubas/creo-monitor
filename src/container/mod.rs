use std::borrow::Borrow;
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

mod error;

pub use error::{Error, Result};

/// The maximum allowed length for a [`ContainerID`].
const CONTAINER_ID_MAX_LEN: usize = 255;

/// A validated container identifier.
///
/// # Examples
///
/// ```
/// # use creo_monitor::container::{ContainerID, Error};
/// let raw_id = "abc123abc123abc123abc123abc123abc123abc123abc123abc123abc123abcd";
/// let container_id = ContainerID::new(raw_id).unwrap();
/// assert_eq!(container_id.as_ref(), "abc123abc123abc123abc123abc123abc123abc123abc123abc123abc123abcd");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContainerID(Arc<str>);

impl ContainerID {
    /// Creates a new `ContainerID` from the given raw id.
    ///
    /// Returns an error if the raw id length exceeds [`CONTAINER_ID_MAX_LEN`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidContainerID`] if the length of the input exceeds
    /// [`CONTAINER_ID_MAX_LEN`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use creo_monitor::container::{ContainerID, Error};
    /// let valid = "abcdef012345abcdef012345abcdef012345abcdef012345abcdef012345abcd";
    /// let id = ContainerID::new(valid);
    /// assert!(id.is_ok());
    /// ```
    pub fn new(src: impl AsRef<str>) -> Result<Self> {
        let src = src.as_ref();
        if src.len() > CONTAINER_ID_MAX_LEN {
            return Err(Error::InvalidContainerID(src.to_owned()));
        }

        Ok(Self(src.into()))
    }

    pub fn to_arc(&self) -> Arc<str> {
        Arc::clone(&self.0)
    }
}

impl AsRef<str> for ContainerID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for ContainerID {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ContainerID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineID([u8; 16]);

impl MachineID {
    pub fn new(src: [u8; 16]) -> Result<Self> {
        Ok(Self(src))
    }

    pub fn as_raw(&self) -> [u8; 16] {
        self.0
    }
}

impl FromStr for MachineID {
    type Err = Error;

    /// Attempts to parse a `MachineID` from a string slice.
    ///
    /// Returns an error if the input is not exactly  characters long
    /// or contains characters other than lowercase letters (`a-z`) or digits (`0-9`).
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() != 32 {
            return Err(Error::InvalidMachineID(s.to_owned()));
        }
        let mut bytes = [0u8; 16];
        for i in (0..s.len()).step_by(2) {
            bytes[i / 2] = u8::from_str_radix(&s[i..i + 2], 16)
                .map_err(|_| Error::InvalidMachineID(s.to_owned()))?;
        }

        MachineID::new(bytes)
    }
}

impl fmt::Display for MachineID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.0 {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}
