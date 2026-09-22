// T8-GULT: Assembly Index Enforcer
use crate::base60_math::{SexagesimalRatio, CCZ_14_GATE_LIMIT, T8_BASE};
use crate::sextic_regulator::{SexticRegulator, OamExhaust};

#[derive(Debug, Clone)]
pub struct TopologicalKnot {
    pub geometric_state: SexagesimalRatio,
    pub assembly_index: u8,
    pub thermodynamic_depth: SexagesimalRatio,
}

pub struct AssemblyIndexEnforcer {
    regulator: SexticRegulator,
}

impl AssemblyIndexEnforcer {
    pub fn new() -> Self { AssemblyIndexEnforcer { regulator: SexticRegulator::new() } }
    
    pub fn calculate_assembly_index(state: SexagesimalRatio) -> u8 {
        let mut ak = 1;
        let mut comp = (state.numerator.abs() / state.denominator.abs()).max(1);
        while comp >= T8_BASE && ak <= CCZ_14_GATE_LIMIT { comp /= T8_BASE; ak += 1; }
        ak
    }
    
    pub fn process_lattice_knot(&self, incoming: SexagesimalRatio) -> (TopologicalKnot, Option<OamExhaust>) {
        let mut ak = Self::calculate_assembly_index(incoming);
        let mut state = incoming;
        let mut exhaust = None;
        
        if ak >= CCZ_14_GATE_LIMIT {
            let (reg_state, exh) = self.regulator.apply_topological_brake(ak, incoming);
            state = reg_state; exhaust = exh; ak = CCZ_14_GATE_LIMIT;
        }
        
        let depth = SexagesimalRatio::new((T8_BASE as u128).pow(ak as u32) as i128, 1);
        (TopologicalKnot { geometric_state: state, assembly_index: ak, thermodynamic_depth: depth }, exhaust)
    }
}
