use std::io;

fn main() {
    println!("Convert Fahrenheit to Celsius");
    println!("Enter the temperature in Fahrenheit:");
    // init a variable to store our user input
    let temp = get_input();
    let celsius = convert_temp(&temp);
    println!("{} degrees Fahrenheit is equal to {} degrees Celsius.", temp, celsius);
}

fn get_input() -> f64 {
    let mut temp = String::new();
    // read input and store in temp
    io::stdin().read_line(&mut temp)
        .expect("Failed to red line");
    // convert temp from string to f64
    let temp: f64 = match temp.trim().parse() {
        // if temp can be parsed into f64, return that f64
        Ok(num) => num,
        // if temp cannot be parsed into f64, output an error message
        // and recall the convert_temp function
        Err(e) => {
            println!("Please input a number ({})", e);
            get_input()
        }
    };
    temp // return our new f64 value and move ownership back to main
}

fn convert_temp(temp: &f64) -> f64 {
    let c: f64 = (temp - 32_f64) * (5_f64/9_f64);
    c
}
