use crate::field_meta::field_info::FieldInfo;
use crate::field_meta::visitors::FieldVisitor;
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

/// Trait for visiting all fields in an `HList`.
pub trait HListVisitor {
    /// Visits all fields in the `HList` with the given visitor.
    fn visit_all<V: FieldVisitor>(visitor: &mut V);
}

impl HListVisitor for HNil {
    fn visit_all<V: FieldVisitor>(_visitor: &mut V) {}
}

impl<H: FieldInfo, T: HListVisitor> HListVisitor for HCons<H, T> {
    fn visit_all<V: FieldVisitor>(visitor: &mut V) {
        visitor.visit::<H>();
        T::visit_all(visitor);
    }
}
