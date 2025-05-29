use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::Duration;
use link_shortener_backend::services::shortener::*;

pub fn bench_generate_short_code(c: &mut Criterion) {
    let mut group = c.benchmark_group("shortener");
    
    let test_cases = [
        ("short_url", "https://example.com"),
        ("medium_url", "https://example.com/path/to/resource"),
        ("long_url", "https://very-long-domain-name.example.com/very/long/path/to/resource?with=query&parameters=and&more=data"),
    ];
    
    for (name, url) in &test_cases {
        group.bench_with_input(
            BenchmarkId::new("generate_short_code", name),
            url,
            |b, url| {
                let salt = b"benchmark_salt";
                b.iter(|| {
                    generate_short_code(black_box(url), black_box(salt));
                });
            },
        );
    }
    
    group.finish();
}

pub fn bench_generate_short_code_base62(c: &mut Criterion) {
    let test_urls = [
        "https://example.com",
        "https://github.com/rust-lang/rust",
        "https://very-long-domain-name.example.com/path/to/resource?with=query&parameters=and&more=data",
    ];
    
    c.bench_function("generate_short_code_base62", |b| {
        b.iter(|| {
            let url = test_urls[fastrand::usize(..test_urls.len())];
            generate_short_code_base62(black_box(url));
        });
    });
}

pub fn bench_generate_short_code_with_timestamp(c: &mut Criterion) {
    c.bench_function("generate_short_code_with_timestamp", |b| {
        b.iter(|| {
            generate_short_code_with_timestamp(black_box("https://example.com"));
        });
    });
}

pub fn bench_is_valid_custom_code(c: &mut Criterion) {
    let test_codes = [
        "valid_code",
        "invalid code",
        "test123",
        "test-code_123",
        "",
        "a".repeat(25).as_str(),
    ];
    
    c.bench_function("is_valid_custom_code", |b| {
        b.iter(|| {
            let code = test_codes[fastrand::usize(..test_codes.len())];
            is_valid_custom_code(black_box(code));
        });
    });
}

criterion_group!(
    benches,
    bench_generate_short_code,
    bench_generate_short_code_base62,
    bench_generate_short_code_with_timestamp,
    bench_is_valid_custom_code
);

criterion_main!(benches); 