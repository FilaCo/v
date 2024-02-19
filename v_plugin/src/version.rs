use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Version {
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn major(&self) -> &u8 {
        &self.major
    }

    pub fn minor(&self) -> &u8 {
        &self.minor
    }

    pub fn patch(&self) -> &u8 {
        &self.patch
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.major() != other.major() {
            return self.major().cmp(other.major());
        }

        if self.minor() != other.minor() {
            return self.minor().cmp(other.minor());
        }

        self.patch().cmp(other.patch())
    }
}

#[cfg(test)]
mod tests {
    
}
