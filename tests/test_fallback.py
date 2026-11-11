# Copyright (C) 2023-2026 Pain001. All rights reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

"""Tests for the pure-Python fallback in ``pain001_fast/__init__.py``.

The package defines Python versions of every binding for an install
whose compiled extension cannot be imported. pain001 cannot tell the
two apart (it imports ``pain001_fast`` and calls what it finds), so the
fallback must give the same verdicts. These tests load the package a
second time, under another name, with the extension blocked.
"""

from __future__ import annotations

import importlib.util
import sys
from pathlib import Path
from types import ModuleType
from typing import Any

import pain001_fast
import pytest

INIT = Path(pain001_fast.__file__)


@pytest.fixture(scope="module")
def fallback() -> Any:
    """Load the package's ``__init__`` with the extension unimportable."""
    blocked = "pain001_fast._pain001_fast"
    saved = sys.modules.get(blocked)
    sys.modules[blocked] = None  # type: ignore[assignment]
    try:
        spec = importlib.util.spec_from_file_location("_fallback", INIT)
        assert spec is not None and spec.loader is not None
        module: ModuleType = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
    finally:
        if saved is None:
            del sys.modules[blocked]
        else:
            sys.modules[blocked] = saved
    assert module.validate_iban is not pain001_fast.validate_iban
    return module


def test_iban_valid(fallback: Any) -> None:
    for iban in (
        "DE89370400440532013000",
        "GB82WEST12345698765432",
        "FR1420041010050500013M02606",
        "de89 3704-0044 0532 0130 00",
    ):
        assert fallback.validate_iban_detailed(iban) == (True, ""), iban
        assert fallback.validate_iban(iban) is True


@pytest.mark.parametrize(
    ("iban", "message"),
    [
        ("", "IBAN cannot be empty"),
        ("DE89", "IBAN length must be 15-34 characters (got 4)"),
        (
            "1289370400440532013000",
            "IBAN format invalid: must start with 2-letter country code",
        ),
        (
            "DEXX370400440532013000",
            (
                "IBAN format invalid: characters 3-4 must be check digits"
                " (00-99)"
            ),
        ),
        (
            "DE8937040044053201300!",
            "IBAN format invalid: must contain only alphanumeric characters",
        ),
        (
            "DE893704004405320130001",
            "Invalid IBAN length for DE: 23 characters (expected 22)",
        ),
        (
            "DE00370400440532013000",
            "IBAN checksum validation failed (mod 97 = 9, expected 1)",
        ),
        (
            "ZZ89370400440532013000",
            "IBAN checksum validation failed (mod 97 = 68, expected 1)",
        ),
    ],
)
def test_iban_refusals(fallback: Any, iban: str, message: str) -> None:
    assert fallback.validate_iban_detailed(iban) == (False, message)
    assert pain001_fast.validate_iban_detailed(iban) == (False, message)
    assert fallback.validate_iban(iban) is False


def test_bic_valid(fallback: Any) -> None:
    for bic in ("DEUTDEFF", "DEUTDEFF500", "deut-deff 500"):
        assert fallback.validate_bic_detailed(bic) == (True, ""), bic
        assert fallback.validate_bic(bic) is True


@pytest.mark.parametrize(
    ("bic", "message"),
    [
        ("", "BIC cannot be empty"),
        ("DEUTDE", "BIC must be 8 or 11 characters (got 6: 'DEUTDE')"),
        (
            "1234DEFF",
            (
                "BIC format invalid: bank code (first 4 chars) must be letters"
                " (got '1234')"
            ),
        ),
        (
            "DEUT99FF",
            (
                "BIC format invalid: country code (chars 5-6) must be letters"
                " (got '99')"
            ),
        ),
        (
            "DEUTDE!!",
            (
                "BIC format invalid: location code (chars 7-8) must be"
                " alphanumeric (got '!!')"
            ),
        ),
        (
            "DEUTDEFF50!",
            (
                "BIC format invalid: branch code (chars 9-11) must be"
                " alphanumeric (got '50!')"
            ),
        ),
        (
            "DEUTZZFF",
            "BIC country code 'ZZ' is not a valid ISO 3166-1 alpha-2 code",
        ),
    ],
)
def test_bic_refusals(fallback: Any, bic: str, message: str) -> None:
    assert fallback.validate_bic_detailed(bic) == (False, message)
    assert pain001_fast.validate_bic_detailed(bic) == (False, message)
    assert fallback.validate_bic(bic) is False


