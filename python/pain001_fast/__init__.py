# Copyright (C) 2023-2026 Pain001. All rights reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

"""pain001-fast: High-throughput native acceleration core for ISO 20022.

This package provides native Rust bindings for high-performance financial
validation, achieving > 50,000 tx/sec throughput for IBAN, BIC, restricted
character set validation, and bulk payment batch analysis.
"""

from __future__ import annotations

from typing import Any

__version__ = "0.0.85"

try:
    from pain001_fast._pain001_fast import (
        find_invalid_characters,
        is_valid_charset,
        sanitize_to_charset,
        validate_bic,
        validate_bic_detailed,
        validate_iban,
        validate_iban_detailed,
        validate_payment_batch,
    )
except ImportError:
    # Pure Python fallback when compiled extension is unavailable
    import unicodedata

    _ALLOWED_CHARS = frozenset(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/-?:().,'+ "
    )

    _IBAN_LENGTHS: dict[str, int] = {
        "AD": 24,
        "AE": 23,
        "AL": 28,
        "AT": 20,
        "AZ": 28,
        "BA": 20,
        "BE": 16,
        "BG": 22,
        "BH": 22,
        "BR": 29,
        "BY": 28,
        "CH": 21,
        "CR": 22,
        "CY": 28,
        "CZ": 24,
        "DE": 22,
        "DK": 18,
        "DO": 28,
        "EE": 20,
        "EG": 29,
        "ES": 24,
        "FI": 18,
        "FO": 18,
        "FR": 27,
        "GB": 22,
        "GE": 22,
        "GI": 23,
        "GL": 18,
        "GR": 27,
        "GT": 28,
        "HR": 21,
        "HU": 28,
        "IE": 22,
        "IL": 23,
        "IQ": 23,
        "IS": 26,
        "IT": 27,
        "JO": 30,
        "KW": 30,
        "KZ": 20,
        "LB": 28,
        "LC": 32,
        "LI": 21,
        "LT": 20,
        "LU": 20,
        "LV": 21,
        "MC": 27,
        "MD": 24,
        "ME": 22,
        "MK": 19,
        "MR": 27,
        "MT": 31,
        "MU": 30,
        "NL": 18,
        "NO": 15,
        "PK": 24,
        "PL": 28,
        "PS": 29,
        "PT": 25,
        "QA": 29,
        "RO": 24,
        "RS": 22,
        "SA": 24,
        "SE": 24,
        "SI": 19,
        "SK": 24,
        "SM": 27,
        "TN": 24,
        "TR": 26,
        "UA": 29,
        "VA": 22,
        "VG": 24,
        "XK": 20,
    }

    _VALID_BIC_COUNTRIES = frozenset(
        [
            "AD",
            "AE",
            "AL",
            "AR",
            "AT",
            "AU",
            "AZ",
            "BA",
            "BE",
            "BG",
            "BH",
            "BR",
            "BY",
            "CA",
            "CH",
            "CL",
            "CN",
            "CO",
            "CR",
            "CY",
            "CZ",
            "DE",
            "DK",
            "DO",
            "EE",
            "EG",
            "ES",
            "FI",
            "FO",
            "FR",
            "GB",
            "GE",
            "GI",
            "GL",
            "GR",
            "GT",
            "HK",
            "HR",
            "HU",
            "ID",
            "IE",
            "IL",
            "IN",
            "IQ",
            "IS",
            "IT",
            "JO",
            "JP",
            "KR",
            "KW",
            "KZ",
            "LB",
            "LC",
            "LI",
            "LT",
            "LU",
            "LV",
            "MC",
            "MD",
            "ME",
            "MK",
            "MR",
            "MT",
            "MU",
            "MX",
            "MY",
            "NL",
            "NO",
            "NZ",
            "PH",
            "PK",
            "PL",
            "PS",
            "PT",
            "QA",
            "RO",
            "RS",
            "RU",
            "SA",
            "SE",
            "SG",
            "SI",
            "SK",
            "SM",
            "TH",
            "TN",
            "TR",
            "TW",
            "UA",
            "US",
            "VA",
            "VG",
            "XK",
            "ZA",
        ]
    )

    # The extension works on ASCII bytes. str.isalpha(), isdigit(),
    # isalnum() and upper() accept or produce non-ASCII characters
    # (an accented letter, a superscript digit, a ligature that
    # uppercases to two ASCII letters), so each check here also
    # requires ASCII, and only ASCII letters are uppercased.
    _ASCII_UPPER = str.maketrans(
        "abcdefghijklmnopqrstuvwxyz", "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    )

    def _clean(value: str) -> str:
        return value.replace(" ", "").replace("-", "").translate(_ASCII_UPPER)

    def _ascii_alpha(text: str) -> bool:
        return text.isascii() and text.isalpha()

    def _ascii_digits(text: str) -> bool:
        return text.isascii() and text.isdigit()

    def _ascii_alnum(text: str) -> bool:
        return text.isascii() and text.isalnum()

    def _iban_format_error(clean: str) -> str:
        if not _ascii_alpha(clean[:2]):
            return "IBAN format invalid: must start with 2-letter country code"
        if not _ascii_digits(clean[2:4]):
            return "IBAN format invalid: characters 3-4 must be check digits (00-99)"
        if not _ascii_alnum(clean[4:]):
            return "IBAN format invalid: must contain only alphanumeric characters"
        expected = _IBAN_LENGTHS.get(clean[:2])
        if expected is not None and len(clean) != expected:
            return f"Invalid IBAN length for {clean[:2]}: {len(clean)} characters (expected {expected})"
        return ""

    def validate_iban_detailed(iban: str) -> tuple[bool, str]:
        """Validate an IBAN, returning ``(True, "")`` or ``(False, reason)``."""
        if not iban:
            return False, "IBAN cannot be empty"
        clean = _clean(iban)
        if not 15 <= len(clean) <= 34:
            return (
                False,
                f"IBAN length must be 15-34 characters (got {len(clean)})",
            )
        error = _iban_format_error(clean)
        if error:
            return False, error
        # Every character is now an ASCII letter or digit; base 36
        # maps 0-9 to 0-9 and A-Z to 10-35, as ISO 13616 does.
        rearranged = clean[4:] + clean[:4]
        remainder = int("".join(str(int(ch, 36)) for ch in rearranged)) % 97
        if remainder != 1:
            return (
                False,
                f"IBAN checksum validation failed (mod 97 = {remainder}, expected 1)",
            )
        return True, ""

    def validate_iban(iban: str) -> bool:
        """Whether ``iban`` is a valid IBAN."""
        valid, _ = validate_iban_detailed(iban)
        return valid

    def validate_bic_detailed(bic: str) -> tuple[bool, str]:
        """Validate a BIC, returning ``(True, "")`` or ``(False, reason)``."""
        if not bic:
            return False, "BIC cannot be empty"
        clean = _clean(bic)
        if len(clean) not in (8, 11):
            return (
                False,
                f"BIC must be 8 or 11 characters (got {len(clean)}: '{clean}')",
            )
        bank = clean[:4]
        country = clean[4:6]
        loc = clean[6:8]
        branch = clean[8:11] if len(clean) == 11 else ""
        if not _ascii_alpha(bank):
            return (
                False,
                f"BIC format invalid: bank code (first 4 chars) must be letters (got '{bank}')",
            )
        if not _ascii_alpha(country):
            return (
                False,
                f"BIC format invalid: country code (chars 5-6) must be letters (got '{country}')",
            )
        if not _ascii_alnum(loc):
            return (
                False,
                f"BIC format invalid: location code (chars 7-8) must be alphanumeric (got '{loc}')",
            )
        if branch and not _ascii_alnum(branch):
            return (
                False,
                f"BIC format invalid: branch code (chars 9-11) must be alphanumeric (got '{branch}')",
            )
        if country not in _VALID_BIC_COUNTRIES:
            return (
                False,
                f"BIC country code '{country}' is not a valid ISO 3166-1 alpha-2 code",
            )
        return True, ""

    def validate_bic(bic: str) -> bool:
        """Whether ``bic`` is a valid BIC."""
        valid, _ = validate_bic_detailed(bic)
        return valid

    def find_invalid_characters(value: str) -> list[str]:
        """The distinct characters outside the restricted set, sorted."""
        return sorted({ch for ch in value if ch not in _ALLOWED_CHARS})

    def is_valid_charset(value: str) -> bool:
        """Whether every character is in the restricted Latin set."""
        return not find_invalid_characters(value)

    def sanitize_to_charset(value: str, replacement: str = " ") -> str:
        """Strip accents and replace other refused characters."""
        decomposed = unicodedata.normalize("NFKD", value)
        stripped = "".join(
            ch for ch in decomposed if not unicodedata.combining(ch)
        )
        return "".join(
            ch if ch in _ALLOWED_CHARS else replacement for ch in stripped
        )

    def validate_payment_batch(
        records: list[dict[str, Any]],
        message_type: str = "pain.001.001.03",
    ) -> dict[str, Any]:
        """Validate payment records; see the extension for the fields."""
        control_sums: dict[str, float] = {}
        errors: list[dict[str, Any]] = []
        seen: set[str] = set()
        duplicates: list[dict[str, Any]] = []
        for idx, rec in enumerate(records):
            iban = rec.get("creditor_iban", "") or rec.get("cdtr_iban", "")
            if iban:
                ok, err = validate_iban_detailed(iban)
                if not ok:
                    errors.append(
                        {"row": idx, "field": "creditor_iban", "error": err}
                    )
            bic = rec.get("creditor_bic", "") or rec.get("cdtr_bic", "")
            if bic:
                ok, err = validate_bic_detailed(bic)
                if not ok:
                    errors.append(
                        {"row": idx, "field": "creditor_bic", "error": err}
                    )
            curr = str(rec.get("currency", "EUR")).upper()
            try:
                amt = float(rec.get("amount", 0.0))
                control_sums[curr] = control_sums.get(curr, 0.0) + amt
            except (ValueError, TypeError):
                errors.append(
                    {
                        "row": idx,
                        "field": "amount",
                        "error": "Invalid amount format",
                    }
                )
                amt = 0.0
            fingerprint = (
                f"{iban}:{bic}:{amt}:{curr}:{rec.get('execution_date', '')}"
            )
            if fingerprint in seen:
                duplicates.append({"row": idx, "fingerprint": fingerprint})
            else:
                seen.add(fingerprint)
        return {
            "valid": len(errors) == 0,
            "transaction_count": len(records),
            "control_sums": control_sums,
            "errors": errors,
            "duplicates": duplicates,
            "duration_micros": 0,
        }


__all__ = [
    "__version__",
    "find_invalid_characters",
    "is_valid_charset",
    "sanitize_to_charset",
    "validate_bic",
    "validate_bic_detailed",
    "validate_iban",
    "validate_iban_detailed",
    "validate_payment_batch",
]
