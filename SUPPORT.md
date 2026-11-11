<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Getting support

Thanks for using `pain001-fast`. Here is the fastest way to get help,
by need.

## Read first

- **[README.md](README.md)**: install, quick start and what each
  binding does.
- **[`examples/`](examples/)**: three runnable, self-checking scripts,
  exercised in CI.
- **[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)**: how the Rust core,
  the PyO3 bindings and the Python fallback fit together.

## Questions and how-to

Open a [GitHub issue](https://github.com/sebastienrousseau/pain001-fast/issues/new/choose)
(the pain001 suite does not use GitHub Discussions) with:

- Python version, OS and CPU architecture
- `pain001-fast` version
  (`python -c "import pain001_fast; print(pain001_fast.__version__)"`)
  and, if relevant, the `pain001` version
- Whether you installed a release wheel or built from source
- A minimal reproducer and the full error output

## Bugs

Open a [bug report](https://github.com/sebastienrousseau/pain001-fast/issues/new?template=bug_report.yml)
with the data above and the expected and actual result. Never include
a real account number; the
[examples](examples/) use public test IBANs.

A verdict that differs between `pain001-fast` and pain001 without it
is a bug in one of them; say which inputs differ.

## Feature requests

Open a [feature request](https://github.com/sebastienrousseau/pain001-fast/issues/new?template=feature_request.yml).
A change to what a validator accepts is decided in the core first,
since pain001's results must not depend on whether this extension is
installed.

## Security

**Do not** open public issues for vulnerabilities. Follow the private
disclosure process in [SECURITY.md](SECURITY.md).

## Support

This package is free open-source software under Apache-2.0 OR MIT,
maintained in a personal capacity. There is no paid tier, no SLA and
no out-of-hours cover.

- **Community support** (issues and PRs): best effort.

## Contributing & maintaining

See [CONTRIBUTING.md](CONTRIBUTING.md) and
[GOVERNANCE.md](GOVERNANCE.md). The project has one maintainer; help
with triage, review or co-maintenance is the most valuable
contribution.

## The pain001 suite

This package is one of the pain001 suite's packages, which ship
together at one version. The closest is
[`pain001`](https://github.com/sebastienrousseau/pain001), the core
library that uses it. The
[family map](https://github.com/sebastienrousseau/pain001#the-pain001-ecosystem)
lists every member. Issues spanning several packages can be filed
against the core; the maintainer will route them.

## Supported versions

See [SECURITY.md](SECURITY.md#supported-versions). Requires Python
3.11 or newer; building from source needs Rust 1.89 or newer.
