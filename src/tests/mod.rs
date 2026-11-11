// Copyright (C) 2023-2026 Pain001. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Unit tests for the native core.
//!
//! They call the crate's public surface only, so a change to how the
//! crate is laid out inside does not change what they check. Every
//! error message is compared in full: pain001 shows these strings to
//! its users, and a reworded message is a visible change.

mod batch;
mod bic;
mod charset;
mod iban;
mod module;

use pyo3::prelude::*;

/// Run `f` with an initialised interpreter attached to this thread.
pub(crate) fn with_py<F, R>(f: F) -> R
where
    F: for<'py> FnOnce(Python<'py>) -> R,
{
    Python::initialize();
    Python::attach(f)
}

/// Shorthand for a failed verdict with its message.
pub(crate) fn fail(message: &str) -> (bool, String) {
    (false, message.to_string())
}

/// Shorthand for a passing verdict.
pub(crate) fn pass() -> (bool, String) {
    (true, String::new())
}
