enum Insurance{
    Car(String),
    House(u16),
}
impl Insurance{
    fn show_info(&self){
        match self{
            Insurance::Car(model) => println!("my car model is {:?}",model),
            Insurance::House(year) => println!("My house was built {:?}",year)
        }
    }
}
fn main(){
    let car = Insurance::Car("BMW".to_string());
    let house = "2004";
    car.show_info();
    house.show_info();
}