//! This crate provides Dockerfile language support for the tree-sitter parsing library.
//!
//! The grammar exposes a `LanguageFn` so downstream crates using tree-sitter 0.26+
//! can convert it into their local `tree_sitter::Language` type without linking an
//! older copy of the tree-sitter runtime.

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_dockerfile() -> *const ();
}

/// The tree-sitter language for this grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_dockerfile) };

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &'static str = include_str!("../../src/node-types.json");
