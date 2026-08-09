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
    fn mat_count(&self) -> Mat {
        let m: Mat = self.akshara.into();
        return m;
    }

    pub fn mat_to_ava(&self, m: Mat) -> Ava {
        let (rounds, r1) = (m.0 / self.mat_count().0, m.0 % self.mat_count().0);
        let (aks, m) = (r1 / self.nadai, r1 % self.nadai);
        return Ava::new(rounds, Aks::new(self.nadai, aks), Mat(m));
    }
}

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
