//Rust has two categories for errors, recoverable and unrecoverable errors.

use std::fs::File;
use std::io::ErrorKind;
use std::fs::rename;
use std::io::Error;

fn main() {

    let vec = vec![1];
    //vec[10]; //This doesn't work since there is no 10th element

    //Using the CL, I can set the backtrace parameter for that specific command with "RUST_BACKTRACE=1 cargo run"
    //The backtrace gives us a list of all the function in a recursive order, leading up to the error.

    //This is a macro to indicate an unrecoverable error.
    //panic!("Painicked here!");

    //The result type allows us to prepare, in case there is a failure, without having the program close.
    //Result is an enum that has two variants, "ok" and "err".

    let file = File::open("error.txt");
    let file = match file{ //match the file value based on if a file was succesfuly opened
        Ok(file) => file,
        Err(error) => match error.kind(){//panic!("Error: {:?}", error),
        //If it wasn't created, match the error to what kind it was
            ErrorKind::NotFound => match File::create("error.txt"){
                //If it was a NotFound error, try to create the file
                Ok(file_created) => file_created,
                Err(err) => panic!("Cannot create the file!"),
            }
            _ => panic!("It was some other error kind!"),
            //If it was some other error, panic and end the program.
        }
    };

    //We can propogate the error of the callstack using a ? operator
    let test = open_file();
    test.unwrap();

    rename_file().unwrap();

}

fn open_file() -> Result<File, Error> {
    let file = File::open("error.txt")?;
    //Instead of having to make the exhaustive number of match statements we had earlier,
    //we can use the ? operator to indicate we want this to propogate an error if this doesn't work.
    //The ? operator would not work in the main method, because there is not anything for it to be passed up to.
    Ok(file)
}

fn rename_file() -> Result<(), Error>{//() repreesnts an empty generic
    let file = rename("error.txt", "rename.txt")?;
    Ok(file)
}

//Expected result enumeration
// enum Result<T,E>
// {
//     Ok(T),
//     Err(E),
// }
