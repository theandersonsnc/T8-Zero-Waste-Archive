// T8-GULT: EML Operator
use crate::base60_math::{SexagesimalRatio, T8_BASE};

pub struct EmlOperator;

impl EmlOperator {
    pub fn apply_eml_shift(current: SexagesimalRatio, steps: u32) -> SexagesimalRatio {
        let exp_num = current.numerator * (T8_BASE.pow(steps) as i128);
        SexagesimalRatio::new(exp_num, current.denominator)
    }
}
