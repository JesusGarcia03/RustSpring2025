//create constant freezing point variable
const Freezing_Fahrenheit: f64 = 32.0;

//Function that will convert from farenheit to celsius
fn fahrenheit_to_celsius(f: f64) -> f64{
    (f- 32.0) * 5.0/9.0
}
//Function that will convert from celsius to farenheit
fn celsius_to_fahrenheit(c: f64) -> f64{
    (c * 9.0/5.0) + 32.0
}

fn main() {
    //create variable that hold farhrenheit temperature
   let mut Temp_fahr = 32.0;
   //variable that holds the converted celsius value
   let Temp_cels = fahrenheit_to_celsius(Temp_fahr);
   println!("{}°F is {:.2}°C", Temp_fahr,Temp_cels);
//loop that loops the value of fahrenheit and increments
//it by one then prints that new value in celsius and fahrenheit
   for i in 1..=5{
    Temp_fahr += 1.0;
    let Temp_cels = fahrenheit_to_celsius(Temp_fahr);
    println!("{}°F is {:.2}°C", Temp_fahr,Temp_cels);
   }
}
