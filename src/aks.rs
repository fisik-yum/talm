use crate::unit::Mathrai;
use std::fmt;
use std::ops::{Add, Mul};

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Akshara {
    pub nadai: Mathrai,
    pub count: usize,
}
impl Akshara {
    pub fn new(n: Mathrai, c: usize) -> Self {
        Self { nadai: n, count: c }
    }

    pub fn from_mat(m: Mathrai, n: Mathrai) -> Self {
        return Self {
            nadai: n,
            count: m.0 / n.0,
        };
    }
}

impl Add for Akshara {
    type Output = Mathrai;
    fn add(self, rhs: Self) -> Self::Output {
        Mathrai(self.nadai.0 * self.count + rhs.nadai.0 * rhs.count)
    }
}
impl Add<Mathrai> for Akshara {
    type Output = PartialAkshara;
    fn add(self, rhs: Mathrai) -> Self::Output {
        PartialAkshara {
            nadai: self.nadai,
            count: self.count + rhs.0 / self.nadai.0,
            mathrai: Mathrai(rhs.0 % self.nadai.0),
        }
    }
}
impl Add<PartialAkshara> for Akshara {
    type Output = Mathrai;
    fn add(self, rhs: PartialAkshara) -> Self::Output {
        Mathrai::from(self) + Mathrai::from(rhs)
    }
}
impl Mul<usize> for Akshara {
    type Output = Akshara;
    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            nadai: self.nadai,
            count: self.count * rhs,
        }
    }
}
impl fmt::Display for Akshara {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}a", self.count)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartialAkshara {
    pub nadai: Mathrai,
    pub count: usize,
    pub mathrai: Mathrai,
}
impl Add for PartialAkshara {
    type Output = PartialAkshara;
    fn add(self, rhs: Self) -> Self::Output {
        if self.nadai != rhs.nadai {
            panic!("cannot add partial aksharas with different nadai")
        }
        PartialAkshara {
            nadai: self.nadai,
            count: self.count + rhs.count,
            mathrai: self.mathrai + rhs.mathrai,
        }
    }
}
impl fmt::Display for PartialAkshara {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}a{}m", self.count, self.mathrai.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Carry {
    pub num: usize,
    pub den: usize,
}
impl Carry {
    pub const ZERO: Carry = Carry { num: 0, den: 1 };
    pub const HALF: Carry = Carry { num: 1, den: 2 };
    pub const QUARTER: Carry = Carry { num: 1, den: 4 };

    pub fn new(num: usize, den: usize) -> Self {
        if num == 0 {
            return Carry::ZERO;
        }
        let g = gcd(num, den);
        Carry {
            num: num / g,
            den: den / g,
        }
    }

    pub fn from_extra(extra: Mathrai, nadai: Mathrai) -> Self {
        Carry::new(extra.0, nadai.0)
    }

    pub fn to_mathrai(self, nadai: Mathrai) -> Mathrai {
        Mathrai(nadai.0 * self.num / self.den)
    }
}
impl Add for Carry {
    type Output = (usize, Carry);
    fn add(self, rhs: Self) -> Self::Output {
        let den = lcm(self.den, rhs.den);
        let num = self.num * (den / self.den) + rhs.num * (den / rhs.den);
        let whole = num / den;
        let rem = num % den;
        if rem == 0 {
            return (whole, Carry::ZERO);
        }
        let g = gcd(rem, den);
        (whole, Carry { num: rem / g, den: den / g })
    }
}
impl fmt::Display for Carry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num as f64 / self.den as f64)
    }
}

pub struct StandardAkshara {
    pub count: usize,
    pub edam: Carry,
}
impl From<Akshara> for StandardAkshara {
    fn from(value: Akshara) -> Self {
        Self {
            count: value.count,
            edam: Carry::ZERO,
        }
    }
}
impl From<PartialAkshara> for StandardAkshara {
    fn from(value: PartialAkshara) -> Self {
        Self {
            count: value.count + value.mathrai.0 / value.nadai.0,
            edam: Carry::from_extra(Mathrai(value.mathrai.0 % value.nadai.0), value.nadai),
        }
    }
}
impl StandardAkshara {
    pub fn from_mathrai(m: Mathrai, nadai: Mathrai) -> Self {
        Self {
            count: m.0 / nadai.0,
            edam: Carry::from_extra(Mathrai(m.0 % nadai.0), nadai),
        }
    }
}
impl Add for StandardAkshara {
    type Output = StandardAkshara;
    fn add(self, rhs: Self) -> Self::Output {
        let (whole, edam) = self.edam + rhs.edam;
        StandardAkshara {
            count: self.count + rhs.count + whole,
            edam,
        }
    }
}
impl fmt::Display for StandardAkshara {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}a{}e", self.count, self.edam)
    }
}
