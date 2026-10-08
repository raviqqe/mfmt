use allocator_api2::alloc::{AllocError, Allocator, Global, Layout};
use bumpalo::Bump;
use core::{cell::RefCell, ptr::NonNull};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use mfmt::{Builder, Document, FormatOptions, format, line};

const SMALL_DOCUMENT_SIZE: usize = 100;
const LARGE_DOCUMENT_SIZE: usize = 10_000;
const NESTING_DEPTH: usize = 64;

type BuildDocument<A> = for<'a> fn(&Builder<&'a A>, usize) -> Document<'a>;

/// An arena of memory blocks from the global allocator.
///
/// It frees all the memory blocks on drop since document builders leak them.
#[derive(Default)]
struct GlobalArena {
    blocks: RefCell<Vec<(NonNull<u8>, Layout)>>,
}

// SAFETY: Memory blocks are valid until the arena is dropped.
unsafe impl Allocator for &GlobalArena {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let block = Global.allocate(layout)?;

        self.blocks.borrow_mut().push((block.cast(), layout));

        Ok(block)
    }

    unsafe fn deallocate(&self, _pointer: NonNull<u8>, _layout: Layout) {}
}

impl Drop for GlobalArena {
    fn drop(&mut self) {
        for (block, layout) in self.blocks.get_mut().drain(..) {
            // SAFETY: The block is allocated by the global allocator with the
            // layout.
            unsafe { Global.deallocate(block, layout) }
        }
    }
}

fn repeat<'a, A>(
    builder: &Builder<&'a A>,
    size: usize,
    build_item: impl Fn() -> Document<'a>,
) -> Document<'a>
where
    for<'b> &'b A: Allocator,
{
    builder.sequence((0..size).map(|_| builder.sequence([build_item(), line()])))
}

fn nest<'a>(wrap: impl Fn(Document<'a>) -> Document<'a>) -> Document<'a> {
    (0..NESTING_DEPTH).fold("foo".into(), |document, _| wrap(document))
}

fn lines<'a, A>(builder: &Builder<&'a A>, size: usize) -> Document<'a>
where
    for<'b> &'b A: Allocator,
{
    repeat(builder, size, || "foo".into())
}

fn flat_groups<'a, A>(builder: &Builder<&'a A>, size: usize) -> Document<'a>
where
    for<'b> &'b A: Allocator,
{
    repeat(builder, size, || {
        builder.flatten(builder.sequence(["foo".into(), line(), "bar".into()]))
    })
}

fn line_suffixes<'a, A>(builder: &Builder<&'a A>, size: usize) -> Document<'a>
where
    for<'b> &'b A: Allocator,
{
    repeat(builder, size, || {
        builder.sequence(["foo".into(), builder.line_suffixes([" ", "; ", "bar"])])
    })
}

fn nested_indent<'a, A>(builder: &Builder<&'a A>, size: usize) -> Document<'a>
where
    for<'b> &'b A: Allocator,
{
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

fn nested_offside<'a, A>(builder: &Builder<&'a A>, size: usize) -> Document<'a>
where
    for<'b> &'b A: Allocator,
{
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

fn benchmark_allocator<A: Default>(criterion: &mut Criterion, allocator_name: &str)
where
    for<'a> &'a A: Allocator,
{
    for (name, build) in [
        ("lines", lines as BuildDocument<A>),
        ("flat_groups", flat_groups),
        ("line_suffixes", line_suffixes),
        ("nested_indent", nested_indent),
        ("nested_offside", nested_offside),
    ] {
        for (size_name, size) in [
            ("small", SMALL_DOCUMENT_SIZE),
            ("large", LARGE_DOCUMENT_SIZE),
        ] {
            criterion.bench_function(
                &format!("build_{name}_{size_name}_{allocator_name}"),
                |bencher| {
                    bencher.iter(|| {
                        let allocator = A::default();

                        black_box(build(&Builder::new(&allocator), black_box(size)));
                    })
                },
            );

            let allocator = A::default();
            let document = build(&Builder::new(&allocator), size);

            criterion.bench_function(
                &format!("format_{name}_{size_name}_{allocator_name}"),
                |bencher| {
                    bencher.iter(|| {
                        let mut string = String::new();

                        format(black_box(&document), &mut string, FormatOptions::new(2)).unwrap();

                        black_box(string)
                    })
                },
            );
        }
    }
}

fn benchmark(criterion: &mut Criterion) {
    benchmark_allocator::<Bump>(criterion, "bump");
    benchmark_allocator::<GlobalArena>(criterion, "global");
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
