# mfmt

[![GitHub Action](https://img.shields.io/github/actions/workflow/status/raviqqe/mfmt/test.yaml?branch=main&style=flat-square)](https://github.com/raviqqe/mfmt/actions?query=workflow%3Atest)
[![Crate](https://img.shields.io/crates/v/mfmt.svg?style=flat-square)](https://crates.io/crates/mfmt)
[![License](https://img.shields.io/github/license/raviqqe/mfmt.svg?style=flat-square)](https://github.com/raviqqe/mfmt/blob/main/UNLICENSE)

Meta formatter library in Rust

## Technical notes

Unlike [the Wadler's algorithm][wadler] or some other formatters like prettier, `mfmt` does not search the best format given source codes. For example, we do not have any "group" combinator. Instead, we rather give `mfmt` information to reconstruct the "best" format that is available in the original source codes like Go.

## References

- [The Wadler's algorithm][wadler]

## License

[The Unlicense](UNLICENSE)

[wadler]: https://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf
