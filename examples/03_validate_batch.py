# Copyright (C) 2023-2026 Pain001. All rights reserved.
# SPDX-License-Identifier: Apache-2.0 OR MIT

"""Validate a batch of payment records in parallel.

The result carries per-currency control sums, the first fault of each
record, and records repeated in full. Run with
``python examples/03_validate_batch.py``.
"""

from pain001_fast import validate_payment_batch

payment = {
    "creditor_iban": "DE89370400440532013000",
    "creditor_bic": "DEUTDEFF",
    "amount": "1250.50",
    "currency": "EUR",
    "execution_date": "2026-10-01",
}
records = [
    payment,
    dict(payment),  # the same payment again
    dict(payment, creditor_iban="DE00370400440532013000", amount="10"),
    dict(payment, amount="12,50", currency="gbp"),
]

result = validate_payment_batch(records)
for error in result["errors"]:
    print(f"row {error['row']} {error['field']}: {error['error']}")

assert result["valid"] is False
assert result["transaction_count"] == 4
assert [(e["row"], e["field"]) for e in result["errors"]] == [
    (2, "creditor_iban"),
    (3, "amount"),
]
assert [d["row"] for d in result["duplicates"]] == [1]
# A record whose IBAN fails still counts its amount; one whose amount
# does not parse adds nothing.
assert result["control_sums"] == {"EUR": 2511.0}
