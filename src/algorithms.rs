pub mod algo_funcs {

    use std::collections::HashMap;
    // Given a list of integers, find the mean, median, and mode
    pub fn get_stats(mut targets: Vec<i32>) -> (f32, i32, i32) {
        let mut sum = 0;
        let target_count = targets.len();
        let middle = target_count / 2;
        let mut number_counter = HashMap::new();

        for t in &targets {
            sum += t;
            let count = number_counter.entry(t).or_insert(0);
            *count += 1;
        }
        let mode = {
            let mut max = 0;
            let mut mode = 0;
            for (k, v) in &number_counter {
                if v >= &max {
                    max = *v;
                    mode = **k;
                }
            }
            mode
        };

        println!("number counter: {:?}", number_counter);
        let mean = sum as f32 / target_count as f32;
        let median = {
            targets.sort();
            targets[middle]
        };

        (mean, median, mode)
    }

    // Generates nth fibonacci number
    pub fn gen_fib(n: u16) -> u32 {
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

    pub fn pig_latin(text: &str) -> String {
        let mut out_string = String::from("");
        let vowels: String = String::from("aeiouAEIOU");

        for word in text.split_whitespace() {
            let start = &word[..1];
            // if word starts with a vowel, maintain order and add 'ay'
            let pigged = match vowels.contains(start) {
                true => {
                    format!("{}ay ", &word[..])
                },
                // Otherwise, move first letter to end and add 'ay'
                false => {
                    format!("{}{}{}", &word[1..], &word[0..1], "ay ")
                },
            };

            out_string.push_str(&pigged[..]);
        }

        String::from(out_string.trim())
    }
}
