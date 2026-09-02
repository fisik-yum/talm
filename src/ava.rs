use crate::aks::{self, Carry};

pub struct Avartana {
    count: usize,
    carry: aks::Carry,
}
impl Avartana {
    pub fn from_standard(value: aks::StandardAkshara, aksharas_per_cycle: usize) -> Self {
        Self {
            count: value.count / aksharas_per_cycle,
            carry: Carry::new(
                (value.count % aksharas_per_cycle) * value.edam.den + value.edam.num,
                value.edam.den * aksharas_per_cycle,
            ),
        }
    }
}
