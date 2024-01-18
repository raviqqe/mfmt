mod state;

use crate::{document::Document, FormatOptions};
use alloc::{string::ToString, vec, vec::Vec};
use core::{
    fmt::{self, Write},
    iter::repeat,
};
use state::State;

#[derive(Debug)]
struct Context<'a, W: Write> {
    writer: &'a mut W,
    column: usize,
    next_indent: usize,
    line_suffixes: Vec<&'a str>,
    space: &'a str,
    indent: usize,
}

/// Formats a document.
pub fn format(document: &Document, mut writer: impl Write, options: FormatOptions) -> fmt::Result {
    let space = options.space().to_string();
    let mut context = Context {
        writer: &mut writer,
        column: 0,
        next_indent: 0,
        line_suffixes: vec![],
        space: &space,
        indent: options.indent(),
    };

    format_document(&mut context, document, Default::default())
}

fn format_document<'a>(
    context: &mut Context<'a, impl Write>,
    document: &'a Document,
    state: State,
) -> fmt::Result {
    #[cfg(test)]
    extern crate std;
    #[cfg(test)]
    std::dbg!(context.next_indent, &context.column, state, document);
    match document {
        Document::Break(broken, document) => {
            format_document(context, document, state.set_broken(*broken))?
        }
        Document::Indent(document) => {
            format_document(
                context,
                document,
                state.set_indent(state.indent() + context.indent),
            )?;
        }
        Document::Line => {
            if state.broken() {
                for string in context.line_suffixes.drain(..).chain(["\n"]) {
                    context.writer.write_str(string)?;
                }

                context.next_indent = state.indent();
                context.column = state.indent();
            } else {
                context.writer.write_char(' ')?;
                context.column += 1;
            }
        }
        Document::LineSuffix(suffix) => {
            if !suffix.is_empty() {
                flush(context)?;
            }

            context.line_suffixes.push(suffix);
        }
        Document::Offside(document) => {
            format_document(
                context,
                document,
                if state.broken() {
                    state
                } else {
                    state.set_indent(context.column)
                },
            )?;
        }
        Document::Sequence(documents) => {
            for document in *documents {
                format_document(context, document, state)?;
            }
        }
        Document::String(string) => {
            if !string.is_empty() {
                flush(context)?;
            }

            context.writer.write_str(string)?;
            context.column += string.len();
        }
    }

    Ok(())
}

