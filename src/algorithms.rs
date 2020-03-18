pub mod algo_funcs {

    #[derive(Debug)]
    pub enum FuncTypes {
        Fibonacci
    }

    pub fn write_output(func: FuncTypes, n: u16) {
        let output = match func {
            FuncTypes::Fibonacci => gen_fib(n)
        };

        println!("{:?} with input {} results in {}", func, n, output);
    }

    // Generates nth fibonacci number
    fn gen_fib(n: u16) -> u32 {
        let mut x = 1;
        let mut y = 0;

        let mut step = 0;
        while step < n - 1 {
            let temp = x + y;
            y = x;
            x = temp;
            step = step + 1;
        }

        x
    }
}
