//Question 1 of the assignment
macro_rules! define_structs {
    ($name:ident) => {
        struct $name {
            mpg: u32,
            color: String,
            top_speed: u32,
        }
    };
}

define_structs!(Car);
define_structs!(Motorcycle);

//This macro does effectively the same thing as below, but now I can define multiple structs with the same fields easily
// struct Car
// {
//     mpg: u32,
//     color: String,
//     top_speed: u32,
// }

// struct Motorcycle
// {
//     mpg: u32,
//     color: String,
//     top_speed: u32,
// }

//Define the features I want in my trait
trait Features
{
    fn set_mpg(&mut self, new_mpg: u32);
    fn set_color(&mut self, new_color: String);
    fn set_top_speed(&mut self, new_top_speed: u32);
}

// Define a macro to implement the Features trait for both Cars and Motorcycles simply
macro_rules! impl_features {
    ($struct_name:ident) => {
        impl Features for $struct_name {
            fn set_mpg(&mut self, new_mpg: u32) {
                self.mpg = new_mpg;
            }

            fn set_color(&mut self, new_color: String) {
                self.color = new_color;
            }

            fn set_top_speed(&mut self, new_top_speed: u32) {
                self.top_speed = new_top_speed;
            }
        }
    };
}

//By doing this, I can reduce the need to rewrite the traits specifically for Car and Motorcycle structs
impl_features!(Car);
impl_features!(Motorcycle);

//Question 2 of the assignment
fn print<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}

fn main() {

    let mut Mercedes = Car{mpg: 12, color: String::from("Pink"), top_speed: 200};
    let mut Ducati = Motorcycle{mpg: 20, color: String::from("Red"), top_speed: 300};

    println!("The Mercedes has a mile per gallon of {}, it comes in the color of {}, and its top speed is {}", Mercedes.mpg, Mercedes.color, Mercedes.top_speed);
    println!("The Ducati has a mile per gallon of {}, it comes in the color of {}, and its top speed is {}", Ducati.mpg, Ducati.color, Ducati.top_speed);

    Mercedes.set_mpg(15);
    Mercedes.set_color("Orange".to_string());
    Mercedes.set_top_speed(300);
    Ducati.set_mpg(25);
    Ducati.set_color("Green".to_string());
    Ducati.set_top_speed(275);

    println!("The new Mercedes has a mile per gallon of {}, it comes in the color of {}, and its top speed is {}", Mercedes.mpg, Mercedes.color, Mercedes.top_speed);
    println!("The new Ducati has a mile per gallon of {}, it comes in the color of {}, and its top speed is {}", Ducati.mpg, Ducati.color, Ducati.top_speed);

    print(42);
    print("Yo Momma");
}