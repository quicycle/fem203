extern crate fem203;

use fem203::*;

fn main() {
    println!("Gausian values ftw");

    let vals = (-100..100)
        .map(|t| gaussian(t as f64 / 100.0, 0.5, 0.1))
        .collect::<Vec<_>>();
    println!("{:?}", vals);
}
