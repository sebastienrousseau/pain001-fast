# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Rendered by `make demo` (VHS: https://github.com/charmbracelet/vhs)
# with .venv/bin first on PATH, so `python` is the checkout's build of
# the extension. It runs the identifier and batch examples from
# examples/, which assert what they print.

Output .github/demo.gif

Set Shell "bash"
Set FontSize 15
Set Width 1200
Set Height 300
Set Padding 20
Set Theme "Catppuccin Mocha"
Set TypingSpeed 35ms

Hide
Type "export PS1='$ ' && clear"
Enter
Show

Type "python examples/01_validate_identifiers.py"
Enter
Wait@60s /\$$/
Sleep 3s

Type "python examples/03_validate_batch.py"
Enter
Wait@60s /\$$/
Sleep 4s
