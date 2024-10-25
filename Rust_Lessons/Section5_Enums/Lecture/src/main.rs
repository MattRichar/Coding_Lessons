//Enumerations allow you to define a type, by enumerating its possible variants
//Enums are important, because it allows variables to have value, or be nothing
//Rust doesn't have a Null value, it is just an empty value of an enum
enum Pet {dog, cat, fish}

//impl is the keyword for a method in rust.
impl Pet
{
    fn what_am_i(self) -> &'static str 
    {

        //match is equivalent to a switch in C...
        match self
        {
            Pet::dog => "I am a dog",
            Pet::cat => "I am a cat",
            Pet::fish => "I am a fish",
        }
    }
}

enum IpAddrKind1
{
    V4,
    V6,
}

enum IpAddrKind2
{
    V4(String),
    V6,
}

struct IpAddr
{
    kind: IpAddrKind1,
    address: String,
}

fn main() 
{
    let Dog = Pet::dog;
    println!("{}", Dog.what_am_i());

    let home1 = IpAddr{
        kind: IpAddrKind1::V4,
        address: String::from("127.0.0.1"),
    };
    //Here we are letting home be a struct of type IpAddr
    //We are enumerating its kind as an IpAddrKind, and speciftying it to V4.
    //We enter in the string of the ip address

    let home2 = IpAddrKind2::V4(String::from("127.0.0.1"));

    //These two declarations for home are equivalent
    //Notice how you can contain the string of the ip address as a parameter of another enmeration.

    println!("{:?}", home1.address);
    //home1's address can be printed out directly because it is a field in the IpAddr struct
    match home2 {
        IpAddrKind2::V4(addr) => println!("{:?}", addr),
        IpAddrKind2::V6 => println!("No address for V6"),
    }
    //For home2, the IpAddrKind2 enum variant V4 holds the IP address inside a String,
    //so we use a match expression to extract and print it. 
    //If the variant were V6, we handle it with a message.

    let loopack = IpAddr{
        kind: IpAddrKind1::V6,
        address: String::from("::1"),
    };

    let some_number = Some(5);
    let some_string = Some("a string");
    let nothing: Option<i32> = None;//Option<T>

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    //let sum = x + y;
    //This doesn't work since we do not provide the correct option for it.

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}, {:?}, {:?}", five, six, none);

    what_pet("Dog");
    what_pet("Cow");

    //The extension of the Some keyword can work the same with conditional statements.
    let dog2 = Some(Pet::cat);
    if let Some(Pet::dog) = dog2
    {
        println!("The animal is a dog!");
    }
    else
    {
        println!("Not a dog!");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    //Here, "top" is being give the value of the popped value from the previous declared "stack" vector
    while let Some(top) = stack.pop()
    {
        println!("{}", top);
    }

    let x = 1;

    match x 
    {
        1 | 2 => println!("One or two!"),
        3..=5 => println!("From three to five!"),
        _ => println!("Doesn't match ant value"),
    }

    let x = Some(5);
    let y = 5;

    match x
    {
        Some(10) => println!("Ten!"),
        Some(x) if x == y => println!("Matches"),
        _ => println!("Default!"),
    }

}

//This is the Option keyword, allows one to set conditions in which a value has "Some" value, or if it has no value, aka "None"
// enum Option<T>
// {
//     None,
//     Some(T),
// }

fn plus_one(x: Option<i32>) -> Option<i32>
{
    match x
    {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn what_pet(input: &str)
{
    match input
    {
        "Dog" => println!("I have a dog!"),
        "cat" => println!("I have a cat!"),
        "fish" => println!("I have a fish!"),
        _ => println!("I have no clue what pet you have :("),
        //The underscore works as the default keyword
    }
}
