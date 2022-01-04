use super::super::super::{DbXxxKeyType, HashValue};
use super::FileDbMap;
use std::fmt::{Display, Error, Formatter};
use std::ops::Deref;

/// DbInt Map in a file databse.
pub type FileDbMapDbInt = FileDbMap<DbInt>;

/// DbInt
/// New type pattern of `Vec<u8>`.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct DbInt(Vec<u8>);

impl DbXxxKeyType for DbInt {
    #[inline]
    fn signature() -> [u8; 8] {
        [b'u', b'i', b'n', b't', b'6', b'4', 0u8, 0u8]
    }
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }
    #[inline]
    fn from(bytes: &[u8]) -> Self {
        DbInt(bytes.to_vec())
    }
    fn byte_len(&self) -> usize {
        self.0.len()
    }
    fn cmp_u8(&self, other: &[u8]) -> std::cmp::Ordering {
        self.0.as_slice().cmp(other)
    }
}
impl HashValue for DbInt {}

impl Display for DbInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let ss = String::from_utf8_lossy(&self.0).to_string();
        write!(f, "'{}'", ss)
    }
}

impl Deref for DbInt {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

/*
impl Borrow<[u8]> for DbInt {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}
*/

impl From<&[u8]> for DbInt {
    #[inline]
    fn from(a: &[u8]) -> Self {
        DbInt(a.to_vec())
    }
}

impl From<Vec<u8>> for DbInt {
    #[inline]
    fn from(a: Vec<u8>) -> Self {
        DbInt(a)
    }
}

impl From<&str> for DbInt {
    #[inline]
    fn from(a: &str) -> Self {
        DbInt(a.as_bytes().to_vec())
    }
}

impl From<String> for DbInt {
    #[inline]
    fn from(a: String) -> Self {
        DbInt(a.into_bytes())
    }
}

impl From<&String> for DbInt {
    #[inline]
    fn from(a: &String) -> Self {
        DbInt(a.as_bytes().to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for DbInt {
    #[inline]
    fn from(a: &[u8; N]) -> Self {
        DbInt(a.to_vec())
    }
}

impl From<u64> for DbInt {
    #[inline]
    fn from(a: u64) -> Self {
        DbInt(a.to_be_bytes().to_vec())
    }
}

impl From<&u64> for DbInt {
    #[inline]
    fn from(a: &u64) -> Self {
        DbInt(a.to_be_bytes().to_vec())
    }
}
