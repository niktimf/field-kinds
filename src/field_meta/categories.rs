use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

pub trait TypeCategory: 'static + Copy {
    const NAME: &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Numeric;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Text;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bool;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Optional;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Collection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unknown;

impl TypeCategory for Numeric {
    const NAME: &'static str = "numeric";
}
impl TypeCategory for Text {
    const NAME: &'static str = "text";
}
impl TypeCategory for Bool {
    const NAME: &'static str = "bool";
}
impl TypeCategory for Optional {
    const NAME: &'static str = "optional";
}
impl TypeCategory for Collection {
    const NAME: &'static str = "collection";
}
impl TypeCategory for Unknown {
    const NAME: &'static str = "unknown";
}

pub trait IsNumeric {}
impl IsNumeric for Numeric {}

pub trait IsText {}
impl IsText for Text {}

pub trait IsBool {}
impl IsBool for Bool {}

pub trait IsOptional {}
impl IsOptional for Optional {}

pub trait IsCollection {}
impl IsCollection for Collection {}

pub trait Categorized {
    type Category: TypeCategory;
}

// Numeric
// u*
impl Categorized for u8 {
    type Category = Numeric;
}
impl Categorized for u16 {
    type Category = Numeric;
}
impl Categorized for u32 {
    type Category = Numeric;
}
impl Categorized for u64 {
    type Category = Numeric;
}
impl Categorized for u128 {
    type Category = Numeric;
}
impl Categorized for usize {
    type Category = Numeric;
}

// i*
impl Categorized for i8 {
    type Category = Numeric;
}
impl Categorized for i16 {
    type Category = Numeric;
}
impl Categorized for i32 {
    type Category = Numeric;
}
impl Categorized for i64 {
    type Category = Numeric;
}
impl Categorized for i128 {
    type Category = Numeric;
}
impl Categorized for isize {
    type Category = Numeric;
}

// f*
impl Categorized for f32 {
    type Category = Numeric;
}
impl Categorized for f64 {
    type Category = Numeric;
}

// Text
impl Categorized for String {
    type Category = Text;
}
impl Categorized for &str {
    type Category = Text;
}
impl Categorized for Box<str> {
    type Category = Text;
}
impl Categorized for char {
    type Category = Text;
}

// Bool
impl Categorized for bool {
    type Category = Bool;
}

// Optional
impl<T> Categorized for Option<T> {
    type Category = Optional;
}

// Collection
impl<T> Categorized for Vec<T> {
    type Category = Collection;
}
impl<T, const N: usize> Categorized for [T; N] {
    type Category = Collection;
}
impl<T> Categorized for &[T] {
    type Category = Collection;
}
impl<T> Categorized for HashSet<T> {
    type Category = Collection;
}
impl<T> Categorized for BTreeSet<T> {
    type Category = Collection;
}
impl<K, V> Categorized for HashMap<K, V> {
    type Category = Collection;
}
impl<K, V> Categorized for BTreeMap<K, V> {
    type Category = Collection;
}
