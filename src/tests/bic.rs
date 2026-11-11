// Copyright (C) 2023-2026 Pain001. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::{fail, pass};
use crate::{validate_bic, validate_bic_detailed, validate_bic_detailed_fast};

#[test]
fn valid_bics_pass() {
    for bic in ["DEUTDEFF", "DEUTDEFF500", "deutdeff", "DEUT-DEFF 500"] {
        assert_eq!(validate_bic_detailed_fast(bic), pass(), "{bic}");
    }
}

#[test]
fn empty_is_refused() {
    assert_eq!(validate_bic_detailed_fast(""), fail("BIC cannot be empty"));
}

#[test]
fn length_other_than_8_or_11_is_refused() {
    assert_eq!(
        validate_bic_detailed_fast("DEUTDE"),
        fail("BIC must be 8 or 11 characters (got 6: 'DEUTDE')")
    );
    assert_eq!(
        validate_bic_detailed_fast("DEUTDEF"),
        fail("BIC must be 8 or 11 characters (got 7: 'DEUTDEF')")
    );
    // Over 11 stops at the twelfth character and quotes the input.
    assert_eq!(
        validate_bic_detailed_fast("DEUTDEFF500X"),
        fail("BIC must be 8 or 11 characters (got 12): 'DEUTDEFF500X'")
    );
    // Length is counted in bytes: a two-byte letter makes nine.
    assert_eq!(
        validate_bic_detailed_fast("DEU\u{e9}DEFF"),
        fail("BIC must be 8 or 11 characters (got 9: 'DEU\u{e9}DEFF')")
    );
}

#[test]
fn each_part_fault_is_named() {
    assert_eq!(
        validate_bic_detailed_fast("1234DEFF"),
        fail("BIC format invalid: bank code (first 4 chars) must be letters (got '1234')")
    );
    assert_eq!(
        validate_bic_detailed_fast("DEUT99FF"),
        fail("BIC format invalid: country code (chars 5-6) must be letters (got '99')")
    );
    assert_eq!(
        validate_bic_detailed_fast("DEUTDE!!"),
        fail("BIC format invalid: location code (chars 7-8) must be alphanumeric (got '!!')")
    );
    assert_eq!(
        validate_bic_detailed_fast("DEUTDEFF50!"),
        fail("BIC format invalid: branch code (chars 9-11) must be alphanumeric (got '50!')")
    );
}

#[test]
fn all_part_faults_are_reported_together() {
    assert_eq!(
        validate_bic_detailed_fast("12349!!!!!!"),
        fail(
            "BIC format invalid: bank code (first 4 chars) must be letters (got '1234'); \
             country code (chars 5-6) must be letters (got '9!'); \
             location code (chars 7-8) must be alphanumeric (got '!!'); \
             branch code (chars 9-11) must be alphanumeric (got '!!!')"
        )
    );
}

#[test]
fn a_part_that_splits_a_character_is_quoted_empty() {
    // Eleven bytes; the bank and country slices each cut the two-byte
    // letter in half, so neither is valid UTF-8 on its own.
    assert_eq!(
        validate_bic_detailed_fast("DEU\u{e9}DEFF50"),
        fail(
            "BIC format invalid: bank code (first 4 chars) must be letters (got ''); \
             country code (chars 5-6) must be letters (got '')"
        )
    );
}

#[test]
fn unlisted_country_is_refused() {
    assert_eq!(
        validate_bic_detailed_fast("DEUTZZFF"),
        fail("BIC country code 'ZZ' is not a valid ISO 3166-1 alpha-2 code")
    );
}

#[test]
fn python_bindings_return_the_same_verdict() {
    assert!(validate_bic("DEUTDEFF"));
    assert!(!validate_bic("DEUTZZFF"));
    assert_eq!(validate_bic_detailed(""), fail("BIC cannot be empty"));
}

#[test]
fn the_country_list_is_sorted_for_binary_search() {
    let list = crate::bic::BIC_COUNTRIES;
    assert!(list.windows(2).all(|pair| pair[0] < pair[1]));
    for country in list {
        let bic = format!("BANK{}2L", std::str::from_utf8(&country).unwrap());
        assert_eq!(validate_bic_detailed_fast(&bic), pass(), "{bic}");
    }
}
