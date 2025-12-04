use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

/// Trait for type category markers.
///
/// Implemented by category marker types ([`Numeric`], [`Text`], etc.).
pub trait TypeCategory: 'static + Copy {
    /// String name of the category (e.g., "numeric", "text").
    const NAME: &'static str;
}

/// Marker type for numeric types (`i8`-`i128`, `u8`-`u128`, `f32`, `f64`, `isize`, `usize`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Numeric;

/// Marker type for text types (`String`, `&str`, `Box<str>`, `char`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Text;

/// Marker type for boolean type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bool;

/// Marker type for optional types (`Option<T>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Optional;

/// Marker type for collection types (`Vec`, `HashSet`, `HashMap`, arrays, slices).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Collection;

/// Marker type for types that don't match any known category.
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

/// Trait for mapping Rust types to their categories.
///
/// Implement this trait for custom types to enable automatic categorization.
///
/// # Example
///
/// ```rust
/// use field_kinds::{Categorized, Numeric};
///
/// struct MyNumber(i32);
///
/// impl Categorized for MyNumber {
///     type Category = Numeric;
/// }
/// ```
pub trait Categorized {
    /// The category marker type for this type.
    type Category: TypeCategory;
}

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

impl Categorized for f32 {
    type Category = Numeric;
}
impl Categorized for f64 {
    type Category = Numeric;
}

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

impl Categorized for bool {
    type Category = Bool;
}

impl<T> Categorized for Option<T> {
    type Category = Optional;
}

impl<T> Categorized for Vec<T> {
    type Category = Collection;
}
impl<T, const N: usize> Categorized for [T; N] {
    type Category = Collection;
}
impl<T> Categorized for &[T] {
    type Category = Collection;
}
impl<T, S> Categorized for HashSet<T, S> {
    type Category = Collection;
}
impl<T> Categorized for BTreeSet<T> {
    type Category = Collection;
}
impl<K, V, S> Categorized for HashMap<K, V, S> {
    type Category = Collection;
}
impl<K, V> Categorized for BTreeMap<K, V> {
    type Category = Collection;
}
