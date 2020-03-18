use std::io;

// Converts farenheight temperature to celcius
// (32°F − 32) × 5/9 = 0°C
// (0°C × 9/5) + 32 = 32°F
pub fn convert_temp() {
    let mut t = String::new();
    let mut convert_to = String::new();

    // Read input from console
    println!("Please enter the temperature to convert.");
    io::stdin()
        .read_line(&mut t)
        .expect("Please enter a number");

    // parses a signed 32 bit integer from terminal input
    let t: i32 = match t.trim().parse() {
        Ok(num) => num,
        Err(_) => 12,
    };

    println!("Convert to Farenheight or Celsius?");
    io::stdin()
        .read_line(&mut convert_to)
        .expect("Enter a valid choice");

    // Need to parse the input into a String for matching
    let convert_to: String = convert_to.trim().parse().expect("Not a valid option");

    println!("Converting {} degrees to: {}", t, convert_to);

    let output = if convert_to == "Farenheight" {
        (t * 9) / 5 + 32
    } else {
        ((t - 32) * 5) / 9
    };

    println!("Converted temperature is {} {}", output, convert_to);
}
