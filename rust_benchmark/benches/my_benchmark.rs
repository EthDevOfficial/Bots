use std::vec;
use std::{thread, time};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::vec::Vec;

fn vec_test(n: &Vec<&str>) -> u32 {
    let mut a= 0;
    for s in n.iter(){
        a += match s {
            &"baoswap" => (1),
            _ => (1),
        };
    }
    return a;
}

fn string_benchmark(c: &mut Criterion) {
    let mut vec1 = vec!["honey","eqwewq","adsadsa","baoswap","wqe231321"];
    let vec2 = vec!["honey","eqwewq","adsadsa","baoswap","wqe231321"];

    let mut i = 0;

    while i < 10000 {    
        vec1.append(&mut vec2.clone());
        i = i + 1;
    }

    println!("{}", vec1.len());
    println!("{}", vec2.len());

    c.bench_function("String Vector", |b| b.iter(|| vec_test(black_box(&vec1))));
    c.bench_function("String Vector2", |b| b.iter(|| vec_test(black_box(&vec2))));
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn bench_fib(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(40)));
}

// fn tuple_benchmark(c: &mut Criterion) {
//     let mut vec: Vec<(String, u32)> = Vec::new();

//     c.bench_function("fib 20", |b| b.iter(|| VecTest(vec)));
// }

// fn hashmap_benchmark(c: &mut Criterion) {
//     let mut vec: Vec<(String, u32)> = Vec::new();

//     c.bench_function("fib 20", |b| b.iter(|| VecTest(vec)));
// }

criterion_group!(benches, string_benchmark);
criterion_main!(benches);