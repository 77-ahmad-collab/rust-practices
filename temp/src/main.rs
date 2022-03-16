use std::io;
fn main() {
  loop{
   println!("Please slect the specific Coversion");
   println!("1. Fahrenheit to Celsius");
   println!("2. Celsius to Fahrenheit");
   println!("3. --- EXIT ---");
   let mut conversion_type = String::new();
   io::stdin()
   .read_line(&mut conversion_type)
   .expect("Failed to read line");
   let conversion_type = conversion_type.trim();
   let conversion_type = match conversion_type{
       "1"=>1,
       "2"=>2,
       "3"=>3,
       _=>{
        println!("Please input 1 or 2!");
       continue;
       }
    };
  
    if conversion_type == 3{
        println!("Thank you for using the temperature converter");
        break;
    }
    println!("the value that has been entered {}", conversion_type);
     println!("Please input the temperature:");
   let mut temperature = String::new();
        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");
let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a valid temperature");
                continue;
            }
        };
    let converted_temperature = if conversion_type == 1 {
        fahrenheit_to_celsius(temperature)  }
        else {
            celsius_to_fahrenheit(temperature)
        };
     println!("The converted temperature is {}", converted_temperature);
     continue;
  }
 
}
fn fahrenheit_to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) * (5.0 / 9.0)
}
fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    temperature * (9.0 / 5.0) + 32.0
}
// . This is known as a double free error and is one of the memory safety bugs we mentioned previously.
//  Freeing memory twice can lead to memory corruption, which can potentially lead to security
//   vulnerabilities