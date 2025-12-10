use crate::field_meta::field_info::FieldInfo;
use frunk::{HCons, HNil};

/// Trait for counting fields in an `HList`.
pub trait FieldCount {
    /// Number of fields.
    const COUNT: usize;
}

impl FieldCount for HNil {
    const COUNT: usize = 0;
}

impl<H: FieldInfo, T: FieldCount> FieldCount for HCons<H, T> {
    const COUNT: usize = 1 + T::COUNT;
}

/// Marker trait for `HLists` of field types.
pub trait HListVisitor {}

impl HListVisitor for HNil {}

impl<H: FieldInfo, T: HListVisitor> HListVisitor for HCons<H, T> {}
