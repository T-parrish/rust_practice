mod algorithms;
// mod geometry;
// mod temp;

use algorithms::algo_funcs;
// use geometry::geo_structs;

fn main() {
    // temp::convert_temp();
    // geo_structs::rectangles();
    println!("Fibonacci {} is: {}", 10, algo_funcs::gen_fib(10));

    let (a, b, c) = algo_funcs::get_stats(vec![10, 4, 3, 8, 22, 36, 4, 6, 3]);
    println!("Mean: {} -- Median: {} -- Mode: {}", a, b, c);
}
