struct Car
{
    mpg: u32,
    color: String,
    top_speed: u32,
}

impl Car
{
    fn set_mpg(&mut self, new_mpg: u32)
    {
        self.mpg = new_mpg;
    }

    fn set_color(&mut self, new_color: String)
    {
        self.color = new_color;
    }

    fn set_top_speed(&mut self, new_top_speed: u32)
    {
        self.top_speed = new_top_speed;
    }
}

fn main() {

    let mut Mercedes = Car{mpg: 12, color: String::from("Pink"), top_speed: 200};

    println!("The Mercedes has a mile per gallon of {}, it comes in the color of {}, and its top speed is {}", Mercedes.mpg, Mercedes.color, Mercedes.top_speed);

    Mercedes.set_mpg(15);
    Mercedes.set_color("Orange".to_string());
    Mercedes.set_top_speed(300);

    println!("The new Mercedes has a mile per gallon of {}, it comes in the color of {}, and its top speed is {}", Mercedes.mpg, Mercedes.color, Mercedes.top_speed);

}
