use crate::field_meta::field_info::FieldInfo;

/// Terminator for a heterogeneous list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HNil;

/// A heterogeneous list node containing a head element and a tail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HCons<H, T> {
    /// The head element.
    pub head: H,
    /// The rest of the list.
    pub tail: T,
}

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
