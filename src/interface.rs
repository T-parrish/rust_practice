// Using a hash map and vectors, create a text interface to allow
// a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

pub mod employee_admin {
    use std::collections::HashMap;
    use std::io;
    pub fn start_program() {
        let mut employee_dict: HashMap<String, String> = HashMap::new();

        loop {
            let mut action_type = String::new();
            println!("\nPress 1 to add employees \nPress 2 to list employees by department. \nPress 3 to quit.");

            io::stdin()
                .read_line(&mut action_type)
                .expect("Enter a number!");

            match action_type.trim().parse() {
                Ok(1) => {
                    let (name, job) = add_employees();
                    employee_dict.insert(name, job);
                }
                Ok(2) => {
                    let emp_list = list_employees(&employee_dict);
                    println!("\n");
                    for emp in emp_list.0.iter() {
                        println!("{} ------- {}", emp, emp_list.1);
                    }
                }
                Ok(3) => break,
                _ => {
                    println!("please pick 1, 2, or 3\n");
                    continue;
                }
            }
        }
    }

    fn add_employees() -> (String, String) {
        println!("\nPlease enter Employee's name.");
        let mut emp = String::new();
        io::stdin()
            .read_line(&mut emp)
            .expect("Please enter employee's name.");

        let emp: String = emp.trim().parse().expect("Not a valid name.");

        println!("\nPlease enter the employee's job title.");
        let mut job = String::new();
        io::stdin()
            .read_line(&mut job)
            .expect("Please enter employee's job title.");
        let job: String = job.trim().parse().expect("Not a valid name.");

        (emp, job)
    }

    fn list_employees(employees: &HashMap<String, String>) -> (Vec<String>, String) {
        println!("\nWhat department would you like to list?");
        let mut target: String = String::new();

        io::stdin()
            .read_line(&mut target)
            .expect("Please enter target department.");

        let target: String = target
            .trim()
            .parse()
            .expect("please enter a department name");

        let mut output: Vec<String> = Vec::new();
        for (k, v) in employees {
            if &target == v {
                output.push(String::from(format!("{}", k)))
            }
        }
        output.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        (output, target)
    }
}

pub mod number_guessing {
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;

    pub fn start_game() {
        println!("Guess the number!");
        println!("Please input your guess.");
        let secret_number = rand::thread_rng().gen_range(0, 101);
        loop {
            let mut guess = String::new();

            io::stdin().read_line(&mut guess).expect("Enter a number!");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("You guessed: {}", guess);
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too low!"),
                Ordering::Greater => println!("Too Big!"),
                Ordering::Equal => {
                    println!("JUST RIGHT");
                    break;
                }
            };
        }
    }
}
