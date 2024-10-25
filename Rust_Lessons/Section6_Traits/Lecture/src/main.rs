//Generics allow us to have standin types for concrete types
//Traits will allow us to represent a capability, that can be implemented in, on many different types

//Operator overloading is how we can make our own types, support arithmetic.
use std::ops::Add;

#[derive(Debug)]
//We need to derive in the Debug fromat
struct Point<T>
{
    x: T,   
    y: T,   
}

impl<T> Add for Point<T>
    where 
    //"where" allows specifying constraints on lifetime and generic parameters.  
    //In this case, where restricts T, to types that can be added to themselves, yielding another T value
    T: Add<Output = T>
    {
        type Output = Self;
        fn add(self, rhs: Self) -> Self{
            Point{
                x:self.x + rhs.x,
                y:self.y + rhs.y,
            }
        }
    }

struct DifPoint<T,U>
{
    x: T,
    y: U,
}

//We can use traits to define shared behavious in an abstract way
trait Overview
{
    fn overview(&self) -> String
    {
        String::from("This is a Rust Course!")
    }
}

struct Course 
{
    headline: String,
    author: String,
}

struct AnotherCourse
{
    headline:String,
    author: String,
}

impl Overview for Course
{
    fn overview(&self) -> String
        {
        // Check if author or headline is empty, return default if true
        if self.author.is_empty() || self.headline.is_empty() {
            String::from("This is a Rust Course!")
        } else {
            format!("{}, {}", self.author, self.headline)
        }
    }
}

impl Overview for AnotherCourse
{
    fn overview(&self) -> String
    {
        format!("{}, {}", self.author, self.headline)
    }
}

impl Default for Course {
    fn default() -> Self {
        Course {
            headline: String::new(),
            author: String::new(),
        }
    }
}

//Similar to a destructor
impl Drop for Course {

    fn drop(&mut self)
    {
        println!("Dropping: {}", self.author)
    }
}

//Clone is for types that can make copies of themselves.
trait Clone:Sized{
    //Clone is a trait and Sized is a "trait bound" meaning that only types whos size is known at compile time can implement clone
    fn clone(&self) -> Self;
    //&self means this method takes a reference to the value (without taking ownership).
    //Self means the method returns an instance of the same type (Self refers to the type implementing the Clone trait).
    fn clone_from(&mut self, source: &Self)
    {
        *self = source.clone()
    }
}

//Copy is the same as clone, except it only works on types that are stored as bytes.
//When you make a typedef a copy, it has this restriction associated with it.

//From and Into traits allow us to perform conversions on a value of one type, and then return it as another.
//fn into(self) -> T
//fn from(T) -> self

//TryFrom and TryInto provides leaniancy when dealing with different byte sizes for the conversion.
//Think going from i32 to i64 or double to long double...
//fn try_from(value: T) -> Result<Self, Self::Error>

fn main() {

//Generics where used in the last section when we discussed options

    let coord = Point{x:5.0, y:5.0};
    let coord2add = Point{x:1.0, y:2.0};
    let coord2 = Point{x: 'x', y: 'y'};

    let sum = coord + coord2add;

    println!("{:?}", sum);

    let dif_coord = DifPoint{x: 'x', y:5.0};

    let course1 = Course{headline: String::from("Headline!"), author: String::from("Matt")};
    let course2 = AnotherCourse{headline: String::from("Another Headline!"), author: String::from("Another Matt")};
    //In order to have a default trait implemented into Course, we needed to explicitly built its implementation into Course
    let course3 = Course::default();

    println!("{}", course3.overview());
    println!("{}", course1.overview());
    println!("{}", course2.overview());

    //We can pass traits as parameters as well..
    call_overview(&course1);
    call_overview(&course2);

    

    //Dropping is freeing the resources that the value is using.
    //This happens in various cases, e.g. variable out of scope, removing elements form vector, ect...
    //This is usually handled automatically by Rust, but there is a specific implementation for this

    //drop(course1);
}//course1 and course3 were automatically dropped here, but since we wrote a drop implementation to write to the command line,
//it will print out the dropping string for both courses.

//Lets create a function that will call the overview method from its item parameter
//We use T as a generic, and call the variable T, for the impl Overview
fn call_overview<T: Overview>(item: &T)
{
    println!("Overview: {}", item.overview())
}

//This is similar in concept to a typedef in C++
// fn overview(item1: &impl Overview, item2: &impl Overview)
// fn overview<T: Overview>(item1: &T, item2: &T)
//These two are equivalent

// fn overview(item1: &impl Overview + AnotherTrait)
// fn overview<T: Overview + AnotherTrait>(item1: &T, item2: &T)
