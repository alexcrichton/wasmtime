//! Defines x64 instructions using the DSL.

mod add;
mod and;
mod mul;
mod or;
mod shld;
mod sub;
mod xor;

use crate::dsl::Inst;

#[must_use]
pub fn list() -> Vec<Inst> {
    let mut all = vec![];
    all.extend(add::list());
    all.extend(and::list());
    all.extend(mul::list());
    all.extend(or::list());
    all.extend(shld::list());
    all.extend(sub::list());
    all.extend(xor::list());
    all
}
