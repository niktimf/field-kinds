# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/niktimf/field-kinds/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/niktimf/field-kinds/releases/tag/v0.1.0
