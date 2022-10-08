//! The navigator is bacially a tree
//! without values.
//! It implements the structure and logic
//! on how the tree is build and how nodes find
//! there parents as well as their children.
//!
//! How the navigator does this, is an implementation detail.
//! Currently it stores an vector of neighboring information,
//! but don't be sure that this documentation changes when
//! that is changed.

mod builder;
mod navigator;
mod neighbors;

// We use the navigator and builder only in this crate!
pub(crate) use builder::Builder;
pub(crate) use navigator::Navigator;
pub use neighbors::Neighbors;
