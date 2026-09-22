// T8-GULT: PyO3 Binding Interface
use pyo3::prelude::*;
mod base60_math; mod sextic_regulator; mod eml_operator; mod assembly_index_enforcer;
use base60_math::{SexagesimalRatio as CoreRatio, T8_BASE as CORE_T8_BASE};
use assembly_index_enforcer::AssemblyIndexEnforcer as CoreEnforcer;

#[pymodule]
fn eml_rust_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("T8_BASE", CORE_T8_BASE)?;
    m.add_class::<SexagesimalRatio>()?;
    m.add_class::<AssemblyIndexEnforcer>()?;
    Ok(())
}

#[pyclass] #[derive(Clone)]
pub struct SexagesimalRatio { pub inner: CoreRatio }
#[pymethods]
impl SexagesimalRatio {
    #[new] fn new(num: i128, den: i128) -> PyResult<Self> { Ok(SexagesimalRatio { inner: CoreRatio::new(num, den) }) }
    #[getter] fn numerator(&self) -> i128 { self.inner.numerator }
    #[getter] fn denominator(&self) -> i128 { self.inner.denominator }
}

#[pyclass]
pub struct AssemblyIndexEnforcer { inner: CoreEnforcer }
#[pymethods]
impl AssemblyIndexEnforcer {
    #[new] fn new() -> Self { AssemblyIndexEnforcer { inner: CoreEnforcer::new() } }
}
