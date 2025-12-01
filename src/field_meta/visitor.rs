use crate::field_meta::field_info::FieldInfo;

pub trait FieldVisitor {
    fn visit<F: FieldInfo>(&mut self);
}

pub trait VisitFields {
    fn visit_fields<V: FieldVisitor>(visitor: &mut V);
}
