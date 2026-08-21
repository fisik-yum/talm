use crate::aks::{Akshara, PartialAkshara};
use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct Mathrai(pub usize);

impl Add for Mathrai {
    type Output = Mathrai;
    fn add(self, rhs: Self) -> Self::Output {
        Mathrai(self.0 + rhs.0)
    }
}

impl Mul<usize> for Mathrai {
    type Output = Mathrai;
    fn mul(self, rhs: usize) -> Self::Output {
        return Mathrai(self.0 * rhs);
    }
}

impl From<Akshara> for Mathrai {
    fn from(value: Akshara) -> Self {
        return value.nadai * value.count;
    }
}
impl From<PartialAkshara> for Mathrai {
    fn from(value: PartialAkshara) -> Self {
        return value.nadai * value.count + value.mathrai;
    }
}

impl fmt::Display for Mathrai {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}m", self.0)
    }
}
