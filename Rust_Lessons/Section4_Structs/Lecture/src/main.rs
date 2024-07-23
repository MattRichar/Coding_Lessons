
//Named field struct
struct User
{
    active: bool,
    username: String,
    sign_in_count: u32,
}

//Tuple like struct
struct Coordinates(i32,i32,i32);

//Unite like struct
struct UnitStruct;

struct Square
{
    width: u32,
    height:u32,
}

//impl stands for implement, and it is the keyword to define methods on the struct
//In Rust, the "self" keyword is used within the context of method definitions inside an impl block 
//to refer to the instance of the struct or enum on which the method is being called. 
//It is similar to "this" in other object-oriented languages like C++ or Java. 


impl Square
{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn whats_my_width(&self) -> u32{
        self.width
    }

    fn change_width(&mut self, new_width: u32)
    {
        self.width = new_width;
    }
}

struct MyString<'a>
{
    text: &'a str,
}


fn main() {
    //cargo new Lecture --bin --vcs git

    //structs will allow you to encapsulate values into a contained scope, or a single value
    //Rust has three types of structs:
    //1.Named field, gives a name to each component
    //2.Tuple like, identifies by the order they appear
    //3.Unit like, has no components (but is not trivial)

    let user1 = User{active: true, username: String::from("Matt"), sign_in_count: 0};

    println!("{}", user1.username);

    let user2 = build_user(String::from("Richards"));

    println!("{}", user2.username);
    
    let coords = Coordinates(1,2,3);
    // 1..5, .. Range {start: 1, end: 5}

    let sq = Square{width: 5, height: 5};
    println!("{}", sq.area());
    println!("{}", sq.whats_my_width());

    //If we want a mutable struct, we need to declare it as such

    let mut sqMut = Square{width: 5, height: 5};
    sqMut.change_width(10);
    println!("{}", sqMut.whats_my_width());

    //Every reference has a lifetime, most of the time lifetimes are implicit, and inferred.
    //The main idea of lifetimes, is to prevent dangling references
    let r;

    {
        let x=5;
        r = &x;
    }// x is dropped here

    //We try to use r, which is a reference to x, but x is dropped since it is not in scope.
    //Thus, we have a dangling reference.
    //This is where lifetime annotations start to help think through this code, and help the 
    //Rust compiler find these dangling references
    //println!("{}", r); //Gives an error

    let str1 = String::from("This is my string");
    let x = MyString{text:str1.as_str()};

    //When we give a reference a static liftime, the reference can live for the entire duration of the program (think static keyword)

    let s: &'static str = "I have a static lifetime";
}

fn build_user(username: String) -> User
{
    User
    {
        username,
        active: true,
        sign_in_count: 1,
    }
}

//Create a function that, assigns here the lifetime, with a parameter called x, with a specific lifetime, and returns an explicit lifetime
//The histroy of liftimes has evolved as Rust has evolved.
//The compiler uses three rules to infer these liftimes, when there aren't explicit annotations
//1. Each parameter that is a reference, gets its own lifetime parameter.
//2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
//3. If there are multiple input liftime parameters, but one is a reference to self or mut self, the lifetime of self is assigned to all output lifetime parameters
fn example<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
{
    x
    //lifetime of return value, must match the declaration (aka x for 'a)
    //Lifetime annotations describe the relationships of the lifetimes of multiple references, without affecting their liftime.
    // &i32
    // &'a i32
    // &'a mut 32 

}
