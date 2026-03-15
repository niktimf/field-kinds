use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::num::{
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
};
use std::sync::Arc;

/// A type-safe wrapper for field category names.
///
/// Provides compile-time constants for built-in categories.
/// Custom categories are created by implementing [`TypeCategory`].
///
/// # Example
///
/// ```rust
/// use field_kinds::{Category, TypeCategory};
///
/// // Built-in categories
/// let category = Category::NUMERIC;
/// assert_eq!(category.name(), "numeric");
///
/// // Custom category via TypeCategory
/// #[derive(Debug, Clone, Copy)]
/// struct Money;
///
/// impl TypeCategory for Money {
///     const NAME: &'static str = "money";
/// }
///
/// assert_eq!(Money::CATEGORY.name(), "money");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Category(&'static str);

impl Category {
    #[doc(hidden)]
    const fn new(name: &'static str) -> Self {
        Self(name)
    }

    /// Returns the underlying category name.
    pub const fn name(&self) -> &'static str {
        self.0
    }
}

impl PartialEq<&str> for Category {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Category> for &str {
    fn eq(&self, other: &Category) -> bool {
        *self == other.0
    }
}

impl core::fmt::Display for Category {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.0)
    }
}

/// Trait for type category markers.
///
/// Implemented by category marker types ([`Numeric`], [`Text`], etc.).
pub trait TypeCategory: 'static + Copy {
    /// String name of the category (e.g., "numeric", "text").
    const NAME: &'static str;

    /// The [`Category`] value for this type category.
    const CATEGORY: Category = Category::new(Self::NAME);
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

// Associated constants referencing TypeCategory::NAME as single source of truth.
// Defined after marker impls so the constants can reference them.
impl Category {
    /// Numeric types (`i8`-`i128`, `u8`-`u128`, `f32`, `f64`, `isize`, `usize`).
    pub const NUMERIC: Self = <Numeric as TypeCategory>::CATEGORY;
    /// Text types (`String`, `&str`, `Box<str>`, `char`).
    pub const TEXT: Self = <Text as TypeCategory>::CATEGORY;
    /// Boolean type.
    pub const BOOL: Self = <Bool as TypeCategory>::CATEGORY;
    /// Optional types (`Option<T>`).
    pub const OPTIONAL: Self = <Optional as TypeCategory>::CATEGORY;
    /// Collection types (`Vec`, `HashSet`, `HashMap`, arrays, slices).
    pub const COLLECTION: Self = <Collection as TypeCategory>::CATEGORY;
    /// Types that don't match any known category.
    pub const UNKNOWN: Self = <Unknown as TypeCategory>::CATEGORY;
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

impl Categorized for NonZeroU8 {
    type Category = Numeric;
}
impl Categorized for NonZeroU16 {
    type Category = Numeric;
}
impl Categorized for NonZeroU32 {
    type Category = Numeric;
}
impl Categorized for NonZeroU64 {
    type Category = Numeric;
}
impl Categorized for NonZeroU128 {
    type Category = Numeric;
}
impl Categorized for NonZeroUsize {
    type Category = Numeric;
}
impl Categorized for NonZeroI8 {
    type Category = Numeric;
}
impl Categorized for NonZeroI16 {
    type Category = Numeric;
}
impl Categorized for NonZeroI32 {
    type Category = Numeric;
}
impl Categorized for NonZeroI64 {
    type Category = Numeric;
}
impl Categorized for NonZeroI128 {
    type Category = Numeric;
}
impl Categorized for NonZeroIsize {
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
impl<'a> Categorized for Cow<'a, str> {
    type Category = Text;
}
impl Categorized for Arc<str> {
    type Category = Text;
}
impl Categorized for std::rc::Rc<str> {
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
