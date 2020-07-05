---
name: Bug report
about: Report an issue with a teensy4-rs crate
title: ''
labels: ''
assignees: ''

---

**Before you submit...**
Let's make sure that this is a `teensy4-rs` issue. For more information, see the contributing guidelines in CONTRIBUTING.md.

- Is your Teensy 4's peripheral not working? If so, you should open a HAL issue in the [imxrt-rs] project.
- Are you looking for more information about a Teensy 4 peripheral? If so, you should request help in the [imxrt-rs] project.

[imxrt-rs]: https://github.com/imxrt-rs/imxrt-rs

Bugs that are pertinent to the `teensy4-rs` project include

- booting or start-up issues; or, something specific to the runtime crate.
- issues with the additional BSP layer, including USB logging.
- a feature that's available in the HAL, but not available in the `teensy4-bsp`.

If you're confident that this is a `teensy4-rs` issue, you may delete this section, and describe your bug in the sections below.

---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Describe how a developer can reproduce the issue. If the issue can be reproduced using one of the existing examples, consider describing the modifications that exhibit the bug.

**Expected behavior**
A clear and concise description of what you expected to happen.

**Additional context**
Add any other context about the problem here.
