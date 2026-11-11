// Copyright (C) 2023-2026 Pain001. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::with_py;
use pyo3::prelude::*;
use pyo3::types::PyModule;

#[test]
fn the_module_exports_every_binding() {
    with_py(|py| {
        let module = PyModule::new(py, "_pain001_fast").unwrap();
        crate::_pain001_fast(&module).unwrap();
        for name in [
            "validate_iban",
            "validate_iban_detailed",
            "validate_bic",
            "validate_bic_detailed",
            "is_valid_charset",
            "sanitize_to_charset",
            "find_invalid_characters",
            "validate_payment_batch",
        ] {
            assert!(module.hasattr(name).unwrap(), "{name}");
        }
        let verdict: bool = module
            .getattr("validate_iban")
            .unwrap()
            .call1(("DE89370400440532013000",))
            .unwrap()
            .extract()
            .unwrap();
        assert!(verdict);
    });
}
