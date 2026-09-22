// T8-GULT: Sextic Regulator
use crate::base60_math::{SexagesimalRatio, CCZ_14_GATE_LIMIT, T8_BASE};

#[derive(Debug, Clone, Copy)]
pub struct OamExhaust {
    pub topological_charge: i128,
    pub phase_shift: SexagesimalRatio,
}

pub struct SexticRegulator {
    pub eta_stiffness: SexagesimalRatio,
}

impl SexticRegulator {
    pub fn new() -> Self {
        SexticRegulator { eta_stiffness: SexagesimalRatio::new(1, 1) }
    }
    pub fn apply_topological_brake(&self, current_ak: u8, dissonance: SexagesimalRatio) -> (SexagesimalRatio, Option<OamExhaust>) {
        if current_ak > CCZ_14_GATE_LIMIT { panic!("Exceeded 14-Gate limit."); }
        let limit_prox = (CCZ_14_GATE_LIMIT - current_ak) as i128;
        
        if limit_prox == 0 {
            let exhaust = OamExhaust {
                topological_charge: dissonance.numerator,
                phase_shift: SexagesimalRatio::new(dissonance.numerator % T8_BASE, T8_BASE),
            };
            return (SexagesimalRatio::new(self.eta_stiffness.numerator * T8_BASE, 1), Some(exhaust));
        }
        let friction = SexagesimalRatio::new(self.eta_stiffness.numerator * T8_BASE, self.eta_stiffness.denominator * limit_prox);
        let reg_state = SexagesimalRatio::new(dissonance.numerator * friction.denominator, dissonance.denominator * friction.numerator);
        (reg_state, None)
    }
}
