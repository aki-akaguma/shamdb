use super::super::super::{DbXxxKeyType, HashValue};
use super::FileDbMap;
use std::fmt::{Display, Error, Formatter};
use std::ops::Deref;

/// DbBytes Map in a file databse.
pub type FileDbMapString = FileDbMap<DbString>;

/// DbBytes
/// New type pattern of `Vec<u8>`.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct DbString(Vec<u8>);

impl DbXxxKeyType for DbString {
    #[inline]
    fn signature() -> [u8; 8] {
        [b's', b't', b'r', b'i', b'n', b'g', 0u8, 0u8]
    }
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }
    #[inline]
    fn from(bytes: &[u8]) -> Self {
        DbString(bytes.to_vec())
    }
    #[inline]
    fn byte_len(&self) -> usize {
        self.as_bytes().len()
    }
    fn cmp_u8(&self, other: &[u8]) -> std::cmp::Ordering {
        self.0.as_slice().cmp(other)
    }
}
impl HashValue for DbString {}

impl Display for DbString {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let ss = String::from_utf8_lossy(&self.0).to_string();
        write!(f, "'{}'", ss)
    }
}

impl Deref for DbString {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

/*
impl Borrow<[u8]> for DbString {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}
*/

impl From<&[u8]> for DbString {
    #[inline]
    fn from(a: &[u8]) -> Self {
        DbString(a.to_vec())
    }
}

impl From<Vec<u8>> for DbString {
    #[inline]
    fn from(a: Vec<u8>) -> Self {
        DbString(a)
    }
}

impl From<&str> for DbString {
    #[inline]
    fn from(a: &str) -> Self {
        DbString(a.as_bytes().to_vec())
    }
}

impl From<String> for DbString {
    #[inline]
    fn from(a: String) -> Self {
        DbString(a.into_bytes())
    }
}

impl From<&String> for DbString {
    #[inline]
    fn from(a: &String) -> Self {
        DbString(a.as_bytes().to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for DbString {
    #[inline]
    fn from(a: &[u8; N]) -> Self {
        DbString(a.to_vec())
    }
}

impl From<u64> for DbString {
    #[inline]
    fn from(a: u64) -> Self {
        DbString(a.to_be_bytes().to_vec())
    }
}

impl From<&u64> for DbString {
    #[inline]
    fn from(a: &u64) -> Self {
        DbString(a.to_be_bytes().to_vec())
    }
}

/*
impl From<DbBytes> for DbString {
    #[inline]
    fn from(a: DbBytes) -> Self {
        DbString(a.0)
    }
}
*/

impl From<&DbString> for DbString {
    #[inline]
    fn from(a: &DbString) -> Self {
        DbString(a.0.clone())
    }
}