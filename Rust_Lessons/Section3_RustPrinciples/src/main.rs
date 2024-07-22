fn main()
{
    //Rust uses an ownership module which is its most unique feature.
    //The compiler will verify if the program is free of memory safety errors like
    //dangling pointers, uninitialized memory, ect...

    //Stack stores memory in the order it gets it (stack on top...)
    //Heap stores memory of unkown size and removes it from the bottom of the stack.

    //Rules of ownership:
    //1. Each value has a variable called its owner
    //2. There can only be one owner at a time
    //3. When the owner goes out of scope, the value will be dropped (also known as freeze)

    let var = 1;
    //This is a fixed size and is pushed on the stack

    let mut s = "hello".to_string(); //created on the heap
    s.push_str(", world");
    //Since we are on the heap, we can add onto this size dynamically

    //A move will move the ownership of one variable to another.
    let x = vec!["tyler".to_string()];
    let y = x;
    //y now owns the value of x
    let z = y;
    //z now owns y
    println!("{:?}",z);

    //A clone will take the value of a variable without taking ownership (which may be expensive)
    let xc = vec!["tyler".to_string()];
    let yc = xc.clone();
    let zc = yc.clone();
    
    println!("xc is {:?}",xc);
    println!("yc is {:?}",yc);
    println!("zc is {:?}",zc);

    //A copy is implemented on types that are already stored on the stack
    //integer, boolean, float, ect...
    let q = 1;
    let r = q;
    println!("q is {}, r is {}", q, r);

    let s = String::from("takes");//Create a variable with a string "takes"
    takes_ownership(s);//Gives ownership to the function

    make_copy(var);//Because this is an i32 value, it will copy the values

    //We called give_ownership() which owns "given"
    let str1: String = give_ownership();
    //We can show that str1 now has ownship of "given"
    println!("{}", str1);

    let str3: String = take_and_give(str1);
    println!("{}", str3);//str1 returns an error since str1 no longer has ownership, but str3 does

    if(true)
    {
        let str4 = str3;
    }
    else
    {
        let str5 = str3;
    }

    //println("{}", str4);

    let mut strLoop = String::from("Tyler");
    let mut strLoop2: String;

    // loop
    // {
    //     //The ownership changes for this value during the entirty of the loop
    //     strLoop2 = strLoop;
    // }

    //References allow us to make a reference to a value without taking ownership
    //aka borrowing the value.

    //A shared reference will let you read, but not modify a value (const)
    let mut share = String::from("Hello");
    //We added the mut key word, making it mutable, so we can change its value inside the function.
    println!("{}", share);
    //By passing by reference, we were able to avoid taking ownership of share.
    //If we don't pass by reference, change_string would have taken ownership of share, then we can't use share furthermore.
    change_string(&mut share);
    println!("{}", share);
}
//var is dropped outside of this scope.
//s is also dropped

fn takes_ownership(s: String)
{
    let string = s;
    println!("{}", string);
}

//of type i32 (32 bit or 4 byte integer)
fn make_copy(one: i32)
{
    let val1 = one;
    println!("{}", val1);
}

fn give_ownership() -> String 
{
    //Notice no semicolon.
    "given".to_string()
}

fn take_and_give(str2: String) -> String
{
    str2
}

fn change_string(some_string: &mut String)
{
    some_string.push_str(", World!");
}