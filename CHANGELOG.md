# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-05-19

### Added

- `JsCompare` trait on `serde_json::Value` with `js_eq`, `js_strict_eq`,
  `js_lt`, `js_le`, `js_gt`, `js_ge`, `js_ne`, `js_strict_ne`.
- `JsonTruthy` trait on `serde_json::Value` with `is_truthy`.
- `JsValue` newtype wrapper that implements `PartialEq` via `js_eq` so values
  can be compared with `==` directly.
- ~190 unit tests covering the JS coercion table.
- Node.js cross-check integration test (`#[ignore]` by default).
