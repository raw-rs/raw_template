//! trash_ — minimal crate scaffold
//!
//! A tiny placeholder library used for project scaffolding and CI checks.
//!
//! # Example
//!
//! ```no_run
//! // Simple public API example (no_run so CI/doc tests don't execute it).
//! let greeting = raw_::greet();
//! println!("{}", greeting);
//! ```
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/239414506?s=400&&v=4")]
#![cfg_attr(not(feature = "std"), no_std)]

/// Returns a static greeting string.
///
/// This function provides a minimal public API for the crate, useful for
/// testing compilation, linting, and documentation generation.
///
/// # Examples
///
/// ```
/// let greeting = raw_::greet();
/// assert_eq!(greeting, "hello from raw_");
/// ```
#[must_use]
pub fn greet() -> &'static str {
    "hello from raw_"
}
