// Copyright (C) 2023-2026 Pain001. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::with_py;
use crate::validate_payment_batch;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::ffi::CStr;

/// Validate the Python list literal `records` and return the result's
/// fields as Python reprs, without the timing.
fn run(records: &CStr) -> Vec<(String, String)> {
    with_py(|py| {
        let list = py.eval(records, None, None).unwrap();
        let list = list.cast::<PyList>().unwrap();
        let result = validate_payment_batch(py, list, "pain.001.001.03").unwrap();
        fields(&result)
    })
}

fn fields(result: &Bound<'_, PyDict>) -> Vec<(String, String)> {
    let duration = result.get_item("duration_micros").unwrap().unwrap();
    assert!(duration.extract::<u128>().is_ok());
    [
        "valid",
        "message_type",
        "transaction_count",
        "control_sums",
        "errors",
        "duplicates",
    ]
    .iter()
    .map(|key| {
        let value = result.get_item(key).unwrap().unwrap();
        ((*key).to_string(), value.repr().unwrap().to_string())
    })
    .collect()
}

fn field(result: &[(String, String)], key: &str) -> String {
    result.iter().find(|(k, _)| k == key).unwrap().1.clone()
}

#[test]
fn a_clean_batch_sums_per_currency() {
    let result = run(c"[
        {'creditor_iban': 'DE89370400440532013000', 'creditor_bic': 'DEUTDEFF',
         'amount': '100.50', 'currency': 'eur', 'execution_date': '2026-10-01'},
        {'cdtr_iban': 'GB82WEST12345698765432', 'cdtr_bic': 'DEUTDEFF500',
         'amount': 250.0, 'currency': 'GBP'},
    ]");
    assert_eq!(field(&result, "valid"), "True");
    assert_eq!(field(&result, "message_type"), "'pain.001.001.03'");
    assert_eq!(field(&result, "transaction_count"), "2");
    assert_eq!(field(&result, "errors"), "[]");
    assert_eq!(field(&result, "duplicates"), "[]");
    let sums = field(&result, "control_sums");
    assert!(sums.contains("'EUR': 100.5"), "{sums}");
    assert!(sums.contains("'GBP': 250.0"), "{sums}");
}

#[test]
fn missing_fields_default_and_non_dicts_are_skipped() {
    let result = run(c"[{}, 42]");
    assert_eq!(field(&result, "valid"), "True");
    // The count is of the input list, skipped items included.
    assert_eq!(field(&result, "transaction_count"), "2");
    assert_eq!(field(&result, "control_sums"), "{'EUR': 0.0}");
}

#[test]
fn the_first_fault_of_a_record_is_reported() {
    let result = run(c"[
        {'creditor_iban': 'DE00370400440532013000', 'creditor_bic': 'BAD',
         'amount': 'x'},
        {'creditor_bic': 'BAD', 'amount': 'x'},
        {'amount': 'not-a-number'},
    ]");
    assert_eq!(field(&result, "valid"), "False");
    assert_eq!(
        field(&result, "errors"),
        "[{'row': 0, 'field': 'creditor_iban', 'error': \
         'IBAN checksum validation failed (mod 97 = 9, expected 1)'}, \
         {'row': 1, 'field': 'creditor_bic', 'error': \
         \"BIC must be 8 or 11 characters (got 3: 'BAD')\"}, \
         {'row': 2, 'field': 'amount', 'error': 'Invalid amount format'}]"
    );
    assert_eq!(field(&result, "control_sums"), "{}");
}

#[test]
fn repeated_records_are_flagged_after_the_first() {
    let result = run(c"[
        {'creditor_iban': 'DE89370400440532013000', 'amount': '1.00',
         'execution_date': '2026-10-01'},
        {'creditor_iban': 'DE89370400440532013000', 'amount': '1.00',
         'execution_date': '2026-10-01'},
    ]");
    assert_eq!(
        field(&result, "duplicates"),
        "[{'row': 1, 'fingerprint': 'DE89370400440532013000::1.00:EUR:2026-10-01'}]"
    );
    assert_eq!(field(&result, "control_sums"), "{'EUR': 2.0}");
}
