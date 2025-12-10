# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2025-12-10

### Added

- `FieldMeta::has_category()` - const fn for checking field category
- `FieldMeta::matches()` - check if field matches multiple criteria (name, category, tag)
- `FieldKindsExt::find_by_name()` - find field metadata by original name
- `FieldKindsExt::find_by_serialized_name()` - find field metadata by serialized name
- `FieldKindsExt::filter_by_category()` - iterator over fields with given category
- `FieldKindsExt::filter_by_tag()` - iterator over fields with given tag
- `FieldKindsExt::field_names_iter()` - iterator over original field names
- `FieldKindsExt::serialized_names_iter()` - iterator over serialized field names

## [0.2.0] - 2025-12-10

### Changed

- **Breaking**: Replaced `FieldVisitor` pattern with static `const FIELDS: &'static [FieldMeta]`
- Field metadata is now available as a compile-time constant slice
- Removed `FieldVisitor` trait and all visitor structs (`CollectMeta`, `CollectNames`, etc.)
- `FieldMeta` no longer contains `type_name` field

### Added

- `FieldMeta::has_tag()` const fn for compile-time tag checking

## [0.1.0] - 2024-12-04

### Added

- `FieldKinds` derive macro for compile-time field introspection
- Automatic type categorization (numeric, text, bool, optional, collection)
- Support for `#[serde(rename)]` and `#[serde(rename_all)]` attributes
- Custom field tags via `#[field_tags(...)]` attribute
- Field skipping via `#[field_kinds(skip)]` attribute
- `FieldVisitor` trait for extensible field processing
- `FieldKindsExt` extension trait with convenience methods:
  - `field_names()` - get original field names
  - `serialized_names()` - get serialized field names
  - `fields_by_category()` - filter fields by type category
  - `fields_by_tag()` - filter fields by custom tag
  - `has_field()` - check field existence
  - `field_category()` - get category for a field
  - `field_meta()` - get full metadata for all fields

[Unreleased]: https://github.com/niktimf/field-kinds/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/niktimf/field-kinds/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/niktimf/field-kinds/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/niktimf/field-kinds/releases/tag/v0.1.0