def test_charset(fallback: Any) -> None:
    assert fallback.is_valid_charset("Invoice 12345") is True
    assert fallback.is_valid_charset("Café") is False
    assert fallback.find_invalid_characters("A & B @ C &") == ["&", "@"]
    assert fallback.sanitize_to_charset("Café Münchën & Co") == (
        "Cafe Munchen   Co"
    )
    assert fallback.sanitize_to_charset("A & B", "and") == "A and B"


def test_batch_clean(fallback: Any) -> None:
    result = fallback.validate_payment_batch(
        [
            {
                "creditor_iban": "DE89370400440532013000",
                "creditor_bic": "DEUTDEFF",
                "amount": "100.50",
                "currency": "eur",
            },
            {
                "cdtr_iban": "GB82WEST12345698765432",
                "cdtr_bic": "DEUTDEFF500",
                "amount": 250.0,
                "currency": "GBP",
            },
            {},
        ]
    )
    assert result == {
        "valid": True,
        "transaction_count": 3,
        "control_sums": {"EUR": 100.5, "GBP": 250.0},
        "errors": [],
        "duplicates": [],
        "duration_micros": 0,
    }


def test_batch_faults_and_duplicates(fallback: Any) -> None:
    bad = {
        "creditor_iban": "DE00370400440532013000",
        "creditor_bic": "BAD",
        "amount": "x",
    }
    result = fallback.validate_payment_batch([bad, dict(bad)])
    assert result["valid"] is False
    assert result["control_sums"] == {}
    assert [(e["row"], e["field"]) for e in result["errors"]] == [
        (0, "creditor_iban"),
        (0, "creditor_bic"),
        (0, "amount"),
        (1, "creditor_iban"),
        (1, "creditor_bic"),
        (1, "amount"),
    ]
    assert result["duplicates"] == [
        {
            "row": 1,
            "fingerprint": "DE00370400440532013000:BAD:0.0:EUR:",
        }
    ]


@pytest.mark.parametrize(
    "iban",
    [
        # Superscript two passes str.isdigit(), so the fallback fed it
        # to int() and raised ValueError instead of refusing the IBAN.
        "DE89²70400440532013000",
        # One half passes str.isalnum() and nothing else, which reached
        # a branch the extension does not have.
        "DE89½70400440532013000",
        # An accented letter passes str.isalpha(); the extension refuses
        # anything outside ASCII.
        "DE89É70400440532013000",
        # A non-ASCII digit in the check-digit position.
        "DE٨937040044053201300",
    ],
)
def test_iban_outside_ascii_is_refused_like_the_extension(
    fallback: Any, iban: str
) -> None:
    expected = pain001_fast.validate_iban_detailed(iban)
    assert expected[0] is False
    assert fallback.validate_iban_detailed(iban) == expected


@pytest.mark.parametrize(
    "bic",
    [
        # Accented letters in the bank or country code passed
        # str.isalpha(), so the fallback accepted a BIC the extension
        # refuses.
        "DÉUTDEFF",
        "DEUTDÉFF",
        "DEUTDEFÉ",
        # str.upper() turns the ligature into two ASCII letters, so the
        # seven-character input became the valid BIC DEUTDEFF.
        "DEUTDEﬀ",
    ],
)
def test_bic_outside_ascii_is_refused(fallback: Any, bic: str) -> None:
    assert pain001_fast.validate_bic(bic) is False
    assert fallback.validate_bic_detailed(bic)[0] is False
