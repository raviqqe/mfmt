use bumpalo::Bump;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use mfmt::{Builder, Document, FormatOptions, format, line};

const SMALL_DOCUMENT_SIZE: usize = 100;
const LARGE_DOCUMENT_SIZE: usize = 10_000;
const NESTING_DEPTH: usize = 64;

type BuildDocument = for<'a> fn(&Builder<&'a Bump>, usize) -> Document<'a, &'a Bump>;

const DOCUMENTS: [(&str, BuildDocument); 5] = [
    ("lines", lines),
    ("flat_groups", flat_groups),
    ("line_suffixes", line_suffixes),
    ("nested_indent", nested_indent),
    ("nested_offside", nested_offside),
];

fn repeat<'a>(
    builder: &Builder<&'a Bump>,
    size: usize,
    build_item: impl Fn() -> Document<'a, &'a Bump>,
) -> Document<'a, &'a Bump> {
    builder.sequence((0..size).map(|_| builder.sequence([build_item(), line()])))
}

fn nest<'a>(
    wrap: impl Fn(Document<'a, &'a Bump>) -> Document<'a, &'a Bump>,
) -> Document<'a, &'a Bump> {
    (0..NESTING_DEPTH).fold("foo".into(), |document, _| wrap(document))
}

fn lines<'a>(builder: &Builder<&'a Bump>, size: usize) -> Document<'a, &'a Bump> {
    repeat(builder, size, || "foo".into())
}

fn flat_groups<'a>(builder: &Builder<&'a Bump>, size: usize) -> Document<'a, &'a Bump> {
    repeat(builder, size, || {
        builder.flatten(builder.sequence(["foo".into(), line(), "bar".into()]))
    })
}

fn line_suffixes<'a>(builder: &Builder<&'a Bump>, size: usize) -> Document<'a, &'a Bump> {
    repeat(builder, size, || {
        builder.sequence(["foo".into(), builder.line_suffixes([" ", "; ", "bar"])])
    })
}

fn nested_indent<'a>(builder: &Builder<&'a Bump>, size: usize) -> Document<'a, &'a Bump> {
    repeat(builder, size.div_ceil(NESTING_DEPTH), || {
        nest(|document| {
            builder.sequence([
                "{".into(),
                builder.indent(builder.sequence([line(), document])),
                line(),
                "}".into(),
            ])
        })
    })
}

fn nested_offside<'a>(builder: &Builder<&'a Bump>, size: usize) -> Document<'a, &'a Bump> {
    repeat(builder, size.div_ceil(NESTING_DEPTH), || {
        nest(|document| {
            builder.sequence([
                "(foo ".into(),
                builder.offside(
                    builder.r#break(builder.sequence([document, line(), "bar".into()])),
                    false,
                ),
                ")".into(),
            ])
        })
    })
}

fn build_document(criterion: &mut Criterion) {
    for (name, build) in DOCUMENTS {
        for (size_name, size) in [
            ("small", SMALL_DOCUMENT_SIZE),
            ("large", LARGE_DOCUMENT_SIZE),
        ] {
            criterion.bench_function(&format!("build_{name}_{size_name}"), |bencher| {
                bencher.iter(|| {
                    let allocator = Bump::new();

                    black_box(build(&Builder::new(&allocator), black_box(size)));
                })
            });
        }
    }
}

fn format_document(criterion: &mut Criterion) {
    for (name, build) in DOCUMENTS {
        for (size_name, size) in [
            ("small", SMALL_DOCUMENT_SIZE),
            ("large", LARGE_DOCUMENT_SIZE),
        ] {
            let allocator = Bump::new();
            let document = build(&Builder::new(&allocator), size);

            criterion.bench_function(&format!("format_{name}_{size_name}"), |bencher| {
                bencher.iter(|| {
                    let mut string = String::new();

                    format(black_box(&document), &mut string, FormatOptions::new(2)).unwrap();

                    black_box(string)
                })
            });
        }
    }
}

criterion_group!(benches, build_document, format_document);
criterion_main!(benches);
