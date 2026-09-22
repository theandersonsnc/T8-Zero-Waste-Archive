// T8-GULT: Base-60 Exact-Ratio Constants
pub const T8_BASE: i128 = 60;
pub const CCZ_14_GATE_LIMIT: u8 = 14;
pub const BIO_OBSERVER_BASE: i128 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SexagesimalRatio {
    pub numerator: i128,
    pub denominator: i128, 
}

impl SexagesimalRatio {
    pub fn new(numerator: i128, denominator: i128) -> Self {
        if denominator == 0 { panic!("Singularity: Denominator cannot be zero."); }
        let mut ratio = SexagesimalRatio { numerator, denominator };
        ratio.reduce_to_base60();
        ratio
    }
    fn reduce_to_base60(&mut self) {
        let gcd = gcd(self.numerator.abs(), self.denominator.abs());
        self.numerator /= gcd;
        self.denominator /= gcd;
    }
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 { let temp = b; b = a % b; a = temp; }
    a
}
