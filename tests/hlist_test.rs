use field_kinds::{
    Bool, FieldCount, FieldInfo, FieldVisitor, HListVisitor, Numeric, Text, VisitFields,
};
use frunk::{HCons, HNil};

struct EmptyStruct {}

struct OneFieldStruct {
    alpha: i32,
}

struct TwoFieldStruct {
    alpha: i32,
    beta: String,
}

struct ThreeFieldStruct {
    alpha: i32,
    beta: String,
    gamma: bool,
}

#[derive(Clone, Copy)]
struct Alpha;
impl FieldInfo for Alpha {
    const NAME: &'static str = "alpha";
    const SERIALIZED_NAME: &'static str = "alpha";
    const CATEGORY_NAME: &'static str = "numeric";
    const TAGS: &'static [&'static str] = &[];
    type Value = i32;
    type Category = Numeric;
}

#[derive(Clone, Copy)]
struct Beta;
impl FieldInfo for Beta {
    const NAME: &'static str = "beta";
    const SERIALIZED_NAME: &'static str = "beta";
    const CATEGORY_NAME: &'static str = "text";
    const TAGS: &'static [&'static str] = &[];
    type Value = String;
    type Category = Text;
}

#[derive(Clone, Copy)]
struct Gamma;
impl FieldInfo for Gamma {
    const NAME: &'static str = "gamma";
    const SERIALIZED_NAME: &'static str = "gamma";
    const CATEGORY_NAME: &'static str = "bool";
    const TAGS: &'static [&'static str] = &[];
    type Value = bool;
    type Category = Bool;
}

type HList0 = HNil;
type HList1 = HCons<Alpha, HNil>;
type HList2 = HCons<Alpha, HCons<Beta, HNil>>;
type HList3 = HCons<Alpha, HCons<Beta, HCons<Gamma, HNil>>>;

#[test]
fn field_count_empty() {
    assert_eq!(HList0::COUNT, 0);
}

#[test]
fn field_count_one() {
    assert_eq!(HList1::COUNT, 1);
}

#[test]
fn field_count_two() {
    assert_eq!(HList2::COUNT, 2);
}

#[test]
fn field_count_three() {
    assert_eq!(HList3::COUNT, 3);
}

struct CountingVisitor(usize);

impl FieldVisitor for CountingVisitor {
    fn visit<F: FieldInfo>(&mut self) {
        self.0 += 1;
    }
}

#[test]
fn hlist_visitor_empty() {
    let mut v = CountingVisitor(0);
    HList0::visit_all(&mut v);
    assert_eq!(v.0, 0);
}

#[test]
fn hlist_visitor_one() {
    let mut v = CountingVisitor(0);
    HList1::visit_all(&mut v);
    assert_eq!(v.0, 1);
}

#[test]
fn hlist_visitor_three() {
    let mut v = CountingVisitor(0);
    HList3::visit_all(&mut v);
    assert_eq!(v.0, 3);
}

// Проверяем порядок обхода
struct OrderVisitor(Vec<&'static str>);

impl FieldVisitor for OrderVisitor {
    fn visit<F: FieldInfo>(&mut self) {
        self.0.push(F::NAME);
    }
}

#[test]
fn hlist_visitor_order() {
    let mut v = OrderVisitor(Vec::new());
    HList3::visit_all(&mut v);
    assert_eq!(v.0, vec!["alpha", "beta", "gamma"]);
}

#[test]
fn count_matches_visits() {
    let mut v = CountingVisitor(0);
    HList3::visit_all(&mut v);
    assert_eq!(HList3::COUNT, v.0);
}

struct CategoryVisitor(Vec<&'static str>);

impl FieldVisitor for CategoryVisitor {
    fn visit<F: FieldInfo>(&mut self) {
        self.0.push(F::CATEGORY_NAME);
    }
}

#[test]
fn hlist_visitor_categories() {
    let mut v = CategoryVisitor(Vec::new());
    HList3::visit_all(&mut v);
    assert_eq!(v.0, vec!["numeric", "text", "bool"]);
}
