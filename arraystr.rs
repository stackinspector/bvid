use crate::{BVID_LEN, av2bv, bv2av};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bvid([u8; BVID_LEN]);

impl core::ops::Deref for Bvid {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { core::str::from_utf8_unchecked(&self.0) }
    }
}

impl core::ops::DerefMut for Bvid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::str::from_utf8_unchecked_mut(&mut self.0) }
    }
}

impl AsRef<str> for Bvid {
    fn as_ref(&self) -> &str { self }
}

impl TryFrom<&str> for Bvid {
    type Error = core::array::TryFromSliceError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Bvid(value.as_bytes().try_into()?))
    }
}

impl core::str::FromStr for Bvid {
    type Err = core::array::TryFromSliceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Bvid(s.as_bytes().try_into()?))
    }
}

impl core::borrow::Borrow<str> for Bvid {
    fn borrow(&self) -> &str { self }
}

impl core::borrow::BorrowMut<str> for Bvid {
    fn borrow_mut(&mut self) -> &mut str { self }
}

impl PartialEq<str> for Bvid {
    fn eq(&self, other: &str) -> bool { &**self == other }
}

impl PartialEq<Bvid> for str {
    fn eq(&self, other: &Bvid) -> bool { self == &**other }
}

impl PartialOrd<str> for Bvid {
    fn partial_cmp(&self, other: &str) -> Option<core::cmp::Ordering> { (**self).partial_cmp(other) }
    fn lt(&self, other: &str) -> bool { &**self < other }
    fn le(&self, other: &str) -> bool { &**self <= other }
    fn gt(&self, other: &str) -> bool { &**self > other }
    fn ge(&self, other: &str) -> bool { &**self >= other }
}

impl PartialOrd<Bvid> for str {
    fn partial_cmp(&self, other: &Bvid) -> Option<core::cmp::Ordering> { self.partial_cmp(&**other) }
    fn lt(&self, other: &Bvid) -> bool { self < &**other }
    fn le(&self, other: &Bvid) -> bool { self <= &**other }
    fn gt(&self, other: &Bvid) -> bool { self > &**other }
    fn ge(&self, other: &Bvid) -> bool { self >= &**other }
}

impl core::hash::Hash for Bvid {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) { (**self).hash(state) }
}

impl core::fmt::Debug for Bvid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { (**self).fmt(f) }
}

impl core::fmt::Display for Bvid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { (**self).fmt(f) }
}

impl Bvid {
    #[inline(always)]
    pub fn from_avid(avid: u64) -> Bvid {
        Bvid(av2bv(avid))
    }

    #[inline(always)]
    pub fn to_avid(self) -> u64 {
        bv2av(self.0)
    }
}
