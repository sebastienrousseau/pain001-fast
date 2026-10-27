# Copyright (C) 2023-2026 Pain001. All rights reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

"""pain001-fast: High-throughput native acceleration core for ISO 20022.

This package provides native Rust bindings for high-performance financial
validation, achieving > 50,000 tx/sec throughput for IBAN, BIC, restricted
character set validation, and bulk payment batch analysis.
"""

from __future__ import annotations

from typing import Any

__version__ = "0.0.2"

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
except ImportError:  # pragma: no cover
    # Pure Python fallback when compiled extension is unavailable
    import unicodedata

    _ALLOWED_CHARS = frozenset(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/-?:().,'+ "
    )

    _IBAN_LENGTHS: dict[str, int] = {
        "AD": 24, "AE": 23, "AL": 28, "AT": 20, "AZ": 28, "BA": 20, "BE": 16,
        "BG": 22, "BH": 22, "BR": 29, "BY": 28, "CH": 21, "CR": 22, "CY": 28,
        "CZ": 24, "DE": 22, "DK": 18, "DO": 28, "EE": 20, "EG": 29, "ES": 24,
        "FI": 18, "FO": 18, "FR": 27, "GB": 22, "GE": 22, "GI": 23, "GL": 18,
        "GR": 27, "GT": 28, "HR": 21, "HU": 28, "IE": 22, "IL": 23, "IQ": 23,
        "IS": 26, "IT": 27, "JO": 30, "KW": 30, "KZ": 20, "LB": 28, "LC": 32,
        "LI": 21, "LT": 20, "LU": 20, "LV": 21, "MC": 27, "MD": 24, "ME": 22,
        "MK": 19, "MR": 27, "MT": 31, "MU": 30, "NL": 18, "NO": 15, "PK": 24,
        "PL": 28, "PS": 29, "PT": 25, "QA": 29, "RO": 24, "RS": 22, "SA": 24,
        "SE": 24, "SI": 19, "SK": 24, "SM": 27, "TN": 24, "TR": 26, "UA": 29,
        "VA": 22, "VG": 24, "XK": 20,
    }

    _VALID_BIC_COUNTRIES = frozenset([
        "AD", "AE", "AL", "AR", "AT", "AU", "AZ", "BA", "BE", "BG", "BH", "BR",
        "BY", "CA", "CH", "CL", "CN", "CO", "CR", "CY", "CZ", "DE", "DK", "DO",
        "EE", "EG", "ES", "FI", "FO", "FR", "GB", "GE", "GI", "GL", "GR", "GT",
        "HK", "HR", "HU", "ID", "IE", "IL", "IN", "IQ", "IS", "IT", "JO", "JP",
        "KR", "KW", "KZ", "LB", "LC", "LI", "LT", "LU", "LV", "MC", "MD", "ME",
        "MK", "MR", "MT", "MU", "MX", "MY", "NL", "NO", "NZ", "PH", "PK", "PL",
        "PS", "PT", "QA", "RO", "RS", "RU", "SA", "SE", "SG", "SI", "SK", "SM",
        "TH", "TN", "TR", "TW", "UA", "US", "VA", "VG", "XK", "ZA",
    ])

    def validate_iban_detailed(iban: str) -> tuple[bool, str]:
        if not iban:
            return False, "IBAN cannot be empty"
        clean = iban.replace(" ", "").replace("-", "").upper()
        if not 15 <= len(clean) <= 34:
            return False, f"IBAN length must be 15-34 characters (got {len(clean)})"
        country = clean[:2]
        if not country.isalpha():
            return False, "IBAN format invalid: must start with 2-letter country code"
        if not clean[2:4].isdigit():
            return False, "IBAN format invalid: characters 3-4 must be check digits (00-99)"
        if not clean[4:].isalnum():
            return False, "IBAN format invalid: must contain only alphanumeric characters"
        if country in _IBAN_LENGTHS and len(clean) != _IBAN_LENGTHS[country]:
            return False, f"Invalid IBAN length for {country}: {len(clean)} characters (expected {_IBAN_LENGTHS[country]})"
        rearranged = clean[4:] + clean[:4]
        numeric = ""
        for char in rearranged:
            if char.isdigit():
                numeric += char
            elif char.isalpha():
                numeric += str(ord(char) - ord("A") + 10)
            else:
                return False, f"Invalid character in IBAN: {char}"
        if int(numeric) % 97 != 1:
            return False, f"IBAN checksum validation failed (mod 97 = {int(numeric) % 97}, expected 1)"
        return True, ""

    def validate_iban(iban: str) -> bool:
        valid, _ = validate_iban_detailed(iban)
        return valid

    def validate_bic_detailed(bic: str) -> tuple[bool, str]:
        if not bic:
            return False, "BIC cannot be empty"
        clean = bic.replace(" ", "").replace("-", "").upper()
        if len(clean) not in (8, 11):
            return False, f"BIC must be 8 or 11 characters (got {len(clean)}: '{clean}')"
        bank = clean[:4]
        country = clean[4:6]
        loc = clean[6:8]
        branch = clean[8:11] if len(clean) == 11 else ""
        if not bank.isalpha():
            return False, f"BIC format invalid: bank code (first 4 chars) must be letters (got '{bank}')"
        if not country.isalpha():
            return False, f"BIC format invalid: country code (chars 5-6) must be letters (got '{country}')"
        if not loc.isalnum():
            return False, f"BIC format invalid: location code (chars 7-8) must be alphanumeric (got '{loc}')"
        if branch and not branch.isalnum():
            return False, f"BIC format invalid: branch code (chars 9-11) must be alphanumeric (got '{branch}')"
        if country not in _VALID_BIC_COUNTRIES:
            return False, f"BIC country code '{country}' is not a valid ISO 3166-1 alpha-2 code"
        return True, ""

    def validate_bic(bic: str) -> bool:
        valid, _ = validate_bic_detailed(bic)
        return valid

    def find_invalid_characters(value: str) -> list[str]:
        return sorted({ch for ch in value if ch not in _ALLOWED_CHARS})

    def is_valid_charset(value: str) -> bool:
        return not find_invalid_characters(value)

    def sanitize_to_charset(value: str, replacement: str = " ") -> str:
        decomposed = unicodedata.normalize("NFKD", value)
        stripped = "".join(ch for ch in decomposed if not unicodedata.combining(ch))
        return "".join(ch if ch in _ALLOWED_CHARS else replacement for ch in stripped)

    def validate_payment_batch(
        records: list[dict[str, Any]],
        message_type: str = "pain.001.001.03",
    ) -> dict[str, Any]:
        control_sums: dict[str, float] = {}
        errors: list[dict[str, Any]] = []
        seen: set[str] = set()
        duplicates: list[dict[str, Any]] = []
        for idx, rec in enumerate(records):
            iban = rec.get("creditor_iban", "") or rec.get("cdtr_iban", "")
            if iban:
                ok, err = validate_iban_detailed(iban)
                if not ok:
                    errors.append({"row": idx, "field": "creditor_iban", "error": err})
            bic = rec.get("creditor_bic", "") or rec.get("cdtr_bic", "")
            if bic:
                ok, err = validate_bic_detailed(bic)
                if not ok:
                    errors.append({"row": idx, "field": "creditor_bic", "error": err})
            curr = str(rec.get("currency", "EUR")).upper()
            try:
                amt = float(rec.get("amount", 0.0))
                control_sums[curr] = control_sums.get(curr, 0.0) + amt
            except (ValueError, TypeError):
                errors.append({"row": idx, "field": "amount", "error": "Invalid amount format"})
                amt = 0.0
            fingerprint = f"{iban}:{bic}:{amt}:{curr}:{rec.get('execution_date', '')}"
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
