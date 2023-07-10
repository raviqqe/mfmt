# mfmt

[![GitHub Action](https://img.shields.io/github/actions/workflow/status/raviqqe/mfmt/test.yaml?branch=main&style=flat-square)](https://github.com/raviqqe/mfmt/actions?query=workflow%3Atest)
[![Crate](https://img.shields.io/crates/v/mfmt.svg?style=flat-square)](https://crates.io/crates/mfmt)
[![License](https://img.shields.io/github/license/raviqqe/mfmt.svg?style=flat-square)](https://github.com/raviqqe/mfmt/blob/main/UNLICENSE)

Meta formatter library in Rust

## Technical notes

Unlike [the Wadler's algorithm][wadler] or some other formatters like prettier, we do
not need to search the best format given source codes. For example, we do
not have any "group" combinator.

However, we are rather given the "best" format by all information available
in the source codes like Go.

We need soft-line and if-break nodes to make nodes totally agnostic about if
parent nodes are broken or not. But that also makes IR more complex.
(e.g. handling trailing commas in function calls)

## References

- [The Wadler's algorithm][wadler]

## License

[The Unlicense](UNLICENSE)

[wadler]: https://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf
