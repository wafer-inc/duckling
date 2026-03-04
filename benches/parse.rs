use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use duckling::{parse, Context, DimensionKind, Lang, Locale, Options};

fn bench_parse_time(c: &mut Criterion) {
    let locale = Locale::new(Lang::EN, None);
    let context = Context::default();
    let options = Options::default();
    let dims = [DimensionKind::Time];

    let inputs: &[(&str, &str)] = &[
        ("short", "tomorrow at 3pm"),
        ("medium", "from 13 to 15 of July"),
        ("long", "meet me next Wednesday at 2:30pm for about 2 hours"),
        ("no_match", "the quick brown fox jumps over the lazy dog"),
        ("empty", ""),
    ];

    let mut group = c.benchmark_group("parse_time");
    for (name, text) in inputs {
        group.bench_with_input(BenchmarkId::new("en", name), text, |b, text| {
            b.iter(|| parse(black_box(text), &locale, &dims, &context, &options));
        });
    }
    group.finish();
}

fn bench_parse_numeral(c: &mut Criterion) {
    let locale = Locale::new(Lang::EN, None);
    let context = Context::default();
    let options = Options::default();
    let dims = [DimensionKind::Numeral];

    let inputs: &[(&str, &str)] = &[
        ("word", "forty-two"),
        ("digits", "100000"),
        ("mixed", "3.5 million"),
    ];

    let mut group = c.benchmark_group("parse_numeral");
    for (name, text) in inputs {
        group.bench_with_input(BenchmarkId::new("en", name), text, |b, text| {
            b.iter(|| parse(black_box(text), &locale, &dims, &context, &options));
        });
    }
    group.finish();
}

fn bench_parse_all_dims(c: &mut Criterion) {
    let locale = Locale::new(Lang::EN, None);
    let context = Context::default();
    let options = Options::default();
    let dims = [];

    c.bench_function("parse_all_dims", |b| {
        b.iter(|| {
            parse(
                black_box("tomorrow at 3pm for $50"),
                &locale,
                &dims,
                &context,
                &options,
            )
        });
    });
}

criterion_group!(
    benches,
    bench_parse_time,
    bench_parse_numeral,
    bench_parse_all_dims
);
criterion_main!(benches);
