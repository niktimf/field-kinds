use crate::field_meta::field_info::FieldInfo;
use crate::field_meta::visitor::FieldVisitor;
use frunk::{HCons, HNil};

pub trait FieldCount {
    const COUNT: usize;
}

impl FieldCount for HNil {
    const COUNT: usize = 0;
}

impl<H: FieldInfo, T: FieldCount> FieldCount for HCons<H, T> {
    const COUNT: usize = 1 + T::COUNT;
}

pub trait HListVisitor {
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
