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


    // let var = 1;
    // //This is a fixed size and is pushed on the stack

    // let mut s = "hello".to_string(); //created on the heap
    // s.push_str(", world");
    //Since we are on the heap, we can add onto this size dynamically

    //A move will move the ownership of one variable to another.
    let x = vec!["tyler".to_string()];
    let y = x;
    //y now owns the value of x
    println!("{:?}",y);
}
//var is dropped outside of this scope.
//s is also dropped