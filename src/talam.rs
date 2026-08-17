use crate::unit::{self, Aks, Mat};

#[derive(Debug, Clone, Copy)]
pub struct Talam {
    base: BaseTalam,
    pub laghu: usize,
    pub kallai: usize,
    pub nadai: usize,
    pub akshara: unit::Aks,
}

impl Talam {
    pub fn new(b: BaseTalam, l: usize, n: usize, k: usize) -> Self {
        Self {
            akshara: Self::count_a(&b, l, n, k),
            base: b,
            laghu: l,
            nadai: n,
            kallai: k,
        }
    }

    fn count_a(b: &BaseTalam, l: usize, n: usize, k: usize) -> Aks {
        match b {
            BaseTalam::Eka => return Aks::new(n, l) * k,
            BaseTalam::Roopaka => return Aks::new(n, l + 2) * k,
            BaseTalam::Triputa => return Aks::new(n, l + 4) * k,
            BaseTalam::Ata => return Aks::new(n, l * 2 + 4) * k,
            BaseTalam::Dhruva => return Aks::new(n, l * 3 + 2) * k,
            BaseTalam::Jampa => return Aks::new(n, l + 3) * k,
            BaseTalam::Matyama => return Aks::new(n, l * 2 + 2) * k,
        }
    }
    pub fn mat_count(&self) -> Mat {
        let m: Mat = self.akshara.into();
        return m;
    }

    pub fn mat_to_ava(&self, m: Mat) -> Ava {
        let (rounds, r1) = (m.0 / self.mat_count().0, m.0 % self.mat_count().0);
        let (aks, m) = (r1 / self.nadai, r1 % self.nadai);
        return Ava::new(rounds, Aks::new(self.nadai, aks), Mat(m));
    }
}

/*
 * Ava is a resultant type of division, there are no links to the talam it was constructed from.
 * */
pub struct Ava {
    rounds: usize,
    aks: Aks,
    remain: Mat,
}
impl Ava {
    fn new(rounds: usize, aks: Aks, remain: Mat) -> Self {
        Self {
            rounds,
            aks,
            remain,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseTalam {
    Eka,
    Roopaka,
    Triputa,
    Ata,
    Dhruva,
    Jampa,
    Matyama,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eka_akshara() {
        let t = Talam::new(BaseTalam::Eka, 4, 4, 1);
        assert_eq!(t.akshara, Aks::new(4, 4));
        assert_eq!(t.mat_count(), Mat(16));
    }

    #[test]
    fn test_roopaka_akshara() {
        let t = Talam::new(BaseTalam::Roopaka, 4, 4, 1);
        assert_eq!(t.akshara, Aks::new(4, 6));
        assert_eq!(t.mat_count(), Mat(24));
    }

    #[test]
    fn test_triputa_akshara() {
        let t = Talam::new(BaseTalam::Triputa, 2, 4, 1);
        assert_eq!(t.akshara, Aks::new(4, 6));
        assert_eq!(t.mat_count(), Mat(24));
    }

    #[test]
    fn test_ata_akshara() {
        let t = Talam::new(BaseTalam::Ata, 2, 4, 1);
        assert_eq!(t.akshara, Aks::new(4, 8));
        assert_eq!(t.mat_count(), Mat(32));
    }

    #[test]
    fn test_dhruva_akshara() {
        let t = Talam::new(BaseTalam::Dhruva, 2, 4, 1);
        assert_eq!(t.akshara, Aks::new(4, 8));
        assert_eq!(t.mat_count(), Mat(32));
    }

    #[test]
    fn test_jampa_akshara() {
        let t = Talam::new(BaseTalam::Jampa, 2, 4, 1);
        assert_eq!(t.akshara, Aks::new(4, 5));
        assert_eq!(t.mat_count(), Mat(20));
    }

    #[test]
    fn test_matyama_akshara() {
        let t = Talam::new(BaseTalam::Matyama, 2, 4, 1);
        assert_eq!(t.akshara, Aks::new(4, 6));
        assert_eq!(t.mat_count(), Mat(24));
    }

    #[test]
    fn test_kallai_multiplies_count() {
        let t = Talam::new(BaseTalam::Eka, 4, 4, 3);
        assert_eq!(t.akshara, Aks::new(4, 12));
        assert_eq!(t.mat_count(), Mat(48));
    }

    #[test]
    fn test_mat_to_ava_exact() {
        let t = Talam::new(BaseTalam::Eka, 4, 4, 1);
        let ava = t.mat_to_ava(Mat(16));
        assert_eq!(ava.rounds, 1);
        assert_eq!(ava.aks, Aks::new(4, 0));
        assert_eq!(ava.remain, Mat(0));
    }

    #[test]
    fn test_mat_to_ava_partial() {
        let t = Talam::new(BaseTalam::Eka, 4, 4, 1);
        let ava = t.mat_to_ava(Mat(10));
        assert_eq!(ava.rounds, 0);
        assert_eq!(ava.aks, Aks::new(4, 2));
        assert_eq!(ava.remain, Mat(2));
    }

    #[test]
    fn test_mat_to_ava_multiple_rounds() {
        let t = Talam::new(BaseTalam::Eka, 4, 4, 1);
        let ava = t.mat_to_ava(Mat(35));
        assert_eq!(ava.rounds, 2);
        assert_eq!(ava.aks, Aks::new(4, 0));
        assert_eq!(ava.remain, Mat(3));
    }

    #[test]
    fn test_mat_div_talam_operator() {
        let t = Talam::new(BaseTalam::Eka, 4, 4, 1);
        let ava = Mat(20) / t;
        assert_eq!(ava.rounds, 1);
        assert_eq!(ava.aks, Aks::new(4, 1));
        assert_eq!(ava.remain, Mat(0));
    }
}
