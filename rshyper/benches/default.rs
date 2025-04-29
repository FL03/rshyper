mod fib {
    /// fibonacci(n) returns the nth fibonacci number
    /// This function uses the definition of Fibonacci where:
    /// F(0) = F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0
    ///
    /// Warning: This will overflow the 128-bit unsigned integer at n=186
    pub fn fibonacci(n: u32) -> u128 {
        // Use a and b to store the previous two values in the sequence
        let mut a = 0;
        let mut b = 1;
        for _i in 0..n {
            // As we iterate through, move b's value into a and the new computed
            // value into b.
            let c = a + b;
            a = b;
            b = c;
        }
        b
    }

    /// fibonacci(n) returns the nth fibonacci number
    /// This function uses the definition of Fibonacci where:
    /// F(0) = F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0
    ///
    /// Warning: This will overflow the 128-bit unsigned integer at n=186
    pub fn recursive_fibonacci(n: u32) -> u128 {
        // Call the actual tail recursive implementation, with the extra
        // arguments set up.
        _recursive_fibonacci(n, 0, 1)
    }

    fn _recursive_fibonacci(n: u32, previous: u128, current: u128) -> u128 {
        if n == 0 {
            current
        } else {
            _recursive_fibonacci(n - 1, current, current + previous)
        }
    }

    pub struct Fibonacci {
        curr: u32,
        next: u32,
    }
    #[allow(unused)]
    impl Fibonacci {
        pub fn new() -> Fibonacci {
            Fibonacci { curr: 0, next: 1 }
        }

        pub fn reset(&mut self) {
            self.curr = 0;
            self.next = 1;
        }

        fn compute_next(&self) -> u32 {
            self.curr + self.next
        }
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            use core::mem::replace;
            let new_next = self.compute_next();
            let new_curr = replace(&mut self.next, new_next);

            Some(replace(&mut self.curr, new_curr))
        }
    }
}

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

fn bench_fib(c: &mut Criterion) {
    c.bench_function("fibonacci", |b| b.iter(|| fib::fibonacci(black_box(20))));
}

fn bench_recursive_fib(c: &mut Criterion) {
    c.bench_function("recursive_fib", |b| {
        b.iter(|| fib::recursive_fibonacci(black_box(20)))
    });
}

fn bench_iter_fib(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci Iter");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(50);

    for &n in &[10, 50, 100, 500, 1000] {
        group.bench_with_input(BenchmarkId::new("iter_fibonacci", n), &n, |b, &count| {
            b.iter_batched(
                fib::Fibonacci::new,
                |fib| {
                    black_box(fib.take(count as usize).collect::<Vec<u32>>());
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_fib, bench_iter_fib, bench_recursive_fib,);
criterion_main!(benches);
