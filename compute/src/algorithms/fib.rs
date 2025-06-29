use std::{thread::sleep, time::Duration};

use num_bigint::{BigUint, ToBigUint};
use tracing::{debug, info};

fn fib(mut n: u128) -> BigUint {
    let mut a = BigUint::ZERO;
    let mut b = 1.to_biguint().unwrap();
    while n > 0 {
        let temp = a;
        a = b.clone();
        b = temp + b;
        n -= 1;
        debug!("Computed intermediate value ({})", a);
        sleep(Duration::from_millis(850));
    }
    a
}

pub fn compute(max_n: u128) {
    for i in 0..max_n {
        let fib = fib(i);
        sleep(Duration::from_secs(2));
        info!("Fibonacci({}) = {}", i + 1, fib);
    }
}
