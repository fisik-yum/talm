use std::ops::{Add, Div, Mul};

use crate::talam;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mat(pub usize);

impl Add for Mat {
    type Output = Mat;
    fn add(self, rhs: Self) -> Self::Output {
        Mat(self.0 + rhs.0)
    }
}
impl Add<Aks> for Mat {
    type Output = Mat;
    fn add(self, rhs: Aks) -> Self::Output {
        Mat(self.0 + rhs.nadai * rhs.count)
    }
}
impl Div<talam::Talam> for Mat {
    type Output = talam::Ava;
    fn div(self, rhs: talam::Talam) -> Self::Output {
        return rhs.mat_to_ava(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Aks {
    nadai: usize, // should this be a Mat qty?
    count: usize,
}
impl Aks {
    pub fn new(n: usize, c: usize) -> Self {
        Self { nadai: n, count: c }
    }

    pub fn from_mat(m: Mat, n: usize) -> Self {
        return Self {
            nadai: n,
            count: m.0 / n,
        };
    }
}
impl From<Aks> for Mat {
    fn from(value: Aks) -> Mat {
        return Mat(value.nadai * value.count);
    }
}

impl Add for Aks {
    type Output = Mat;
    fn add(self, rhs: Self) -> Self::Output {
        Mat(self.nadai * self.count + rhs.nadai * rhs.count)
    }
}
impl Add<Mat> for Aks {
    type Output = Mat;
    fn add(self, rhs: Mat) -> Self::Output {
        Mat(self.nadai * self.count + rhs.0)
    }
}

impl Mul<usize> for Aks {
    type Output = Aks;
    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            nadai: self.nadai,
            count: self.count * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};

    use super::*;

    #[test]
    fn test_mat_add() {
        let lhs = Mat(8);
        let rhs = Mat(16);
        let res = lhs + rhs;
        assert!(res.type_id() == TypeId::of::<Mat>());
        assert_eq!(res, Mat(24));
    }

    #[test]
    fn test_aks_add() {
        let lhs = Aks { nadai: 4, count: 5 };
        let rhs = Aks { nadai: 5, count: 8 };
        let res = lhs + rhs;
        assert!(res.type_id() == TypeId::of::<Mat>());
        assert_eq!(res, Mat(60));
    }

    #[test]
    fn test_aks_add_mat() {
        let lhs = Aks { nadai: 4, count: 5 };
        let rhs = Mat(10);
        let res = lhs + rhs;
        assert_eq!(res, Mat(30));
    }

    #[test]
    fn test_mat_add_aks() {
        let lhs = Mat(10);
        let rhs = Aks { nadai: 4, count: 5 };
        let res = lhs + rhs;
        assert_eq!(res, Mat(30));
    }

    #[test]
    fn test_aks_mul_usize() {
        let a = Aks { nadai: 4, count: 5 };
        let res = a * 3;
        assert_eq!(res, Aks { nadai: 4, count: 15 });
    }

    #[test]
    fn test_aks_from_mat() {
        let m = Mat(20);
        let a = Aks::from_mat(m, 4);
        assert_eq!(a, Aks { nadai: 4, count: 5 });
    }

    #[test]
    fn test_mat_from_aks() {
        let a = Aks { nadai: 4, count: 5 };
        let m: Mat = a.into();
        assert_eq!(m, Mat(20));
    }
}
