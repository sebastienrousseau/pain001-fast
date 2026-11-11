<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# pain001-fast Governance

This document describes how pain001-fast is run, how decisions are
made, and how to take on responsibility for it. It exists to make the
project legible and sustainable, and to reduce its dependence on any
single person.

## Mission and scope

pain001-fast is an optional native extension for the
[`pain001`](https://github.com/sebastienrousseau/pain001) ISO 20022
library. It implements IBAN, BIC and restricted-character-set checks
and a parallel batch validator in Rust, exposed to Python through PyO3;
pain001's own validators use it when it is installed.

Changes are weighed against that scope: correctness, security and
clarity over feature breadth. A change to what a validator accepts or
to its messages is a change to pain001's behaviour, so it is agreed in
the core first.

## Roles

| Role | Who | Can |
| :--- | :--- | :--- |
| **Maintainer** | Listed in [`MAINTAINERS.md`](MAINTAINERS.md) | Merge PRs, cut releases, triage, set direction |
| **Contributor** | Anyone with a merged PR | Propose changes, review, discuss |
| **User** | Everyone | File issues, ask questions, request features |

## Decision making

- **Day-to-day changes** (fixes, docs, tests, performance work that
  keeps every verdict and message) proceed by **lazy consensus**: open
  a PR; if no maintainer objects and CI is green, a maintainer merges
  it.
- **Significant changes** (a new binding, a change to what a validator
  accepts or the text of a message, new dependencies, a raised Rust or
  Python floor) need explicit approval from a maintainer in the PR,
  and should start as an issue.
- **Disagreement** is resolved by discussion aiming for consensus; if
  none is reached, the lead maintainer decides and records the
  rationale.

Every change must pass the full gate (100% Rust line and function
coverage, 100% Python line and branch coverage, clippy and ruff with
the complexity ceilings, rustdoc with warnings denied, mypy) before
merge, enforced in CI, not by trust. Commits carry a Developer
Certificate of Origin sign-off (`git commit -s`).

## Releases

Every member of the pain001 suite ships the same version number as the
core; this extension joined at 0.0.82. Tags are signed, and the
tag-triggered release workflow attaches the wheels and source
distribution to the GitHub release. [DEVELOPMENT.md](DEVELOPMENT.md#release-model)
describes the model. Only maintainers cut releases.

## Becoming a maintainer

More maintainers is the single biggest thing that would de-risk the
project.

1. Contribute a few reviewed PRs ([`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
   is the map; good first areas: a benchmark in the repository, a fuzz
   finding, a platform the wheels do not cover yet).
2. Help triage issues and review others' PRs.
3. Open an issue or email the lead maintainer expressing interest.

A maintainer proposes you; with no objection from existing maintainers
within a week, you are added to `MAINTAINERS.md`.

## Sustainability (bus factor)

pain001-fast has **one** maintainer today. The mitigations in place:

- **The work is legible:** [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
  maps the code and [DEVELOPMENT.md](DEVELOPMENT.md) reproduces every
  gate; the examples are executed by the test suite.
- **Quality is enforced by CI,** not by one person's memory.
- **pain001 does not depend on it:** without the extension, pain001
  uses its own pure-Python validators.
- **The goal is two or more maintainers** with independent release
  authority.

The continuity plan for the suite's shared assets (signing keys,
repositories, the `pain001.com` domain) is in the core's
[GOVERNANCE.md](https://github.com/sebastienrousseau/pain001/blob/main/GOVERNANCE.md#continuity-plan).

## Code of conduct and security

Participation is governed by [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).
Security issues follow the private disclosure process in
[`SECURITY.md`](SECURITY.md); please do not open public issues for
vulnerabilities.
