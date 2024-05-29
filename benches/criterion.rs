use raytracer::raytracer;
use raytracer::{read_scene_file, write_image};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn render_image(file_path_str: &String) {
    let scene = read_scene_file(file_path_str);
    let pixels = raytracer::render(&scene);
    let img_obj = raytracer::build_image((scene.img_width, scene.img_height), &pixels);
    write_image(&"out.png".to_string(), img_obj);
}

fn criterion_benchmark_scene00(c: &mut Criterion) {
    c.bench_function("scene00", |b| {
        b.iter(|| render_image(&"test_scenes/scene00.test".to_owned()))
    });
}

fn criterion_benchmark_scene01(c: &mut Criterion) {
    c.bench_function("scene01", |b| {
        b.iter(|| render_image(black_box(&"test_scenes/scene01.test".to_owned())))
    });
}

fn criterion_benchmark_scene02(c: &mut Criterion) {
    c.bench_function("scene02", |b| {
        b.iter(|| render_image(black_box(&"test_scenes/scene02.test".to_owned())))
    });
}

fn criterion_benchmark_scene03(c: &mut Criterion) {
    c.bench_function("scene03", |b| {
        b.iter(|| render_image(black_box(&"test_scenes/scene03.test".to_owned())))
    });
}

fn criterion_benchmark_scene04_ambient(c: &mut Criterion) {
    c.bench_function("scene04_ambient", |b| {
        b.iter(|| render_image(black_box(&"test_scenes/scene04-ambient.test".to_owned())))
    });
}

fn criterion_benchmark_scene04_diffuse(c: &mut Criterion) {
    c.bench_function("scene04_diffuse", |b| {
        b.iter(|| render_image(black_box(&"test_scenes/scene04-diffuse.test".to_owned())))
    });
}

fn criterion_benchmark_scene04_emission(c: &mut Criterion) {
    c.bench_function("scene04_emission", |b| {
        b.iter(|| render_image(black_box(&"test_scenes/scene04-emission.test".to_owned())))
    });
}

fn criterion_benchmark_scene04_specular(c: &mut Criterion) {
    c.bench_function("scene04_specular", |b| {
        b.iter(|| render_image(black_box(&"test_scenes/scene04-specular.test".to_owned())))
    });
}

fn criterion_benchmark_scene05(c: &mut Criterion) {
    c.bench_function("scene05", |b| {
        b.iter(|| render_image(black_box(&"test_scenes/scene05.test".to_owned())))
    });
}

fn criterion_benchmark_scene06(c: &mut Criterion) {
    c.bench_function("scene06", |b| {
        b.iter(|| render_image(black_box(&"test_scenes/scene06.test".to_owned())))
    });
}

fn criterion_benchmark_scene07(c: &mut Criterion) {
    c.bench_function("scene07", |b| {
        b.iter(|| render_image(black_box(&"test_scenes/scene07.test".to_owned())))
    });
}

criterion_group!{
    benches,
    criterion_benchmark_scene00,
    criterion_benchmark_scene01,
    criterion_benchmark_scene02,
    criterion_benchmark_scene03
}
criterion_main!(benches);
