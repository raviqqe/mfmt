# mfmt

[![GitHub Action](https://img.shields.io/github/actions/workflow/status/raviqqe/mfmt/test.yaml?branch=main&style=flat-square)](https://github.com/raviqqe/mfmt/actions?query=workflow%3Atest)
[![Crate](https://img.shields.io/crates/v/mfmt.svg?style=flat-square)](https://crates.io/crates/mfmt)
[![License](https://img.shields.io/github/license/raviqqe/mfmt.svg?style=flat-square)](https://github.com/raviqqe/mfmt/blob/main/UNLICENSE)

Meta formatter library in Rust

`mfmt` is a language formatter library written in Rust inspired by `go fmt`. It's designed to be (almost) configuration-free and generous about styling. What it is focused on is simply aligning indentations.

This library is used in the following projects.

- [Pen programming language](https://github.com/pen-lang/pen)
- [`schemat`, the Scheme formatter](https://github.com/raviqqe/schemat)

## Technical notes

Unlike [the Wadler's algorithm][wadler] or some other formatters like prettier, `mfmt` does not search the best format given source codes. For example, we do not have any "group" combinator. Instead, we rather give `mfmt` information to reconstruct the "best" format that is available in the original source codes like Go.

## References

- [A prettier printer by Philip Wadler][wadler]

## License

[The Unlicense](https://github.com/raviqqe/mfmt/blob/main/UNLICENSE)

[wadler]: https://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf
