mod algorithms;
// mod geometry;
// mod temp;
mod interface;

use algorithms::algo_funcs;
// use geometry::geo_structs;
use interface::employee_admin;
use interface::number_guessing;

fn main() {
    // temp::convert_temp();
    // geo_structs::rectangles();
    println!("Fibonacci {} is: {}", 10, algo_funcs::gen_fib(10));

    let (a, b, c) = algo_funcs::get_stats(vec![10, 4, 3, 8, 22, 36, 4, 6, 3]);
    println!("Mean: {} -- Median: {} -- Mode: {}", a, b, c);

    let pigged = algo_funcs::pig_latin("westside walk it out");
    println!("{}", pigged);

    employee_admin::start_program();
    number_guessing::start_game();
    
}
