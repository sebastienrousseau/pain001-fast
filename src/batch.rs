// Copyright (C) 2023-2026 Pain001. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Batch validation: read the records under the GIL, validate them in
//! parallel with Rayon, then aggregate control sums, errors and
//! duplicates back into Python objects.

use std::collections::{HashMap, HashSet};
use std::time::Instant;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use rayon::prelude::*;

use crate::{validate_bic_detailed_fast, validate_iban_detailed_fast};

/// The fields of one record, read out of its Python dict as text.
struct RecordData {
    row: usize,
    creditor_iban: String,
    creditor_bic: String,
    amount_str: String,
    currency: String,
    execution_date: String,
}

/// What validation found for one record.
struct ValidatedRecord {
    row: usize,
    error: Option<(&'static str, String)>,
    parsed_amount: Option<f64>,
    currency: String,
    fingerprint: String,
}

/// The aggregated result, still to be written into a Python dict.
struct Summary<'py> {
    control_sums: HashMap<String, f64>,
    errors: Bound<'py, PyList>,
    duplicates: Bound<'py, PyList>,
}

/// Validate a list of payment records in parallel.
///
/// Each record is a dict; any other item is skipped, though it still
/// counts in `transaction_count`. The fields read are `creditor_iban`
/// (or `cdtr_iban`), `creditor_bic` (or `cdtr_bic`), `amount` (default
/// `0.0`), `currency` (default `EUR`, uppercased) and `execution_date`,
/// each through `str()`. A record reports its first fault only, in
/// the order IBAN, BIC, amount. A record whose IBAN, BIC, amount text,
/// currency and execution date all equal an earlier record's is listed
/// in `duplicates`.
///
/// Returns a dict with `valid`, `message_type`, `transaction_count`,
/// `control_sums` (per currency, over the amounts that parsed),
/// `errors` (`row`, `field`, `error`), `duplicates` (`row`,
/// `fingerprint`) and `duration_micros`.
#[pyfunction]
#[pyo3(signature = (records, message_type = "pain.001.001.03"))]
pub fn validate_payment_batch<'py>(
    py: Python<'py>,
    records: &Bound<'py, PyList>,
    message_type: &str,
) -> PyResult<Bound<'py, PyDict>> {
    let start_time = Instant::now();
    let parsed = read_records(records)?;
    let validated: Vec<ValidatedRecord> = parsed.par_iter().map(validate_record).collect();
    let summary = summarise(py, validated)?;

    let result = PyDict::new(py);
    result.set_item("valid", summary.errors.is_empty())?;
    result.set_item("message_type", message_type)?;
    result.set_item("transaction_count", records.len())?;
    result.set_item("control_sums", summary.control_sums)?;
    result.set_item("errors", summary.errors)?;
    result.set_item("duplicates", summary.duplicates)?;
    result.set_item("duration_micros", start_time.elapsed().as_micros())?;
    Ok(result)
}

fn read_records(records: &Bound<'_, PyList>) -> PyResult<Vec<RecordData>> {
    let mut parsed = Vec::with_capacity(records.len());
    for (row, item) in records.iter().enumerate() {
        if let Ok(dict) = item.cast::<PyDict>() {
            parsed.push(read_record(row, dict)?);
        }
    }
    Ok(parsed)
}

fn read_record(row: usize, dict: &Bound<'_, PyDict>) -> PyResult<RecordData> {
    Ok(RecordData {
        row,
        creditor_iban: text(dict, &["creditor_iban", "cdtr_iban"])?.unwrap_or_default(),
        creditor_bic: text(dict, &["creditor_bic", "cdtr_bic"])?.unwrap_or_default(),
        amount_str: text(dict, &["amount"])?.unwrap_or_else(|| "0.0".to_string()),
        currency: text(dict, &["currency"])?
            .map_or_else(|| "EUR".to_string(), |value| value.to_uppercase()),
        execution_date: text(dict, &["execution_date"])?.unwrap_or_default(),
    })
}

/// `str()` of the value under the first of `keys` present in `dict`.
fn text(dict: &Bound<'_, PyDict>, keys: &[&str]) -> PyResult<Option<String>> {
    for key in keys {
        if let Some(value) = dict.get_item(key)? {
            return Ok(Some(value.to_string()));
        }
    }
    Ok(None)
}

fn validate_record(rec: &RecordData) -> ValidatedRecord {
    let parsed_amount = rec.amount_str.trim().parse::<f64>().ok();
    let error = field_error(
        "creditor_iban",
        &rec.creditor_iban,
        validate_iban_detailed_fast,
    )
    .or_else(|| {
        field_error(
            "creditor_bic",
            &rec.creditor_bic,
            validate_bic_detailed_fast,
        )
    })
    .or_else(|| {
        parsed_amount
            .is_none()
            .then(|| ("amount", "Invalid amount format".to_string()))
    });
    ValidatedRecord {
        row: rec.row,
        error,
        parsed_amount,
        currency: rec.currency.clone(),
        fingerprint: format!(
            "{}:{}:{}:{}:{}",
            rec.creditor_iban, rec.creditor_bic, rec.amount_str, rec.currency, rec.execution_date
        ),
    }
}

/// The fault in a non-empty `value`, as `(field, message)`.
fn field_error(
    field: &'static str,
    value: &str,
    validate: fn(&str) -> (bool, String),
) -> Option<(&'static str, String)> {
    if value.is_empty() {
        return None;
    }
    let (valid, message) = validate(value);
    (!valid).then_some((field, message))
}

fn summarise<'py>(py: Python<'py>, validated: Vec<ValidatedRecord>) -> PyResult<Summary<'py>> {
    let mut summary = Summary {
        control_sums: HashMap::new(),
        errors: PyList::empty(py),
        duplicates: PyList::empty(py),
    };
    let mut seen = HashSet::new();
    for item in validated {
        if let Some((field, message)) = item.error {
            let error = row_dict(py, item.row, &[("field", field), ("error", &message)])?;
            summary.errors.append(error)?;
        }
        if let Some(amount) = item.parsed_amount {
            *summary.control_sums.entry(item.currency).or_insert(0.0) += amount;
        }
        if seen.contains(&item.fingerprint) {
            let duplicate = row_dict(py, item.row, &[("fingerprint", &item.fingerprint)])?;
            summary.duplicates.append(duplicate)?;
        } else {
            seen.insert(item.fingerprint);
        }
    }
    Ok(summary)
}

/// A dict holding `row` followed by `fields`, in that order.
fn row_dict<'py>(
    py: Python<'py>,
    row: usize,
    fields: &[(&str, &str)],
) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    dict.set_item("row", row)?;
    for (key, value) in fields {
        dict.set_item(key, value)?;
    }
    Ok(dict)
}
