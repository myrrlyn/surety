# Changelog <!-- omit in toc -->

All notable changes will be documented in this file.

This document is written according to the [Keep a Changelog][kac] style.

[kac]: //keepachangelog.org

## 0.1.0

Initial construction.

### Added

- `Checked<T>` marker discards arithmetic upon overflow.
- `Wrapping<T>` marker forces integer arithmetic to consider `min_value()` and
  `max_value()` as adjacent numbers, with a continuous number circle instead of
  a number line.
- `Overflowing<T>` marker combines `Wrapping`’s arithmetic behavior with
  `Checked`’s overflow detection.
- `Saturating<T>` marker forces integer arithmetic to clamp at the minimum and
  maximum values, essentially changing the number line from a circle to a walled
  box.
