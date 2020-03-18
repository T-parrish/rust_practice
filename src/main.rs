mod algorithms;
mod geometry;
mod temp;

use algorithms::algo_funcs;
use geometry::geo_structs;

fn main() {
    temp::convert_temp();
    geo_structs::rectangles();
    algo_funcs::write_output(algo_funcs::FuncTypes::Fibonacci, 10);
}