fn flush(context: &mut Context<impl Write>) -> fmt::Result {
    // Flush an indent lazily.
    for string in repeat(context.space).take(context.next_indent) {
        context.writer.write_str(string)?;
    }

    // Do not render any indent until the next newline.
    context.next_indent = 0;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{super::build::*, *};
    use alloc::{boxed::Box, string::String};
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    fn default_options() -> FormatOptions {
        FormatOptions::new(2)
    }

    fn allocate<T>(value: T) -> &'static T {
        Box::leak(Box::new(value))
    }

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

    fn format_to_string(document: &Document, options: FormatOptions) -> String {
        let mut string = String::new();

        format(document, &mut string, options).unwrap();

        string
    }

    #[test]
    fn format_string() {
        assert_eq!(format_to_string(&"foo".into(), default_options()), "foo");
    }

    mod group {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn format_flat_group() {
            assert_eq!(
                format_to_string(&flatten(&create_group()), default_options()),
                "{ foo bar }"
            );
        }

        #[test]
        fn format_empty_line_with_indent() {
            assert_eq!(format_to_string(&indent(&line()), default_options()), "\n");
        }

        #[test]
        fn format_broken_group() {
            assert_eq!(
                format_to_string(&create_group(), default_options()),
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
                format_to_string(
                    &sequence(&[
                        "{".into(),
                        indent(&sequence(&[line(), flatten(&create_group())])),
                        line(),
                        "}".into(),
                    ]),
                    default_options()
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
        use pretty_assertions::assert_eq;

        #[test]
        fn format_line_suffix_between_strings() {
            assert_eq!(
                format_to_string(
                    &sequence(&["{".into(), line_suffix("foo"), "}".into(), line()]),
                    default_options()
                ),
                "{}foo\n",
            );
        }

        #[test]
        fn format_two_line_suffixes_between_strings() {
            assert_eq!(
                format_to_string(
                    &sequence(&[
                        "{".into(),
                        line_suffix("foo"),
                        line_suffix("bar"),
                        "}".into(),
                        line()
                    ]),
                    default_options()
                ),
                "{}foobar\n",
            );
        }
    }

    mod offside {
        use super::*;
        use pretty_assertions::assert_eq;

        fn create_group() -> Document<'static> {
            sequence(allocate([
                "foo".into(),
                indent(allocate(sequence(allocate([
                    line(),
                    offside(allocate(r#break(allocate(sequence(allocate([
                        "bar".into(),
                        line(),
                        "baz".into(),
                    ])))))),
                ])))),
            ]))
        }

        #[test]
        fn format_flat_group() {
            assert_eq!(
                format_to_string(&flatten(&create_group()), default_options().set_indent(2)),
                indoc!(
                    "
                    foo bar
                        baz
                    "
                )
                .trim(),
            );
        }

        #[test]
        fn format_broken_group() {
            assert_eq!(
                format_to_string(&r#break(&create_group()), default_options().set_indent(2)),
                indoc!(
                    "
                    foo
                      bar
                      baz
                    "
                )
                .trim(),
            );
        }

        mod nested {
            use super::*;
            use pretty_assertions::assert_eq;

            fn create_groups(
                inner: for<'a> fn(&'a Document<'a>) -> Document<'a>,
            ) -> Document<'static> {
                sequence(allocate([
                    "foo".into(),
                    indent(allocate(sequence(allocate([
                        line(),
                        offside(allocate(r#break(allocate(sequence(allocate([
                            "bar".into(),
                            line(),
                            "baz".into(),
                            line(),
                            inner(allocate(sequence(allocate([
                                "qux".into(),
                                indent(allocate(sequence(allocate([
                                    line(),
                                    offside(allocate(r#break(allocate(sequence(allocate([
                                        "quux".into(),
                                        line(),
                                        "corge".into(),
                                    ])))))),
                                ])))),
                            ])))),
                        ])))))),
                    ])))),
                ]))
            }

            #[test]
            fn format_flat_outer_with_flat_inner() {
                assert_eq!(
                    format_to_string(
                        &flatten(&create_groups(flatten)),
                        default_options().set_indent(2)
                    ),
                    indoc!(
                        "
                    foo bar
                        baz
                        qux quux
                            corge
                    "
                    )
                    .trim(),
                );
            }

            #[test]
            fn format_flat_outer_with_broken_inner() {
                assert_eq!(
                    format_to_string(
                        &flatten(&create_groups(r#break)),
                        default_options().set_indent(2)
                    ),
                    indoc!(
                        "
                    foo bar
                        baz
                        qux
                          quux
                          corge
                    "
                    )
                    .trim(),
                );
            }

            #[test]
            fn format_broken_outer_with_flat_inner() {
                assert_eq!(
                    format_to_string(
                        &r#break(&create_groups(flatten)),
                        default_options().set_indent(2)
                    ),
                    indoc!(
                        "
                    foo
                      bar
                      baz
                      qux quux
                          corge
                    "
                    )
                    .trim(),
                );
            }

            #[test]
            fn format_broken_outer_with_broken_inner() {
                assert_eq!(
                    format_to_string(
                        &r#break(&create_groups(r#break)),
                        default_options().set_indent(2)
                    ),
                    indoc!(
                        "
                    foo
                      bar
                      baz
                      qux
                        quux
                        corge
                    "
                    )
                    .trim(),
                );
            }

            #[test]
            fn format_two_flat_groups() {
                assert_eq!(
                    format_to_string(
                        &flatten(&sequence(&[
                            "qux".into(),
                            line(),
                            r#break(&sequence(&[
                                flatten(&create_group()),
                                line(),
                                flatten(&create_group())
                            ]))
                        ])),
                        default_options().set_indent(1)
                    ),
                    indoc!(
                        "
                        qux foo bar
                                baz
                            foo bar
                                baz
                        "
                    )
                    .trim(),
                );
            }
        }
    }

    mod space {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn format_broken_group_with_space() {
            assert_eq!(
                format_to_string(&create_group(), default_options().set_indent(1)),
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
        fn format_broken_group_with_two_spaces() {
            assert_eq!(
                format_to_string(&create_group(), default_options().set_indent(2)),
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
        fn format_broken_group_with_four_spaces() {
            assert_eq!(
                format_to_string(&create_group(), default_options().set_indent(4)),
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
        fn format_broken_group_with_tab() {
            assert_eq!(
                format_to_string(&create_group(), FormatOptions::tab()),
                indoc!(
                    "
                    {
                    \tfoo
                    \tbar
                    }
                    "
                )
                .trim(),
            );
        }
    }
}
