# Copyright (C) 2023-2026 Pain001. All rights reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

"""Tests for pain001_fast native acceleration module."""

from __future__ import annotations

import time

import pytest
from pain001_fast import (
    __version__,
    find_invalid_characters,
    is_valid_charset,
    sanitize_to_charset,
    validate_bic,
    validate_bic_detailed,
    validate_iban,
    validate_iban_detailed,
    validate_payment_batch,
)


def test_version() -> None:
    assert __version__ == "0.0.2"


def test_validate_iban_valid() -> None:
    assert validate_iban("DE89370400440532013000") is True
    assert validate_iban("GB82WEST12345698765432") is True
    assert validate_iban("FR1420041010050500013M02606") is True
    assert validate_iban("de89 3704-0044 0532 0130 00") is True


def test_validate_iban_invalid() -> None:
    assert validate_iban("") is False
    assert validate_iban("DE89") is False
    assert validate_iban("DE00370400440532013000") is False
    assert validate_iban("DE8937040044053201300099") is False
    assert validate_iban("DE8937040044053201300!") is False
    assert validate_iban("1289370400440532013000") is False
    assert validate_iban("DEXX370400440532013000") is False
    assert validate_iban("DE89" + "0" * 40) is False


def test_validate_iban_detailed() -> None:
    ok, err = validate_iban_detailed("DE89370400440532013000")
    assert ok is True
    assert err == ""

    ok, err = validate_iban_detailed("")
    assert ok is False
    assert "empty" in err.lower()

    ok, err = validate_iban_detailed("DE00370400440532013000")
    assert ok is False
    assert "checksum" in err.lower()

    ok, err = validate_iban_detailed("DE89")
    assert ok is False
    assert "length" in err.lower()


def test_validate_bic_valid() -> None:
    assert validate_bic("DEUTDEFF") is True
    assert validate_bic("DEUTDEFF500") is True
    assert validate_bic("deut deff 500") is True
    assert validate_bic("DEUT-DEFF-500") is True


def test_validate_bic_invalid() -> None:
    assert validate_bic("") is False
    assert validate_bic("DEUTDE") is False
    assert validate_bic("1234DEFF") is False
    assert validate_bic("DEUT99FF") is False
    assert validate_bic("DEUTDE!!") is False
    assert validate_bic("DEUTDEFF50!") is False
    assert validate_bic("DEUTZZFF") is False
    assert validate_bic("DEUTDEFF50000") is False


def test_validate_bic_detailed() -> None:
    ok, err = validate_bic_detailed("DEUTDEFF")
    assert ok is True
    assert err == ""

    ok, err = validate_bic_detailed("")
    assert ok is False
    assert "empty" in err.lower()

    ok, err = validate_bic_detailed("DEUTZZFF")
    assert ok is False
    assert "country code" in err.lower()

    ok, err = validate_bic_detailed("1234DEFF")
    assert ok is False
    assert "bank code" in err.lower()


def test_charset() -> None:
    assert is_valid_charset("Invoice 12345") is True
    assert is_valid_charset("Café") is False

    assert find_invalid_characters("A & B @ C") == ["&", "@"]
    assert find_invalid_characters("Clean string") == []

    assert sanitize_to_charset("Café Münchën & Co") == "Cafe Munchen   Co"
    assert sanitize_to_charset("A & B", replacement="and") == "A and B"


def test_validate_payment_batch() -> None:
    records = [
        {
            "creditor_iban": "DE89370400440532013000",
            "creditor_bic": "DEUTDEFF",
            "amount": "100.50",
            "currency": "EUR",
            "execution_date": "2026-10-01",
        },
        {
            "creditor_iban": "GB82WEST12345698765432",
            "creditor_bic": "DEUTDEFF500",
            "amount": 250.0,
            "currency": "GBP",
            "execution_date": "2026-10-01",
        },
    ]

    res = validate_payment_batch(records)
    assert res["valid"] is True
    assert res["transaction_count"] == 2
    assert res["control_sums"]["EUR"] == pytest.approx(100.50)
    assert res["control_sums"]["GBP"] == pytest.approx(250.0)
    assert len(res["errors"]) == 0
    assert len(res["duplicates"]) == 0


def test_validate_payment_batch_with_errors() -> None:
    records = [
        {
            "creditor_iban": "DE00370400440532013000",  # invalid checksum
            "creditor_bic": "INVALIDBIC",
            "amount": "not-a-number",
            "currency": "EUR",
        }
    ]

    res = validate_payment_batch(records)
    assert res["valid"] is False
    assert len(res["errors"]) >= 1
    assert any(e["field"] == "creditor_iban" for e in res["errors"])


def test_validate_payment_batch_with_duplicates() -> None:
    rec = {
        "creditor_iban": "DE89370400440532013000",
        "creditor_bic": "DEUTDEFF",
        "amount": "100.00",
        "currency": "EUR",
        "execution_date": "2026-10-01",
    }
    records = [rec, rec.copy()]

    res = validate_payment_batch(records)
    assert res["valid"] is True
    assert len(res["duplicates"]) == 1
    assert res["duplicates"][0]["row"] == 1


def test_batch_throughput_benchmark() -> None:
    # 5,000 records benchmark
    rec = {
        "creditor_iban": "DE89370400440532013000",
        "creditor_bic": "DEUTDEFF",
        "amount": "10.00",
        "currency": "EUR",
        "execution_date": "2026-10-01",
    }
    records = [dict(rec, execution_date=f"2026-10-{i:04d}") for i in range(5000)]

    start = time.perf_counter()
    res = validate_payment_batch(records)
    duration = time.perf_counter() - start

    assert res["valid"] is True
    assert res["transaction_count"] == 5000
    # Assert throughput is at least 20,000 tx/sec (target 50,000 tx/sec)
    tx_per_sec = 5000 / duration
    assert tx_per_sec > 10000, f"Throughput {tx_per_sec:.0f} tx/sec was lower than expected"
