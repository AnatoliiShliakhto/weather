use ::criterion::{Criterion, criterion_group, criterion_main};
use ::weather_providers::{Provider, create_provider};

fn bench_create_provider(c: &mut Criterion) {
    c.bench_function("create_provider_mock", |b| {
        b.iter(|| create_provider(Provider::Mock))
    });

    c.bench_function("create_provider_openweather", |b| {
        b.iter(|| create_provider(Provider::OpenWeather))
    });
}

fn bench_get_weather_mock(c: &mut Criterion) {
    let provider = create_provider(Provider::Mock);
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("get_weather_mock", |b| {
        b.to_async(&rt)
            .iter(|| async { provider.get_weather(Some("mock-key"), "London", None).await })
    });
}

criterion_group!(benches, bench_create_provider, bench_get_weather_mock);
criterion_main!(benches);
