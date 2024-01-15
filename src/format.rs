use crate::{document::Document, FormatOptions};
use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::iter::repeat;

struct Context<'a> {
    outputs: Vec<&'a str>,
    // Omit extra indent output so that we do not need to remove them later.
    next_level: usize,
    line_suffixes: Vec<&'a str>,
    space: &'a str,
    options: FormatOptions,
}

pub fn format(document: &Document, options: FormatOptions) -> String {
    let space = options.space.to_string();
    let mut context = Context {
        outputs: vec![],
        next_level: 0,
        line_suffixes: vec![],
        space: &space,
        options,
    };

    format_document(&mut context, document, 0, true);

    context.outputs.concat()
}

fn format_document<'a>(
    context: &mut Context<'a>,
    document: &Document<'a>,
    level: usize,
    broken: bool,
) {
    match document {
        Document::Break(broken, document) => format_document(context, document, level, *broken),
        Document::Indent(document) => format_document(context, document, level + 1, broken),
        Document::Line => {
            if broken {
                format_line(context, level);
            } else {
                context.outputs.push(" ");
            }
        }
        Document::LineSuffix(suffix) => {
            if !suffix.is_empty() {
                flush(context);
            }

            context.line_suffixes.push(suffix);
        }
        Document::Sequence(documents) => {
            for document in *documents {
                format_document(context, document, level, broken);
            }
        }
        Document::String(string) => {
            if !string.is_empty() {
                flush(context);
            }

            context.outputs.push(string);
        }
    }
}

fn format_line(context: &mut Context, level: usize) {
    context
        .outputs
        .extend(context.line_suffixes.drain(..).chain(["\n"]));

    context.next_level = level;
}

fn flush(context: &mut Context) {
    context
        .outputs
        .extend(repeat(context.space).take(context.next_level * context.options.indent));
    context.next_level = 0;
}

#[cfg(test)]
mod tests {
    use core::num::NonZeroUsize;

    use super::{super::build::*, *};
    use alloc::boxed::Box;
    use indoc::indoc;

    const DEFAULT_OPTIONS: FormatOptions = FormatOptions::new(NonZeroUsize::new(2).unwrap());

    fn allocate<T>(value: T) -> &'static T {
        Box::leak(Box::new(value))
    }

    #[test]
    fn format_string() {
        assert_eq!(format(&"foo".into(), DEFAULT_OPTIONS), "foo");
    }

    mod group {
        use super::*;

        fn create_group() -> Document<'static> {
            sequence(allocate([
                "{".into(),
                indent(allocate(sequence(allocate([
                    line(),
                    "foo".into(),
                    line(),
                    "bar".into(),
                ])))),
                line(),
                "}".into(),
            ]))
        }

        #[test]
        fn format_flat_group() {
            assert_eq!(
                format(&flatten(&create_group()), DEFAULT_OPTIONS),
                "{ foo bar }"
            );
        }

        #[test]
        fn format_empty_line_with_indent() {
            assert_eq!(format(&indent(&line()), DEFAULT_OPTIONS), "\n");
        }

        #[test]
        fn format_broken_group() {
            assert_eq!(
                format(&create_group(), DEFAULT_OPTIONS),
                indoc!(
                    "
                    {
                      foo
                      bar
                    }
                    "
                )
                .trim(),
            );
        }

        #[test]
        fn format_unbroken_group_in_broken_group() {
            assert_eq!(
                format(
                    &sequence(&[
                        "{".into(),
                        indent(&sequence(&[line(), flatten(&create_group())])),
                        line(),
                        "}".into(),
                    ]),
                    DEFAULT_OPTIONS
                ),
                indoc!(
                    "
                    {
                      { foo bar }
                    }
                    "
                )
                .trim(),
            );
        }
    }

    mod line_suffix {
        use super::*;

        #[test]
        fn format_line_suffix_between_strings() {
            assert_eq!(
                format(
                    &sequence(&["{".into(), line_suffix("foo"), "}".into(), line()]),
                    DEFAULT_OPTIONS
                ),
                "{}foo\n",
            );
        }

        #[test]
        fn format_two_line_suffixes_between_strings() {
            assert_eq!(
                format(
                    &sequence(&[
                        "{".into(),
                        line_suffix("foo"),
                        line_suffix("bar"),
                        "}".into(),
                        line()
                    ]),
                    DEFAULT_OPTIONS
                ),
                "{}foobar\n",
            );
        }
    }
}
