// Copyright (C) 2023-2026 Pain001. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::{fail, pass};
use crate::{validate_iban, validate_iban_detailed, validate_iban_detailed_fast};

#[test]
fn valid_ibans_pass() {
    for iban in [
        "DE89370400440532013000",
        "GB82WEST12345698765432",
        "FR1420041010050500013M02606",
        "de89 3704-0044 0532 0130 00",
    ] {
        assert_eq!(validate_iban_detailed_fast(iban), pass(), "{iban}");
    }
}

#[test]
fn empty_is_refused() {
    assert_eq!(
        validate_iban_detailed_fast(""),
        fail("IBAN cannot be empty")
    );
}

#[test]
fn length_outside_15_to_34_is_refused() {
    assert_eq!(
        validate_iban_detailed_fast("DE89"),
        fail("IBAN length must be 15-34 characters (got 4)")
    );
    assert_eq!(
        validate_iban_detailed_fast(&"A".repeat(35)),
        fail("IBAN length must be 15-34 characters (got 35)")
    );
    // Separators do not count towards the length.
    assert_eq!(
        validate_iban_detailed_fast("DE89 -"),
        fail("IBAN length must be 15-34 characters (got 4)")
    );
}

#[test]
fn each_format_fault_is_named() {
    assert_eq!(
        validate_iban_detailed_fast("1289370400440532013000"),
        fail("IBAN format invalid: must start with 2-letter country code")
    );
    assert_eq!(
        validate_iban_detailed_fast("DEXX370400440532013000"),
        fail("IBAN format invalid: characters 3-4 must be check digits (00-99)")
    );
    assert_eq!(
        validate_iban_detailed_fast("DE8937040044053201300!"),
        fail("IBAN format invalid: must contain only alphanumeric characters")
    );
    assert_eq!(
        validate_iban_detailed_fast("DE89\u{e9}70400440532013000"),
        fail("IBAN format invalid: must contain only alphanumeric characters")
    );
}

#[test]
fn all_format_faults_are_reported_together() {
    assert_eq!(
        validate_iban_detailed_fast("12XX37040044053201300!"),
        fail(
            "IBAN format invalid: must start with 2-letter country code; \
             characters 3-4 must be check digits (00-99); \
             must contain only alphanumeric characters"
        )
    );
}

#[test]
fn country_length_is_enforced() {
    assert_eq!(
        validate_iban_detailed_fast("DE893704004405320130001"),
        fail("Invalid IBAN length for DE: 23 characters (expected 22)")
    );
}

#[test]
fn unknown_country_skips_the_length_table() {
    assert_eq!(
        validate_iban_detailed_fast("ZZ89370400440532013000"),
        fail("IBAN checksum validation failed (mod 97 = 68, expected 1)")
    );
}

#[test]
fn checksum_is_enforced() {
    assert_eq!(
        validate_iban_detailed_fast("DE00370400440532013000"),
        fail("IBAN checksum validation failed (mod 97 = 9, expected 1)")
    );
}

#[test]
fn python_bindings_return_the_same_verdict() {
    assert!(validate_iban("DE89370400440532013000"));
    assert!(!validate_iban("DE00370400440532013000"));
    assert_eq!(validate_iban_detailed(""), fail("IBAN cannot be empty"));
}
